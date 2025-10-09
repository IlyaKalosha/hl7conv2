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

#[test]
fn test_field_with_repetitions() {
    let field = Hl7Field::from_string("HbA1c~CBC~Glucose", None);

    assert_eq!(field.value, "HbA1c~CBC~Glucose");
    assert!(field.components.is_none());
    assert!(field.repetitions.is_some());

    let repetitions = field.repetitions.unwrap();
    assert_eq!(repetitions.len(), 3);
    assert_eq!(repetitions[0].value, "HbA1c");
    assert_eq!(repetitions[1].value, "CBC");
    assert_eq!(repetitions[2].value, "Glucose");
}

#[test]
fn test_field_with_repetitions_and_components() {
    let field = Hl7Field::from_string("HbA1c^Test1~CBC^Test2~Glucose^Test3", None);

    assert_eq!(field.value, "HbA1c^Test1~CBC^Test2~Glucose^Test3");
    assert!(field.components.is_none());
    assert!(field.repetitions.is_some());

    let repetitions = field.repetitions.unwrap();
    assert_eq!(repetitions.len(), 3);

    assert_eq!(repetitions[0].value, "HbA1c^Test1");
    assert_eq!(
        repetitions[0].components,
        Some(vec!["HbA1c".to_string(), "Test1".to_string()])
    );

    assert_eq!(repetitions[1].value, "CBC^Test2");
    assert_eq!(
        repetitions[1].components,
        Some(vec!["CBC".to_string(), "Test2".to_string()])
    );

    assert_eq!(repetitions[2].value, "Glucose^Test3");
    assert_eq!(
        repetitions[2].components,
        Some(vec!["Glucose".to_string(), "Test3".to_string()])
    );
}

#[test]
fn test_segment_to_json_with_repetitions() {
    let segment = Hl7Segment::from_string("OBX|1|TX|HbA1c~CBC~Glucose||Result", None);
    let json = segment.to_json();

    assert_eq!(json.get("segment_name"), Some(&"OBX".to_string()));
    assert_eq!(json.get("1"), Some(&"1".to_string()));
    assert_eq!(json.get("2"), Some(&"TX".to_string()));
    assert_eq!(json.get("3[0]"), Some(&"HbA1c".to_string()));
    assert_eq!(json.get("3[1]"), Some(&"CBC".to_string()));
    assert_eq!(json.get("3[2]"), Some(&"Glucose".to_string()));
    assert_eq!(json.get("4"), Some(&"".to_string()));
    assert_eq!(json.get("5"), Some(&"Result".to_string()));
}

#[test]
fn test_segment_to_json_with_repetitions_and_components() {
    let segment = Hl7Segment::from_string("OBX|1|TX|HbA1c^Test1~CBC^Test2||Result", None);
    let json = segment.to_json();

    assert_eq!(json.get("segment_name"), Some(&"OBX".to_string()));
    assert_eq!(json.get("1"), Some(&"1".to_string()));
    assert_eq!(json.get("2"), Some(&"TX".to_string()));
    assert_eq!(json.get("3[0].1"), Some(&"HbA1c".to_string()));
    assert_eq!(json.get("3[0].2"), Some(&"Test1".to_string()));
    assert_eq!(json.get("3[1].1"), Some(&"CBC".to_string()));
    assert_eq!(json.get("3[1].2"), Some(&"Test2".to_string()));
    assert_eq!(json.get("4"), Some(&"".to_string()));
    assert_eq!(json.get("5"), Some(&"Result".to_string()));
}
