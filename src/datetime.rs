//! Parsing of HL7 date/time values in the form
//! `YYYYMMDDHHMM[SS[.S[S[S[S]]]]][+/-ZZZZ]`.
//!
//! * `YYYY` is always required.
//! * `MM`, `DD`, `HH` and `MM` (minutes) are required unless the year is `0000`.
//! * Seconds and up to four fractional-second digits are optional.
//! * The UTC offset (`+HHMM` / `-HHMM`) is optional.
//!
//! A year of `0000` denotes an empty (null) date and parses to `None`.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

const MAX_FRACTION_DIGITS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hl7DateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub microsecond: u32,
    /// UTC offset in minutes, if the value carried one.
    pub offset_minutes: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateTimeParseError(pub String);

impl std::fmt::Display for DateTimeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid HL7 date/time: {}", self.0)
    }
}

impl std::error::Error for DateTimeParseError {}

fn err<T>(value: &str, reason: &str) -> Result<T, DateTimeParseError> {
    Err(DateTimeParseError(format!("{value:?} ({reason})")))
}

fn is_leap_year(year: u16) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        2 if is_leap_year(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// Reads a run of ASCII digits as a number. The caller guarantees `digits` is all-digit.
fn number(digits: &str) -> u32 {
    digits
        .bytes()
        .fold(0, |acc, b| acc * 10 + u32::from(b - b'0'))
}

fn all_digits(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())
}

/// Splits a trailing `+ZZZZ` / `-ZZZZ` offset off the value.
fn split_offset<'a>(
    original: &str,
    value: &'a str,
) -> Result<(&'a str, Option<i32>), DateTimeParseError> {
    let Some(sign_pos) = value.rfind(['+', '-']) else {
        return Ok((value, None));
    };
    let (body, offset) = value.split_at(sign_pos);
    let digits = &offset[1..];
    if digits.len() != 4 || !all_digits(digits) {
        return err(original, "UTC offset must be +ZZZZ or -ZZZZ");
    }
    let hours = number(&digits[..2]);
    let minutes = number(&digits[2..]);
    if hours > 23 || minutes > 59 {
        return err(original, "UTC offset out of range");
    }
    let total = (hours * 60 + minutes) as i32;
    let sign = if offset.starts_with('-') { -1 } else { 1 };
    Ok((body, Some(sign * total)))
}

/// Parses an HL7 date/time string.
///
/// Returns `Ok(None)` for a null date (year `0000`).
pub fn parse_hl7_datetime(value: &str) -> Result<Option<Hl7DateTime>, DateTimeParseError> {
    let (body, offset_minutes) = split_offset(value, value)?;

    let (whole, fraction) = match body.split_once('.') {
        Some((whole, fraction)) => (whole, Some(fraction)),
        None => (body, None),
    };

    if whole.len() < 4 || !all_digits(whole) {
        return err(value, "expected at least a 4-digit year");
    }
    if let Some(fraction) = fraction {
        if fraction.len() > MAX_FRACTION_DIGITS || !all_digits(fraction) {
            return err(value, "fractional seconds must be 1 to 4 digits");
        }
    }

    let year = number(&whole[..4]) as u16;
    if year == 0 {
        return Ok(None);
    }

    match whole.len() {
        12 if fraction.is_none() => {}
        12 => return err(value, "fractional seconds require seconds"),
        14 => {}
        _ => return err(value, "expected YYYYMMDDHHMM[SS]"),
    }

    let month = number(&whole[4..6]) as u8;
    let day = number(&whole[6..8]) as u8;
    let hour = number(&whole[8..10]) as u8;
    let minute = number(&whole[10..12]) as u8;
    let second = if whole.len() == 14 {
        number(&whole[12..14]) as u8
    } else {
        0
    };

    if !(1..=12).contains(&month) {
        return err(value, "month out of range");
    }
    if day == 0 || day > days_in_month(year, month) {
        return err(value, "day out of range");
    }
    if hour > 23 {
        return err(value, "hour out of range");
    }
    if minute > 59 {
        return err(value, "minute out of range");
    }
    if second > 59 {
        return err(value, "second out of range");
    }

    // Scale the fraction to microseconds: ".5" -> 500000, ".1234" -> 123400.
    let microsecond = fraction.map_or(0, |f| number(f) * 10u32.pow((6 - f.len()) as u32));

    Ok(Some(Hl7DateTime {
        year,
        month,
        day,
        hour,
        minute,
        second,
        microsecond,
        offset_minutes,
    }))
}

impl Hl7DateTime {
    /// Builds a Python `datetime.datetime`, timezone-aware when an offset is present.
    pub fn to_python<'py>(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let datetime = py.import("datetime")?;
        let kwargs = PyDict::new(py);
        if let Some(offset) = self.offset_minutes {
            let delta_kwargs = PyDict::new(py);
            delta_kwargs.set_item("minutes", offset)?;
            let delta = datetime
                .getattr("timedelta")?
                .call((), Some(&delta_kwargs))?;
            let tz = datetime.getattr("timezone")?.call1((delta,))?;
            kwargs.set_item("tzinfo", tz)?;
        }
        datetime.getattr("datetime")?.call(
            (
                self.year,
                self.month,
                self.day,
                self.hour,
                self.minute,
                self.second,
                self.microsecond,
            ),
            Some(&kwargs),
        )
    }
}

/// Parses an HL7 date/time string (`YYYYMMDDHHMM[SS[.S[S[S[S]]]]][+/-ZZZZ]`)
/// into a `datetime.datetime`. Returns `None` for a null date (year `0000`).
/// Raises `ValueError` if the value does not match the format.
#[pyfunction(name = "parse_hl7_datetime")]
pub fn py_parse_hl7_datetime<'py>(
    py: Python<'py>,
    value: &str,
) -> PyResult<Option<Bound<'py, PyAny>>> {
    match parse_hl7_datetime(value) {
        Ok(Some(dt)) => dt.to_python(py).map(Some),
        Ok(None) => Ok(None),
        Err(e) => Err(PyValueError::new_err(e.to_string())),
    }
}
