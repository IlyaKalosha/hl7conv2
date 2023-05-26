use crate::utils;
use pyo3::prelude::*;
use pyo3::types::PyType;
use pyo3::PyResult;
use std::collections::BTreeMap;
use std::fs;
use std::str::Split;

#[pyclass]
pub struct Hl7Json {
    #[pyo3(get)]
    hl7_string: String,
}

#[pymethods]
impl Hl7Json {
    #[new]
    fn new(hl7_string: String) -> Self {
        Hl7Json {
            hl7_string: utils::replace_eof(hl7_string),
        }
    }

    #[classmethod]
    fn from_file(cls: &PyType, path: String) -> Self {
        let contents = fs::read_to_string(path).expect("Should have been able to read the file");
        Hl7Json {
            hl7_string: utils::replace_eof(contents),
        }
    }

    fn _split_hl7_parent_field_to_json(
        &self,
        parent_key: usize,
        parent_value: &str,
    ) -> BTreeMap<String, String> {
        let list_of_sub_fields: Split<&str> = parent_value.split("^");
        let mut children_json: BTreeMap<String, String> = BTreeMap::new();
        for (key, value) in list_of_sub_fields.enumerate() {
            children_json.insert(format!("{}.{}", parent_key, key + 1), value.to_string());
        }
        children_json
    }

    fn _split_hl7_seg_to_json(&self, seg: String) -> BTreeMap<String, String> {
        let list_of_fields: Split<&str> = seg.split("|");
        let mut parents_json: BTreeMap<String, String> = BTreeMap::new();
        for (key, parent_value) in list_of_fields.enumerate() {
            let mut str_key = key.to_string();
            if key == 0 {
                str_key = "segment_name".to_string();
            }
            if parent_value.contains('^') && !parent_value.contains("^~\\&") {
                let children_json: BTreeMap<String, String> =
                    self._split_hl7_parent_field_to_json(key, parent_value);
                parents_json.extend(children_json)
            } else {
                parents_json.insert(str_key, parent_value.to_string());
            }
        }
        parents_json
    }

    fn _convert_hl7_to_json(&self) -> Vec<BTreeMap<String, String>> {
        let mut message_json: Vec<BTreeMap<String, String>> = Vec::new();
        for seg in utils::split_segments(self.hl7_string.to_string()) {
            let seg_json = self._split_hl7_seg_to_json(seg);
            message_json.push(seg_json);
        }
        message_json
    }

    #[getter]
    fn hl7_json(&self) -> PyResult<Vec<BTreeMap<String, String>>> {
        Ok(self._convert_hl7_to_json())
    }
}
