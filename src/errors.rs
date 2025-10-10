use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Hl7Error {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("HL7 invalid format: {0}")]
    InvalidFormat(String),

    #[error("HL7 message is empty")]
    EmptyMessage,

    #[error("HL7 invalid segment: {0}")]
    InvalidSegment(String),

    #[error("HL7 parsing error at line {line}: {message}")]
    ParsingError { line: usize, message: String },

    #[error("HL7 validation error: {0}")]
    ValidationError(String),

    #[error("HL7 validation failed: {details}")]
    ValidationFailed { details: String },

    #[error("HL7 field error in segment {segment}, field {field}: {message}")]
    FieldError {
        segment: String,
        field: usize,
        message: String,
    },

    #[error(
        "HL7 component error in segment {segment}, field {field}, component {component}: {message}"
    )]
    ComponentError {
        segment: String,
        field: usize,
        component: usize,
        message: String,
    },

    #[error("Invalid escape sequence: {sequence} at position {position}")]
    InvalidEscapeSequence { sequence: String, position: usize },

    #[error("Field value exceeds maximum length: {length} > {max_length} characters")]
    FieldTooLong { length: usize, max_length: usize },

    #[error("Component value exceeds maximum length: {length} > {max_length} characters")]
    ComponentTooLong { length: usize, max_length: usize },

    #[error("Missing required segment: {segment}")]
    MissingRequiredSegment { segment: String },

    #[error("Invalid segment name: {name} (must be exactly 3 alphabetic characters)")]
    InvalidSegmentName { name: String },

    #[error("Invalid field separators: {separators} (must be exactly 4 characters)")]
    InvalidFieldSeparators { separators: String },

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("HL7 version not supported: {version} (supported versions: {supported_versions})")]
    UnsupportedVersion {
        version: String,
        supported_versions: String,
    },

    #[error("HL7 encoding error: {0}")]
    EncodingError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

impl From<Hl7Error> for PyErr {
    fn from(error: Hl7Error) -> Self {
        PyException::new_err(error.to_string())
    }
}

impl Hl7Error {
    pub fn validation_failed(details: &str) -> Self {
        Hl7Error::ValidationFailed {
            details: details.to_string(),
        }
    }

    pub fn field_error(segment: &str, field: usize, message: &str) -> Self {
        Hl7Error::FieldError {
            segment: segment.to_string(),
            field,
            message: message.to_string(),
        }
    }

    pub fn component_error(segment: &str, field: usize, component: usize, message: &str) -> Self {
        Hl7Error::ComponentError {
            segment: segment.to_string(),
            field,
            component,
            message: message.to_string(),
        }
    }

    pub fn parsing_error(line: usize, message: &str) -> Self {
        Hl7Error::ParsingError {
            line,
            message: message.to_string(),
        }
    }

    pub fn unsupported_version(version: &str) -> Self {
        let supported_versions = [
            "2.1", "2.2", "2.3", "2.4", "2.5", "2.5.1", "2.6", "2.7", "2.8", "2.9",
        ];
        Hl7Error::UnsupportedVersion {
            version: version.to_string(),
            supported_versions: supported_versions.join(", "),
        }
    }
}
