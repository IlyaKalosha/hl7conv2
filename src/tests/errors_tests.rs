use crate::errors::Hl7Error;
use std::io;

#[test]
fn error_display_io() {
    let e = Hl7Error::IoError(io::Error::new(io::ErrorKind::NotFound, "file not found"));
    assert!(e.to_string().contains("I/O") || e.to_string().contains("file not found"));
}

#[test]
fn error_display_invalid_format() {
    let e = Hl7Error::InvalidFormat("bad".to_string());
    assert!(e.to_string().contains("invalid format") && e.to_string().contains("bad"));
}

#[test]
fn error_display_empty_message() {
    let e = Hl7Error::EmptyMessage;
    assert!(e.to_string().contains("empty"));
}

#[test]
fn error_display_invalid_segment() {
    let e = Hl7Error::InvalidSegment("X".to_string());
    assert!(e.to_string().contains("invalid segment") && e.to_string().contains("X"));
}

#[test]
fn error_display_parsing_error() {
    let e = Hl7Error::parsing_error(1, "msg");
    assert!(e.to_string().contains("line 1") && e.to_string().contains("msg"));
}

#[test]
fn error_display_validation_error() {
    let e = Hl7Error::ValidationError("fail".to_string());
    assert!(e.to_string().contains("validation") && e.to_string().contains("fail"));
}

#[test]
fn error_display_validation_failed() {
    let e = Hl7Error::validation_failed("details");
    assert!(e.to_string().contains("details"));
}

#[test]
fn error_display_field_error() {
    let e = Hl7Error::field_error("SEG", 2, "bad field");
    assert!(
        e.to_string().contains("SEG")
            && e.to_string().contains("2")
            && e.to_string().contains("bad field")
    );
}

#[test]
fn error_display_component_error() {
    let e = Hl7Error::component_error("SEG", 2, 1, "bad");
    assert!(
        e.to_string().contains("SEG")
            && e.to_string().contains("2")
            && e.to_string().contains("1")
            && e.to_string().contains("bad")
    );
}

#[test]
fn error_display_invalid_escape_sequence() {
    let e = Hl7Error::InvalidEscapeSequence {
        sequence: "X99".to_string(),
        position: 5,
    };
    assert!(e.to_string().contains("X99") && e.to_string().contains("5"));
}

#[test]
fn error_display_field_too_long() {
    let e = Hl7Error::FieldTooLong {
        length: 100,
        max_length: 64,
    };
    assert!(e.to_string().contains("100") && e.to_string().contains("64"));
}

#[test]
fn error_display_component_too_long() {
    let e = Hl7Error::ComponentTooLong {
        length: 100,
        max_length: 64,
    };
    assert!(e.to_string().contains("100") && e.to_string().contains("64"));
}

#[test]
fn error_display_missing_required_segment() {
    let e = Hl7Error::MissingRequiredSegment {
        segment: "PID".to_string(),
    };
    assert!(e.to_string().contains("PID"));
}

#[test]
fn error_display_invalid_segment_name() {
    let e = Hl7Error::InvalidSegmentName {
        name: "AB".to_string(),
    };
    assert!(e.to_string().contains("AB") && e.to_string().contains("3"));
}

#[test]
fn error_display_invalid_field_separators() {
    let e = Hl7Error::InvalidFieldSeparators {
        separators: "^^".to_string(),
    };
    assert!(e.to_string().contains("^^") && e.to_string().contains("4"));
}

#[test]
fn error_display_json_error() {
    let e = Hl7Error::JsonError(serde_json::from_str::<()>("{").unwrap_err());
    let s = e.to_string();
    assert!(
        s.to_lowercase().contains("json") || s.contains("EOF") || s.to_lowercase().contains("eof")
    );
}

#[test]
fn error_display_unsupported_version() {
    let e = Hl7Error::unsupported_version("99");
    assert!(e.to_string().contains("99") && e.to_string().contains("supported"));
}

#[test]
fn error_display_encoding_error() {
    let e = Hl7Error::EncodingError("utf8".to_string());
    assert!(e.to_string().contains("encoding") && e.to_string().contains("utf8"));
}

#[test]
fn error_display_configuration_error() {
    let e = Hl7Error::ConfigurationError("config".to_string());
    assert!(e.to_string().contains("Configuration") && e.to_string().contains("config"));
}
