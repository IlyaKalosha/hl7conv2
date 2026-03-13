use crate::segments::{Hl7Field, Hl7Segment};
use crate::validation::Hl7Validator;
use std::collections::BTreeMap;

fn segment_with_fields(name: &str, fields: BTreeMap<usize, Hl7Field>) -> Hl7Segment {
    Hl7Segment {
        segment_name: name.to_string(),
        fields,
    }
}

fn simple_field(value: &str) -> Hl7Field {
    Hl7Field {
        value: value.to_string(),
        components: None,
        repetitions: None,
    }
}

fn msh_fields(f1: &str, f8: &str, f12: &str) -> BTreeMap<usize, Hl7Field> {
    let mut m: BTreeMap<usize, Hl7Field> = BTreeMap::new();
    m.insert(1, simple_field(f1));
    m.insert(2, simple_field("ADT1"));
    m.insert(3, simple_field("HOSP"));
    m.insert(4, simple_field("LAB"));
    m.insert(5, simple_field("HOSP"));
    m.insert(6, simple_field("20240101120000"));
    m.insert(7, simple_field("SECURITY"));
    m.insert(8, simple_field(f8));
    m.insert(9, simple_field("MSG00001"));
    m.insert(10, simple_field("T"));
    m.insert(11, simple_field("2.5.1"));
    m.insert(12, simple_field(f12));
    m
}

#[test]
fn validation_empty_segments() {
    let validator = Hl7Validator::new();
    let segments: Vec<Hl7Segment> = vec![];
    let result = validator.validate_message(&segments);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("no segments"));
}

#[test]
fn validation_missing_msh() {
    let validator = Hl7Validator::new();
    let seg = segment_with_fields("OBX", BTreeMap::new());
    let result = validator.validate_message(&[seg]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("MSH"));
}

#[test]
fn validation_msh_too_few_fields() {
    let validator = Hl7Validator::new();
    let mut fields = BTreeMap::new();
    for i in 1..=11 {
        fields.insert(i, simple_field("x"));
    }
    let seg = segment_with_fields("MSH", fields);
    let result = validator.validate_message(&[seg]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("12 fields"));
}

#[test]
fn validation_field_separators_wrong_length() {
    let validator = Hl7Validator::new();
    let fields = msh_fields("^~\\", "ADT^A01^ADT_A01", "2.5.1");
    let seg = segment_with_fields("MSH", fields);
    let result = validator.validate_message(&[seg]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("4 characters"));
}

#[test]
fn validation_field_separators_missing() {
    let validator = Hl7Validator::new();
    let mut fields = msh_fields("^~\\&", "ADT^A01^ADT_A01", "2.5.1");
    fields.remove(&1);
    let seg = segment_with_fields("MSH", fields);
    let result = validator.validate_message(&[seg]);
    assert!(result.is_err());
}

#[test]
fn validation_unsupported_version_strict() {
    let validator = Hl7Validator::new().with_strict_mode(true);
    let fields = msh_fields("^~\\&", "ADT^A01^ADT_A01", "99.99");
    let seg = segment_with_fields("MSH", fields);
    let result = validator.validate_message(&[seg]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("not supported"));
}

#[test]
fn validation_version_accept_non_strict() {
    let validator = Hl7Validator::new().with_strict_mode(false);
    let mut fields = msh_fields("^~\\&", "ADT^A01^ADT_A01", "2.5.1");
    fields.insert(12, simple_field("99.99"));
    let seg = segment_with_fields("MSH", fields);
    let pid = Hl7Segment::from_string("PID|1||x", None);
    let result = validator.validate_message(&[seg, pid]);
    assert!(result.is_ok());
}

#[test]
fn validation_missing_field_12() {
    let validator = Hl7Validator::new();
    let mut fields = msh_fields("^~\\&", "ADT^A01^ADT_A01", "2.5.1");
    fields.remove(&12);
    let seg = segment_with_fields("MSH", fields);
    let result = validator.validate_message(&[seg]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let s = err.to_string();
    assert!(s.contains("field 12") || s.contains("12 fields"));
}

#[test]
fn validation_message_type_too_few_components() {
    let validator = Hl7Validator::new().with_strict_mode(true);
    let fields = msh_fields("^~\\&", "ADT^A01", "2.5.1");
    let seg = segment_with_fields("MSH", fields);
    let pid = Hl7Segment::from_string("PID|1||x", None);
    let result = validator.validate_message(&[seg, pid]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("3 components"));
}

#[test]
fn validation_message_type_missing() {
    let validator = Hl7Validator::new();
    let mut fields = msh_fields("^~\\&", "ADT^A01^ADT_A01", "2.5.1");
    fields.remove(&8);
    let seg = segment_with_fields("MSH", fields);
    let result = validator.validate_message(&[seg]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    let s = err.to_string();
    assert!(s.contains("field 8") || s.contains("12 fields"));
}

#[test]
fn validation_missing_pid() {
    let validator = Hl7Validator::new();
    let fields = msh_fields("^~\\&", "ADT^A01^ADT_A01", "2.5.1");
    let seg = segment_with_fields("MSH", fields);
    let result = validator.validate_message(&[seg]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("PID"));
}

#[test]
fn validation_no_pid_when_required_fields_disabled() {
    let validator = Hl7Validator::new().with_required_fields_validation(false);
    let fields = msh_fields("^~\\&", "ADT^A01^ADT_A01", "2.5.1");
    let seg = segment_with_fields("MSH", fields);
    let result = validator.validate_message(&[seg]);
    assert!(result.is_ok());
}

#[test]
fn validation_segment_empty_name() {
    let validator = Hl7Validator::new().with_required_fields_validation(false);
    let fields = msh_fields("^~\\&", "ADT^A01^ADT_A01", "2.5.1");
    let msh = segment_with_fields("MSH", fields);
    let empty_name = segment_with_fields("", BTreeMap::new());
    let result = validator.validate_message(&[msh, empty_name]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("empty segment name"));
}

#[test]
fn validation_segment_name_wrong_length() {
    let validator = Hl7Validator::new().with_required_fields_validation(false);
    let fields = msh_fields("^~\\&", "ADT^A01^ADT_A01", "2.5.1");
    let msh = segment_with_fields("MSH", fields);
    let bad = segment_with_fields("AB", BTreeMap::new());
    let result = validator.validate_message(&[msh, bad]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("exactly 3"));
}

#[test]
fn validation_segment_name_non_alphabetic() {
    let validator = Hl7Validator::new().with_required_fields_validation(false);
    let fields = msh_fields("^~\\&", "ADT^A01^ADT_A01", "2.5.1");
    let msh = segment_with_fields("MSH", fields);
    let bad = segment_with_fields("M12", BTreeMap::new());
    let result = validator.validate_message(&[msh, bad]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("3 alphabetic"));
}

#[test]
fn validation_field_too_long() {
    let validator = Hl7Validator::new().with_required_fields_validation(false);
    let mut fields = msh_fields("^~\\&", "ADT^A01^ADT_A01", "2.5.1");
    let long_value = "x".repeat(65537);
    fields.insert(2, simple_field(&long_value));
    let msh = segment_with_fields("MSH", fields);
    let pid = Hl7Segment::from_string("PID|1||x", None);
    let result = validator.validate_message(&[msh, pid]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("exceeds maximum length"));
}

#[test]
fn validation_component_too_long() {
    let validator = Hl7Validator::new().with_required_fields_validation(false);
    let mut fields = msh_fields("^~\\&", "ADT^A01^ADT_A01", "2.5.1");
    let long_comp = "x".repeat(65537);
    let field_with_long_component = Hl7Field {
        value: "a^b".to_string(),
        components: Some(vec!["a".to_string(), long_comp]),
        repetitions: None,
    };
    fields.insert(2, field_with_long_component);
    let msh = segment_with_fields("MSH", fields);
    let pid = Hl7Segment::from_string("PID|1||x", None);
    let result = validator.validate_message(&[msh, pid]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("exceeds maximum length"));
}

#[test]
fn validation_builder_with_strict_and_required() {
    let v = Hl7Validator::new()
        .with_strict_mode(false)
        .with_required_fields_validation(true);
    assert!(!v.strict_mode);
    assert!(v.validate_required_fields);
}
