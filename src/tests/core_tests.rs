use crate::escape::Hl7EscapeHandler;
use crate::segments::{Hl7Field, Hl7Segment};
use crate::validation::Hl7Validator;

#[test]
fn test_validation_with_valid_message() {
    let segments = vec![
        Hl7Segment::from_string("MSH|^~\\&|ADT1|HOSPITAL|LAB|HOSPITAL|20240101120000|SECURITY|ADT^A01^ADT_A01^V2.5.1|MSG00001|T|2.5.1|2.5.1", None),
        Hl7Segment::from_string("PID|1||PATID1234||DOE^JOHN||19800101|M", None),
    ];

    let validator = Hl7Validator::new();
    let result = validator.validate_message(&segments);
    assert!(result.is_ok());
}

#[test]
fn test_validation_with_invalid_message() {
    let segments = vec![Hl7Segment::from_string("INVALID|SEGMENT", None)];

    let validator = Hl7Validator::new();
    let result = validator.validate_message(&segments);
    assert!(result.is_err());
}

#[test]
fn test_validation_lenient_mode() {
    let segments = vec![
        Hl7Segment::from_string("MSH|^~\\&|ADT1|HOSPITAL|LAB|HOSPITAL|20240101120000|SECURITY|ADT^A01^ADT_A01^V2.5.1|MSG00001|T|2.5.1|2.5.1", None),
    ];

    let validator = Hl7Validator::new()
        .with_strict_mode(false)
        .with_required_fields_validation(false);
    let result = validator.validate_message(&segments);
    assert!(result.is_ok());
}

#[test]
fn test_validator_custom_settings() {
    let segments = vec![
        Hl7Segment::from_string("MSH|^~\\&|ADT1|HOSPITAL|LAB|HOSPITAL|20240101120000|SECURITY|ADT^A01^ADT_A01^V2.5.1|MSG00001|T|2.5.1|2.5.1", None),
        Hl7Segment::from_string("PID|1||PATID1234||DOE^JOHN||19800101|M", None),
    ];

    let validator = Hl7Validator::new()
        .with_strict_mode(false)
        .with_required_fields_validation(false);

    let result = validator.validate_message(&segments);
    assert!(result.is_ok());
}

#[test]
fn test_escape_handler_basic() {
    let handler = Hl7EscapeHandler::default();

    let text = "Test\\F\\Field\\S\\Component";
    let unescaped = handler.unescape(text);
    assert_eq!(unescaped, "Test|Field^Component");

    let escaped = handler.escape("Test|Field^Component");
    assert_eq!(escaped, "Test\\F\\Field\\S\\Component");
}

#[test]
fn test_escape_handler_from_msh() {
    let handler = Hl7EscapeHandler::from_msh_field("^~\\&").unwrap();

    assert_eq!(handler.get_component_separator(), '~');
}

#[test]
fn test_escape_handler_sequences() {
    let handler = Hl7EscapeHandler::default();

    let text = "Line1\\X0A\\Line2";
    let unescaped = handler.unescape(text);
    assert_eq!(unescaped, "Line1\nLine2");

    let text = "Line1\\X0D\\Line2";
    let unescaped = handler.unescape(text);
    assert_eq!(unescaped, "Line1\rLine2");

    let text = "Word1\\X20\\Word2";
    let unescaped = handler.unescape(text);
    assert_eq!(unescaped, "Word1 Word2");
}

#[test]
fn test_escape_handler_double_escape() {
    let handler = Hl7EscapeHandler::default();

    let text = "Test\\\\Escape";
    let unescaped = handler.unescape(text);
    assert_eq!(unescaped, "Test\\Escape");
}

#[test]
fn test_field_with_escaping() {
    let handler = Hl7EscapeHandler::default();

    let field = Hl7Field::from_string("Test\\F\\Field\\S\\Component", Some(&handler));
    assert_eq!(field.value, "Test|Field^Component");
    assert_eq!(
        field.components,
        Some(vec!["Test|Field".to_string(), "Component".to_string()])
    );

    let back_to_string = handler.escape(&field.value);
    assert_eq!(back_to_string, "Test\\F\\Field\\S\\Component");
}
