use crate::escape::Hl7EscapeHandler;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hl7Segment {
    pub segment_name: String,
    pub fields: BTreeMap<usize, Hl7Field>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hl7Field {
    pub value: String,
    pub components: Option<Vec<String>>,
    pub repetitions: Option<Vec<Hl7Field>>,
}

impl Hl7Segment {
    pub fn from_string(segment_str: &str, escape_handler: Option<&Hl7EscapeHandler>) -> Self {
        let parts: Vec<&str> = segment_str.split('|').collect();
        let segment_name = parts.first().unwrap_or(&"").to_string();

        let mut fields = BTreeMap::new();
        for (index, part) in parts.iter().enumerate() {
            if index == 0 {
                continue;
            }

            let field = Hl7Field::from_string(part, escape_handler);
            fields.insert(index, field);
        }

        Hl7Segment {
            segment_name,
            fields,
        }
    }

    pub fn to_json(&self) -> BTreeMap<String, String> {
        let mut json = BTreeMap::new();
        json.insert("segment_name".to_string(), self.segment_name.clone());

        for (index, field) in &self.fields {
            if let Some(ref repetitions) = field.repetitions {
                for (rep_index, repetition) in repetitions.iter().enumerate() {
                    if let Some(ref components) = repetition.components {
                        for (comp_index, component) in components.iter().enumerate() {
                            let key = format!("{}[{}].{}", index, rep_index, comp_index + 1);
                            json.insert(key, component.clone());
                        }
                    } else {
                        let key = format!("{}[{}]", index, rep_index);
                        json.insert(key, repetition.value.clone());
                    }
                }
            } else if let Some(ref components) = field.components {
                for (comp_index, component) in components.iter().enumerate() {
                    let key = format!("{}.{}", index, comp_index + 1);
                    json.insert(key, component.clone());
                }
            } else {
                json.insert(index.to_string(), field.value.clone());
            }
        }

        json
    }
}

impl Hl7Field {
    pub fn from_string(field_str: &str, escape_handler: Option<&Hl7EscapeHandler>) -> Self {
        let repetition_separator = if let Some(handler) = escape_handler {
            handler.get_repetition_separator()
        } else {
            '~'
        };

        if field_str.contains(repetition_separator) && !field_str.contains("^~\\&") {
            let repetitions: Vec<Hl7Field> = field_str
                .split(repetition_separator)
                .map(|rep_str| {
                    let (value, components) = if let Some(handler) = escape_handler {
                        let unescaped_value = handler.unescape(rep_str);
                        let components =
                            if unescaped_value.contains(handler.get_component_separator()) {
                                Some(handler.parse_field_with_escaping(rep_str))
                            } else {
                                None
                            };
                        (unescaped_value, components)
                    } else {
                        let value = rep_str.to_string();
                        let components = if rep_str.contains('^') && !rep_str.contains("^~\\&") {
                            Some(rep_str.split('^').map(|s| s.to_string()).collect())
                        } else {
                            None
                        };
                        (value, components)
                    };
                    Hl7Field {
                        value,
                        components,
                        repetitions: None,
                    }
                })
                .collect();

            Hl7Field {
                value: field_str.to_string(),
                components: None,
                repetitions: Some(repetitions),
            }
        } else {
            let (value, components) = if let Some(handler) = escape_handler {
                let unescaped_value = handler.unescape(field_str);
                let components = if unescaped_value.contains(handler.get_component_separator()) {
                    Some(handler.parse_field_with_escaping(field_str))
                } else {
                    None
                };
                (unescaped_value, components)
            } else {
                let value = field_str.to_string();
                let components = if field_str.contains('^') && !field_str.contains("^~\\&") {
                    Some(field_str.split('^').map(|s| s.to_string()).collect())
                } else {
                    None
                };
                (value, components)
            };

            Hl7Field {
                value,
                components,
                repetitions: None,
            }
        }
    }
}
