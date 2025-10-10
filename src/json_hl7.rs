use crate::errors::Hl7Error;
use crate::escape::{create_default_escape_handler, Hl7EscapeHandler};
use pyo3::prelude::*;
use pyo3::types::PyType;
use pyo3::PyResult;
use std::collections::BTreeMap;

#[pyclass]
pub struct JsonHl7 {
    #[pyo3(get)]
    pub json_data: Vec<BTreeMap<String, String>>,
    pub escape_handler: Hl7EscapeHandler,
}

#[pymethods]
impl JsonHl7 {
    #[new]
    pub fn new(json_data: Vec<BTreeMap<String, String>>) -> Self {
        JsonHl7 {
            json_data,
            escape_handler: create_default_escape_handler(),
        }
    }

    #[classmethod]
    pub fn from_file(_cls: &Bound<PyType>, path: String) -> PyResult<Self> {
        let contents = std::fs::read_to_string(&path).map_err(Hl7Error::IoError)?;

        if contents.trim().is_empty() {
            return Err(Hl7Error::EmptyMessage.into());
        }

        let json_data: Vec<BTreeMap<String, String>> = serde_json::from_str(&contents)
            .map_err(|e| Hl7Error::InvalidFormat(format!("Invalid JSON: {}", e)))?;

        Ok(JsonHl7 {
            json_data,
            escape_handler: create_default_escape_handler(),
        })
    }

    #[getter]
    fn hl7_string(&self) -> PyResult<String> {
        Ok(self._convert_json_to_hl7())
    }

    pub fn _convert_json_to_hl7(&self) -> String {
        let mut hl7_segments = Vec::new();

        for segment_json in &self.json_data {
            let segment_string = self._convert_segment_json_to_hl7(segment_json);
            hl7_segments.push(segment_string);
        }

        hl7_segments.join("\n")
    }
}

impl JsonHl7 {
    pub fn _convert_segment_json_to_hl7(&self, segment_json: &BTreeMap<String, String>) -> String {
        let mut fields = Vec::new();

        if let Some(segment_name) = segment_json.get("segment_name") {
            fields.push(segment_name.clone());
        }

        let mut field_map: BTreeMap<usize, BTreeMap<usize, BTreeMap<usize, String>>> =
            BTreeMap::new();

        for (key, value) in segment_json {
            if key == "segment_name" {
                continue;
            }

            if let Ok(field_index) = key.parse::<usize>() {
                let escaped_value = self.escape_handler.escape(value);
                field_map
                    .entry(field_index)
                    .or_default()
                    .entry(0)
                    .or_default()
                    .insert(0, escaped_value);
            } else if key.contains('[') && key.contains(']') {
                if let Some(bracket_start) = key.find('[') {
                    if let Some(bracket_end) = key.find(']') {
                        let field_part = &key[..bracket_start];
                        let rep_part = &key[bracket_start + 1..bracket_end];
                        let component_part = &key[bracket_end + 1..];

                        if let (Ok(field_index), Ok(rep_index)) =
                            (field_part.parse::<usize>(), rep_part.parse::<usize>())
                        {
                            if let Some(component_str) = component_part.strip_prefix('.') {
                                if let Ok(component_index) = component_str.parse::<usize>() {
                                    let escaped_value = self.escape_handler.escape(value);
                                    field_map
                                        .entry(field_index)
                                        .or_default()
                                        .entry(rep_index)
                                        .or_default()
                                        .insert(component_index, escaped_value);
                                }
                            } else {
                                let escaped_value = self.escape_handler.escape(value);
                                field_map
                                    .entry(field_index)
                                    .or_default()
                                    .entry(rep_index)
                                    .or_default()
                                    .insert(0, escaped_value);
                            }
                        }
                    }
                }
            } else if key.contains('.') {
                let parts: Vec<&str> = key.split('.').collect();
                if parts.len() == 2 {
                    if let (Ok(parent_index), Ok(component_index)) =
                        (parts[0].parse::<usize>(), parts[1].parse::<usize>())
                    {
                        let escaped_value = self.escape_handler.escape(value);
                        field_map
                            .entry(parent_index)
                            .or_default()
                            .entry(0)
                            .or_default()
                            .insert(component_index, escaped_value);
                    }
                }
            }
        }

        let max_index = field_map.keys().max().copied().unwrap_or(0);
        for i in 1..=max_index {
            if let Some(repetitions) = field_map.get(&i) {
                let mut rep_strings = Vec::new();
                let max_rep = repetitions.keys().max().copied().unwrap_or(0);

                for rep_index in 0..=max_rep {
                    if let Some(components) = repetitions.get(&rep_index) {
                        let max_comp = components.keys().max().copied().unwrap_or(0);
                        let mut comp_strings = Vec::new();

                        for comp_index in 1..=max_comp {
                            if let Some(comp_value) = components.get(&comp_index) {
                                comp_strings.push(comp_value.clone());
                            } else {
                                comp_strings.push(String::new());
                            }
                        }

                        if comp_strings.is_empty() {
                            if let Some(value) = components.get(&0) {
                                rep_strings.push(value.clone());
                            } else {
                                rep_strings.push(String::new());
                            }
                        } else {
                            rep_strings.push(comp_strings.join("^"));
                        }
                    } else {
                        rep_strings.push(String::new());
                    }
                }

                fields.push(rep_strings.join("~"));
            } else {
                fields.push(String::new());
            }
        }

        fields.join("|")
    }
}
