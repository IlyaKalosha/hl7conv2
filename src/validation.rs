use crate::errors::Hl7Error;
use crate::segments::{Hl7Field, Hl7Segment};

#[derive(Debug, Clone)]
pub struct Hl7Validator {
    pub strict_mode: bool,
    pub validate_required_fields: bool,
}

impl Default for Hl7Validator {
    fn default() -> Self {
        Self {
            strict_mode: true,
            validate_required_fields: true,
        }
    }
}

impl Hl7Validator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }

    pub fn with_required_fields_validation(mut self, validate: bool) -> Self {
        self.validate_required_fields = validate;
        self
    }

    pub fn validate_message(&self, segments: &[Hl7Segment]) -> Result<(), Hl7Error> {
        if segments.is_empty() {
            return Err(Hl7Error::ValidationError(
                "Message contains no segments".to_string(),
            ));
        }

        self.validate_msh_segment(&segments[0])?;

        if self.validate_required_fields {
            self.validate_required_segments(segments)?;
        }

        for (i, segment) in segments.iter().enumerate() {
            self.validate_segment(segment, i)?;
        }

        Ok(())
    }

    fn validate_msh_segment(&self, msh: &Hl7Segment) -> Result<(), Hl7Error> {
        if msh.segment_name != "MSH" {
            return Err(Hl7Error::MissingRequiredSegment {
                segment: "MSH (Message Header)".to_string(),
            });
        }

        if msh.fields.len() < 12 {
            return Err(Hl7Error::ValidationFailed {
                details: "MSH segment must have at least 12 fields".to_string(),
            });
        }

        self.validate_field_separators(msh)?;
        self.validate_hl7_version(msh)?;
        self.validate_message_type(msh)?;

        Ok(())
    }

    fn validate_field_separators(&self, msh: &Hl7Segment) -> Result<(), Hl7Error> {
        if let Some(field_1) = msh.fields.get(&1) {
            if field_1.value.len() != 4 {
                return Err(Hl7Error::InvalidFieldSeparators {
                    separators: field_1.value.clone(),
                });
            }
        } else {
            return Err(Hl7Error::ValidationFailed {
                details: "Field separators (field 1) are required".to_string(),
            });
        }

        Ok(())
    }

    fn validate_hl7_version(&self, msh: &Hl7Segment) -> Result<(), Hl7Error> {
        if let Some(field_12) = msh.fields.get(&12) {
            let version = &field_12.value;
            let supported_versions = [
                "2.1", "2.2", "2.3", "2.4", "2.5", "2.5.1", "2.6", "2.7", "2.8", "2.9",
            ];

            if self.strict_mode && !supported_versions.contains(&version.as_str()) {
                return Err(Hl7Error::UnsupportedVersion {
                    version: version.clone(),
                    supported_versions: supported_versions.join(", "),
                });
            }
        } else {
            return Err(Hl7Error::ValidationFailed {
                details: "HL7 version (field 12) is required".to_string(),
            });
        }

        Ok(())
    }

    fn validate_message_type(&self, msh: &Hl7Segment) -> Result<(), Hl7Error> {
        if let Some(field_8) = msh.fields.get(&8) {
            let message_type = &field_8.value;

            if self.strict_mode {
                let parts: Vec<&str> = message_type.split('^').collect();
                if parts.len() < 3 {
                    return Err(Hl7Error::ValidationFailed {
                        details: "Message type (field 8) must have at least 3 components: event^structure^version".to_string(),
                    });
                }
            }
        } else {
            return Err(Hl7Error::ValidationFailed {
                details: "Message type (field 8) is required".to_string(),
            });
        }

        Ok(())
    }

    fn validate_required_segments(&self, segments: &[Hl7Segment]) -> Result<(), Hl7Error> {
        let mut has_pid = false;
        let mut _has_evn = false;

        for segment in segments {
            match segment.segment_name.as_str() {
                "PID" => has_pid = true,
                "EVN" => _has_evn = true,
                _ => {}
            }
        }

        if !has_pid {
            return Err(Hl7Error::MissingRequiredSegment {
                segment: "PID (Patient Identification)".to_string(),
            });
        }

        Ok(())
    }

    fn validate_segment(&self, segment: &Hl7Segment, index: usize) -> Result<(), Hl7Error> {
        if segment.segment_name.is_empty() {
            return Err(Hl7Error::ValidationFailed {
                details: format!("Segment {} has empty segment name", index + 1),
            });
        }

        if segment.segment_name.len() != 3 {
            return Err(Hl7Error::InvalidSegmentName {
                name: segment.segment_name.clone(),
            });
        }

        if !segment
            .segment_name
            .chars()
            .all(|c| c.is_ascii_alphabetic())
        {
            return Err(Hl7Error::InvalidSegmentName {
                name: segment.segment_name.clone(),
            });
        }

        for (field_index, field) in &segment.fields {
            self.validate_field(segment, *field_index, field)?;
        }

        Ok(())
    }

    fn validate_field(
        &self,
        _segment: &Hl7Segment,
        _field_index: usize,
        field: &Hl7Field,
    ) -> Result<(), Hl7Error> {
        const MAX_FIELD_LENGTH: usize = 65536;

        if field.value.len() > MAX_FIELD_LENGTH {
            return Err(Hl7Error::FieldTooLong {
                length: field.value.len(),
                max_length: MAX_FIELD_LENGTH,
            });
        }

        if let Some(components) = &field.components {
            for component in components.iter() {
                if component.len() > MAX_FIELD_LENGTH {
                    return Err(Hl7Error::ComponentTooLong {
                        length: component.len(),
                        max_length: MAX_FIELD_LENGTH,
                    });
                }
            }
        }

        Ok(())
    }
}
