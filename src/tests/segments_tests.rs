use crate::segments::{Hl7Field, Hl7Segment};

#[test]
fn test_segment_from_string() {
    let segment = Hl7Segment::from_string("MSH|^~\\&|ADT1|HOSPITAL|LAB", None);

    assert_eq!(segment.segment_name, "MSH");
    assert_eq!(segment.fields.len(), 4);
    assert_eq!(segment.fields.get(&1).unwrap().value, "^~\\&");
    assert_eq!(segment.fields.get(&2).unwrap().value, "ADT1");
}

#[test]
fn test_field_with_components() {
    let field = Hl7Field::from_string("ADT^A01^ADT_A01", None);

    assert_eq!(field.value, "ADT^A01^ADT_A01");
    assert_eq!(
        field.components,
        Some(vec![
            "ADT".to_string(),
            "A01".to_string(),
            "ADT_A01".to_string()
        ])
    );
    assert_eq!(
        field.components.as_ref().unwrap().first(),
        Some(&"ADT".to_string())
    );
    assert_eq!(
        field.components.as_ref().unwrap().get(1),
        Some(&"A01".to_string())
    );
    assert_eq!(
        field.components.as_ref().unwrap().get(2),
        Some(&"ADT_A01".to_string())
    );
}

#[test]
fn test_field_without_components() {
    let field = Hl7Field::from_string("ADT1", None);

    assert_eq!(field.value, "ADT1");
    assert_eq!(field.components, None);
}

#[test]
fn test_segment_to_json() {
    let segment = Hl7Segment::from_string("MSH|^~\\&|ADT1|HOSPITAL", None);
    let json = segment.to_json();

    assert_eq!(json.get("segment_name"), Some(&"MSH".to_string()));
    assert_eq!(json.get("1"), Some(&"^~\\&".to_string()));
    assert_eq!(json.get("2"), Some(&"ADT1".to_string()));
    assert_eq!(json.get("3"), Some(&"HOSPITAL".to_string()));
}
