use crate::errors::Hl7Error;
use pyo3::prelude::*;
use pyo3::types::PyType;
use pyo3::PyResult;
use std::collections::BTreeMap;

#[pyclass]
pub struct JsonHl7 {
    #[pyo3(get)]
    pub json_data: Vec<BTreeMap<String, String>>,
}

#[pymethods]
impl JsonHl7 {
    #[new]
    pub fn new(json_data: Vec<BTreeMap<String, String>>) -> Self {
        JsonHl7 { json_data }
    }

    #[classmethod]
    pub fn from_file(_cls: &Bound<PyType>, path: String) -> PyResult<Self> {
        let contents = std::fs::read_to_string(&path).map_err(Hl7Error::IoError)?;

        if contents.trim().is_empty() {
            return Err(Hl7Error::EmptyMessage.into());
        }

        let json_data: Vec<BTreeMap<String, String>> = serde_json::from_str(&contents)
            .map_err(|e| Hl7Error::InvalidFormat(format!("Invalid JSON: {}", e)))?;

        Ok(JsonHl7 { json_data })
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

        let mut field_map: BTreeMap<usize, String> = BTreeMap::new();

        for (key, value) in segment_json {
            if key == "segment_name" {
                continue;
            }

            if let Ok(field_index) = key.parse::<usize>() {
                field_map.insert(field_index, value.clone());
            } else if key.contains('.') {
                let parts: Vec<&str> = key.split('.').collect();
                if parts.len() == 2 {
                    if let (Ok(parent_index), Ok(component_index)) =
                        (parts[0].parse::<usize>(), parts[1].parse::<usize>())
                    {
                        if let Some(existing) = field_map.get(&parent_index) {
                            let components: Vec<&str> = existing.split('^').collect();
                            let mut new_components =
                                vec![""; component_index.max(components.len())];

                            for (i, component) in components.iter().enumerate() {
                                if i < new_components.len() {
                                    new_components[i] = component;
                                }
                            }

                            if component_index <= new_components.len() {
                                new_components[component_index - 1] = value;
                            }

                            field_map.insert(parent_index, new_components.join("^"));
                        } else {
                            let mut components = vec![""; component_index];
                            components[component_index - 1] = value;
                            field_map.insert(parent_index, components.join("^"));
                        }
                    }
                }
            }
        }

        let max_index = field_map.keys().max().copied().unwrap_or(0);
        for i in 1..=max_index {
            if let Some(field_value) = field_map.get(&i) {
                fields.push(field_value.clone());
            } else {
                fields.push(String::new());
            }
        }

        fields.join("|")
    }
}
