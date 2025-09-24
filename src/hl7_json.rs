use crate::errors::Hl7Error;
use crate::segments;
use crate::utils;
use pyo3::prelude::*;
use pyo3::types::PyType;
use pyo3::PyResult;
use std::collections::BTreeMap;
use std::fs;

#[pyclass]
pub struct Hl7Json {
    #[pyo3(get)]
    pub hl7_string: String,
}

#[pymethods]
impl Hl7Json {
    #[new]
    pub fn new(hl7_string: String) -> Self {
        Hl7Json {
            hl7_string: utils::replace_eof(hl7_string),
        }
    }

    #[classmethod]
    fn from_file(_cls: &Bound<PyType>, path: String) -> PyResult<Self> {
        let contents = fs::read_to_string(&path).map_err(Hl7Error::IoError)?;

        if contents.trim().is_empty() {
            return Err(Hl7Error::EmptyMessage.into());
        }

        Ok(Hl7Json {
            hl7_string: utils::replace_eof(contents),
        })
    }

    pub fn _split_hl7_seg_to_json(&self, seg: &str) -> BTreeMap<String, String> {
        let segment = segments::Hl7Segment::from_string(seg);
        segment.to_json()
    }

    pub fn _convert_hl7_to_json(&self) -> Vec<BTreeMap<String, String>> {
        let mut message_json: Vec<BTreeMap<String, String>> = Vec::new();
        for seg in utils::split_segments(self.hl7_string.clone()) {
            let seg_json = self._split_hl7_seg_to_json(&seg);
            message_json.push(seg_json);
        }
        message_json
    }

    #[getter]
    fn hl7_json(&self) -> PyResult<Vec<BTreeMap<String, String>>> {
        Ok(self._convert_hl7_to_json())
    }
}
