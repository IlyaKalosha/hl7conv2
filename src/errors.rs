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

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("HL7 version not supported: {0}")]
    UnsupportedVersion(String),

    #[error("HL7 encoding error: {0}")]
    EncodingError(String),
}

impl From<Hl7Error> for PyErr {
    fn from(error: Hl7Error) -> Self {
        PyException::new_err(error.to_string())
    }
}
