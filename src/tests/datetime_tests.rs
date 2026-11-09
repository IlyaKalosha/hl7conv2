use crate::datetime::{parse_hl7_datetime, Hl7DateTime};
use pyo3::prelude::*;

fn dt(
    (year, month, day): (u16, u8, u8),
    (hour, minute, second): (u8, u8, u8),
    microsecond: u32,
    offset_minutes: Option<i32>,
) -> Hl7DateTime {
    Hl7DateTime {
        year,
        month,
        day,
        hour,
        minute,
        second,
        microsecond,
        offset_minutes,
    }
}

#[test]
fn parses_minimal_value() {
    assert_eq!(
        parse_hl7_datetime("202401311530").unwrap(),
        Some(dt((2024, 1, 31), (15, 30, 0), 0, None))
    );
}

#[test]
fn parses_seconds() {
    assert_eq!(
        parse_hl7_datetime("20240131153045").unwrap(),
        Some(dt((2024, 1, 31), (15, 30, 45), 0, None))
    );
}

#[test]
fn parses_fractional_seconds_of_any_length() {
    let cases = [
        ("20240131153045.1", 100_000),
        ("20240131153045.12", 120_000),
        ("20240131153045.123", 123_000),
        ("20240131153045.1234", 123_400),
    ];
    for (value, micros) in cases {
        let parsed = parse_hl7_datetime(value).unwrap().unwrap();
        assert_eq!(parsed.microsecond, micros, "{value}");
    }
}

#[test]
fn parses_offsets() {
    assert_eq!(
        parse_hl7_datetime("20240131153045.5+0530").unwrap(),
        Some(dt((2024, 1, 31), (15, 30, 45), 500_000, Some(330)))
    );
    assert_eq!(
        parse_hl7_datetime("202401311530-0800")
            .unwrap()
            .unwrap()
            .offset_minutes,
        Some(-480)
    );
    assert_eq!(
        parse_hl7_datetime("202401311530+0000")
            .unwrap()
            .unwrap()
            .offset_minutes,
        Some(0)
    );
}

#[test]
fn null_year_returns_none() {
    assert_eq!(parse_hl7_datetime("0000").unwrap(), None);
    assert_eq!(parse_hl7_datetime("0000+0100").unwrap(), None);
}

#[test]
fn accepts_leap_day_only_in_leap_years() {
    assert!(parse_hl7_datetime("202402291200").unwrap().is_some());
    assert!(parse_hl7_datetime("200002291200").unwrap().is_some());
    assert!(parse_hl7_datetime("202302291200").is_err());
    assert!(parse_hl7_datetime("190002291200").is_err());
}

#[test]
fn rejects_invalid_values() {
    let invalid = [
        "",
        "202",
        "2024",                 // year other than 0000 needs month..minute
        "20240131",             // missing time
        "2024013115",           // missing minutes
        "2024013115301",        // odd length seconds
        "202401311530.5",       // fraction without seconds
        "20240131153045.",      // empty fraction
        "20240131153045.12345", // too many fraction digits
        "202413011200",         // month 13
        "202400011200",         // month 0
        "202404311200",         // April 31
        "202401312400",         // hour 24
        "202401311260",         // minute 60
        "20240131123060",       // second 60
        "202401311530+530",     // short offset
        "202401311530+2400",    // offset hours out of range
        "202401311530+0060",    // offset minutes out of range
        "2024-01-31T15:30",     // ISO format
        "2024013115３0",        // non-ASCII digit
        "20240131153045+0100Z", // trailing garbage
    ];
    for value in invalid {
        assert!(parse_hl7_datetime(value).is_err(), "{value:?} should fail");
    }
}

#[test]
fn python_function_returns_datetime() {
    Python::initialize();
    Python::attach(|py| {
        let module = pyo3::wrap_pymodule!(crate::hl7conv2)(py);
        let parse = module.getattr(py, "parse_hl7_datetime").unwrap();

        let naive = parse.call1(py, ("20240131153045.25",)).unwrap();
        let iso: String = naive
            .call_method0(py, "isoformat")
            .unwrap()
            .extract(py)
            .unwrap();
        assert_eq!(iso, "2024-01-31T15:30:45.250000");

        let aware = parse.call1(py, ("202401311530-0230",)).unwrap();
        let iso: String = aware
            .call_method0(py, "isoformat")
            .unwrap()
            .extract(py)
            .unwrap();
        assert_eq!(iso, "2024-01-31T15:30:00-02:30");

        let null = parse.call1(py, ("0000",)).unwrap();
        assert!(null.is_none(py));

        let err = parse.call1(py, ("not a date",)).unwrap_err();
        assert!(err.is_instance_of::<pyo3::exceptions::PyValueError>(py));
    });
}
