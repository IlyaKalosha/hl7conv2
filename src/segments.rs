use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Hl7Segment {
    pub segment_name: String,
    pub fields: BTreeMap<usize, Hl7Field>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hl7Field {
    pub value: String,
    pub components: Option<Vec<String>>,
}

impl Hl7Segment {
    pub fn from_string(segment_str: &str) -> Self {
        let parts: Vec<&str> = segment_str.split('|').collect();
        let segment_name = parts.first().unwrap_or(&"").to_string();

        let mut fields = BTreeMap::new();
        for (index, part) in parts.iter().enumerate() {
            if index == 0 {
                continue;
            }

            let field = Hl7Field::from_string(part);
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
            if let Some(ref components) = field.components {
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
    pub fn from_string(field_str: &str) -> Self {
        let value = field_str.to_string();
        let components = if field_str.contains('^') && !field_str.contains("^~\\&") {
            Some(field_str.split('^').map(|s| s.to_string()).collect())
        } else {
            None
        };

        Hl7Field { value, components }
    }
}
