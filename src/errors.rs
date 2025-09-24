use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::fmt;

#[derive(Debug)]
pub enum Hl7Error {
    IoError(std::io::Error),
    InvalidFormat(String),
    EmptyMessage,
    InvalidSegment(String),
}

impl fmt::Display for Hl7Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Hl7Error::IoError(e) => write!(f, "IO error: {}", e),
            Hl7Error::InvalidFormat(msg) => write!(f, "Invalid HL7 format: {}", msg),
            Hl7Error::EmptyMessage => write!(f, "Empty HL7 message"),
            Hl7Error::InvalidSegment(msg) => write!(f, "Invalid segment: {}", msg),
        }
    }
}

impl std::error::Error for Hl7Error {}

impl From<std::io::Error> for Hl7Error {
    fn from(error: std::io::Error) -> Self {
        Hl7Error::IoError(error)
    }
}

impl From<Hl7Error> for PyErr {
    fn from(error: Hl7Error) -> Self {
        PyException::new_err(error.to_string())
    }
}
