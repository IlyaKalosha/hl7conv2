use crate::errors::Hl7Error;
use crate::escape::{create_default_escape_handler, Hl7EscapeHandler};
use crate::segments;
use crate::utils;
use crate::validation::Hl7Validator;
use pyo3::prelude::*;
use pyo3::types::PyType;
use pyo3::PyResult;
use std::collections::BTreeMap;
use std::fs;

#[pyclass]
pub struct Hl7Json {
    #[pyo3(get)]
    pub hl7_string: String,
    #[pyo3(get, set)]
    pub validation_enabled: bool,
    #[pyo3(get, set)]
    pub strict_validation: bool,
    #[pyo3(get, set)]
    pub escaping_enabled: bool,
    pub escape_handler: Hl7EscapeHandler,
}

#[pymethods]
impl Hl7Json {
    #[new]
    pub fn new(
        hl7_string: String,
        validation_enabled: Option<bool>,
        strict_validation: Option<bool>,
        escaping_enabled: Option<bool>,
    ) -> Self {
        Hl7Json {
            hl7_string: utils::replace_eof(hl7_string),
            validation_enabled: validation_enabled.unwrap_or(false),
            strict_validation: strict_validation.unwrap_or(true),
            escaping_enabled: escaping_enabled.unwrap_or(false),
            escape_handler: create_default_escape_handler(),
        }
    }

    #[classmethod]
    fn from_file(
        _cls: &Bound<PyType>,
        path: String,
        validation_enabled: Option<bool>,
        strict_validation: Option<bool>,
        escaping_enabled: Option<bool>,
    ) -> PyResult<Self> {
        let contents = fs::read_to_string(&path).map_err(Hl7Error::IoError)?;

        if contents.trim().is_empty() {
            return Err(Hl7Error::EmptyMessage.into());
        }

        Ok(Hl7Json {
            hl7_string: utils::replace_eof(contents),
            validation_enabled: validation_enabled.unwrap_or(false),
            strict_validation: strict_validation.unwrap_or(true),
            escaping_enabled: escaping_enabled.unwrap_or(false),
            escape_handler: create_default_escape_handler(),
        })
    }

    pub fn _split_hl7_seg_to_json(&self, seg: &str) -> BTreeMap<String, String> {
        let segment = segments::Hl7Segment::from_string(seg, None);
        segment.to_json()
    }

    pub fn _split_hl7_seg_to_json_with_escaping(&self, seg: &str) -> BTreeMap<String, String> {
        let segment = segments::Hl7Segment::from_string(seg, Some(&self.escape_handler));
        segment.to_json()
    }

    pub fn _convert_hl7_to_json(&self) -> PyResult<Vec<BTreeMap<String, String>>> {
        if self.validation_enabled {
            self.validate(Some(self.strict_validation), Some(true))?;
        }

        let mut message_json: Vec<BTreeMap<String, String>> = Vec::new();
        for seg in utils::split_segments(self.hl7_string.clone()) {
            let seg_json = if self.escaping_enabled {
                self._split_hl7_seg_to_json_with_escaping(&seg)
            } else {
                self._split_hl7_seg_to_json(&seg)
            };
            message_json.push(seg_json);
        }
        Ok(message_json)
    }

    #[getter]
    fn hl7_json(&self) -> PyResult<Vec<BTreeMap<String, String>>> {
        self._convert_hl7_to_json()
    }

    pub fn validate(
        &self,
        strict_mode: Option<bool>,
        validate_required_fields: Option<bool>,
    ) -> PyResult<()> {
        let segments: Vec<segments::Hl7Segment> = utils::split_segments(self.hl7_string.clone())
            .iter()
            .map(|seg| segments::Hl7Segment::from_string(seg, None))
            .collect();

        let use_strict_mode = strict_mode.unwrap_or(self.strict_validation);
        let use_required_fields_validation = validate_required_fields.unwrap_or(true);

        let validator = Hl7Validator::new()
            .with_strict_mode(use_strict_mode)
            .with_required_fields_validation(use_required_fields_validation);

        validator.validate_message(&segments).map_err(|e| e.into())
    }
}
