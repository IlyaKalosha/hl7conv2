use crate::json_hl7::JsonHl7;
use std::collections::BTreeMap;

#[test]
fn test_json_to_hl7_with_repetitions() {
    let mut segment_json = BTreeMap::new();
    segment_json.insert("segment_name".to_string(), "OBX".to_string());
    segment_json.insert("1".to_string(), "1".to_string());
    segment_json.insert("2".to_string(), "TX".to_string());
    segment_json.insert("3[0]".to_string(), "HbA1c".to_string());
    segment_json.insert("3[1]".to_string(), "CBC".to_string());
    segment_json.insert("3[2]".to_string(), "Glucose".to_string());
    segment_json.insert("4".to_string(), "".to_string());
    segment_json.insert("5".to_string(), "Result".to_string());

    let json_hl7 = JsonHl7::new(vec![segment_json]);
    let hl7_string = json_hl7._convert_json_to_hl7();

    assert_eq!(hl7_string, "OBX|1|TX|HbA1c~CBC~Glucose||Result");
}

#[test]
fn test_json_to_hl7_with_repetitions_and_components() {
    let mut segment_json = BTreeMap::new();
    segment_json.insert("segment_name".to_string(), "OBX".to_string());
    segment_json.insert("1".to_string(), "1".to_string());
    segment_json.insert("2".to_string(), "TX".to_string());
    segment_json.insert("3[0].1".to_string(), "HbA1c".to_string());
    segment_json.insert("3[0].2".to_string(), "Test1".to_string());
    segment_json.insert("3[1].1".to_string(), "CBC".to_string());
    segment_json.insert("3[1].2".to_string(), "Test2".to_string());
    segment_json.insert("4".to_string(), "".to_string());
    segment_json.insert("5".to_string(), "Result".to_string());

    let json_hl7 = JsonHl7::new(vec![segment_json]);
    let hl7_string = json_hl7._convert_json_to_hl7();

    assert_eq!(hl7_string, "OBX|1|TX|HbA1c^Test1~CBC^Test2||Result");
}

#[test]
fn test_json_to_hl7_roundtrip() {
    let original_hl7 = "OBX|1|TX|HbA1c^Test1~CBC^Test2||Result";

    let mut segment_json = BTreeMap::new();
    segment_json.insert("segment_name".to_string(), "OBX".to_string());
    segment_json.insert("1".to_string(), "1".to_string());
    segment_json.insert("2".to_string(), "TX".to_string());
    segment_json.insert("3[0].1".to_string(), "HbA1c".to_string());
    segment_json.insert("3[0].2".to_string(), "Test1".to_string());
    segment_json.insert("3[1].1".to_string(), "CBC".to_string());
    segment_json.insert("3[1].2".to_string(), "Test2".to_string());
    segment_json.insert("4".to_string(), "".to_string());
    segment_json.insert("5".to_string(), "Result".to_string());

    let json_hl7 = JsonHl7::new(vec![segment_json]);
    let converted_hl7 = json_hl7._convert_json_to_hl7();

    assert_eq!(converted_hl7, original_hl7);
}

#[test]
fn test_json_to_hl7_mixed_repetitions_and_regular_fields() {
    let mut segment_json = BTreeMap::new();
    segment_json.insert("segment_name".to_string(), "OBX".to_string());
    segment_json.insert("1".to_string(), "1".to_string());
    segment_json.insert("2".to_string(), "TX".to_string());
    segment_json.insert("3[0]".to_string(), "HbA1c".to_string());
    segment_json.insert("3[1]".to_string(), "CBC".to_string());
    segment_json.insert("4.1".to_string(), "Component1".to_string());
    segment_json.insert("4.2".to_string(), "Component2".to_string());
    segment_json.insert("5".to_string(), "Result".to_string());

    let json_hl7 = JsonHl7::new(vec![segment_json]);
    let hl7_string = json_hl7._convert_json_to_hl7();

    assert_eq!(
        hl7_string,
        "OBX|1|TX|HbA1c~CBC|Component1^Component2|Result"
    );
}

#[test]
fn test_json_to_hl7_escaping_component_separator() {
    let mut segment_json = BTreeMap::new();
    segment_json.insert("segment_name".to_string(), "PID".to_string());
    segment_json.insert("1".to_string(), "1".to_string());
    segment_json.insert("5.1".to_string(), "DOE^JOHN".to_string());
    segment_json.insert("5.2".to_string(), "JOHN".to_string());

    let json_hl7 = JsonHl7::new(vec![segment_json]);
    let hl7_string = json_hl7._convert_json_to_hl7();

    assert_eq!(hl7_string, "PID|1||||DOE\\S\\JOHN^JOHN");
}

#[test]
fn test_json_to_hl7_escaping_field_separator() {
    let mut segment_json = BTreeMap::new();
    segment_json.insert("segment_name".to_string(), "PID".to_string());
    segment_json.insert("1".to_string(), "1".to_string());
    segment_json.insert("5.1".to_string(), "DOE|JOHN".to_string());

    let json_hl7 = JsonHl7::new(vec![segment_json]);
    let hl7_string = json_hl7._convert_json_to_hl7();

    assert_eq!(hl7_string, "PID|1||||DOE\\F\\JOHN");
}

#[test]
fn test_json_to_hl7_escaping_repetition_separator() {
    let mut segment_json = BTreeMap::new();
    segment_json.insert("segment_name".to_string(), "PID".to_string());
    segment_json.insert("1".to_string(), "1".to_string());
    segment_json.insert("5.1".to_string(), "DOE~JOHN".to_string());

    let json_hl7 = JsonHl7::new(vec![segment_json]);
    let hl7_string = json_hl7._convert_json_to_hl7();

    assert_eq!(hl7_string, "PID|1||||DOE\\R\\JOHN");
}

#[test]
fn test_json_to_hl7_escaping_escape_character() {
    let mut segment_json = BTreeMap::new();
    segment_json.insert("segment_name".to_string(), "PID".to_string());
    segment_json.insert("1".to_string(), "1".to_string());
    segment_json.insert("5.1".to_string(), "DOE\\JOHN".to_string());

    let json_hl7 = JsonHl7::new(vec![segment_json]);
    let hl7_string = json_hl7._convert_json_to_hl7();

    assert_eq!(hl7_string, "PID|1||||DOE\\\\JOHN");
}

#[test]
fn test_json_to_hl7_escaping_subcomponent_separator() {
    let mut segment_json = BTreeMap::new();
    segment_json.insert("segment_name".to_string(), "PID".to_string());
    segment_json.insert("1".to_string(), "1".to_string());
    segment_json.insert("5.1".to_string(), "DOE&JOHN".to_string());

    let json_hl7 = JsonHl7::new(vec![segment_json]);
    let hl7_string = json_hl7._convert_json_to_hl7();

    assert_eq!(hl7_string, "PID|1||||DOE\\T\\JOHN");
}

#[test]
fn test_json_to_hl7_escaping_multiple_special_chars() {
    let mut segment_json = BTreeMap::new();
    segment_json.insert("segment_name".to_string(), "PID".to_string());
    segment_json.insert("1".to_string(), "1".to_string());
    segment_json.insert(
        "5.1".to_string(),
        "DOE^JOHN|SMITH~BROWN\\JONES&WILSON".to_string(),
    );

    let json_hl7 = JsonHl7::new(vec![segment_json]);
    let hl7_string = json_hl7._convert_json_to_hl7();

    assert_eq!(
        hl7_string,
        "PID|1||||DOE\\S\\JOHN\\F\\SMITH\\R\\BROWN\\\\JONES\\T\\WILSON"
    );
}

#[test]
fn test_json_to_hl7_escaping_with_repetitions() {
    let mut segment_json = BTreeMap::new();
    segment_json.insert("segment_name".to_string(), "PID".to_string());
    segment_json.insert("1".to_string(), "1".to_string());
    segment_json.insert("3[0]".to_string(), "ID1^TYPE1".to_string());
    segment_json.insert("3[1]".to_string(), "ID2^TYPE2".to_string());

    let json_hl7 = JsonHl7::new(vec![segment_json]);
    let hl7_string = json_hl7._convert_json_to_hl7();

    assert_eq!(hl7_string, "PID|1||ID1\\S\\TYPE1~ID2\\S\\TYPE2");
}

#[test]
fn test_json_to_hl7_escaping_with_components() {
    let mut segment_json = BTreeMap::new();
    segment_json.insert("segment_name".to_string(), "PID".to_string());
    segment_json.insert("1".to_string(), "1".to_string());
    segment_json.insert("5.1".to_string(), "DOE".to_string());
    segment_json.insert("5.2".to_string(), "JOHN^MIDDLE".to_string());
    segment_json.insert("5.3".to_string(), "A".to_string());

    let json_hl7 = JsonHl7::new(vec![segment_json]);
    let hl7_string = json_hl7._convert_json_to_hl7();

    assert_eq!(hl7_string, "PID|1||||DOE^JOHN\\S\\MIDDLE^A");
}

#[test]
fn test_json_to_hl7_escaping_newlines_and_spaces() {
    let mut segment_json = BTreeMap::new();
    segment_json.insert("segment_name".to_string(), "NTE".to_string());
    segment_json.insert("1".to_string(), "1".to_string());
    segment_json.insert("2".to_string(), "L".to_string());
    segment_json.insert(
        "3".to_string(),
        "Line1\nLine2\rLine3\r\n  Spaces  \tTab".to_string(),
    );

    let json_hl7 = JsonHl7::new(vec![segment_json]);
    let hl7_string = json_hl7._convert_json_to_hl7();

    assert_eq!(hl7_string, "NTE|1|L|Line1\\X0A\\Line2\\X0D\\Line3\\X0D\\\\X0A\\\\X20\\\\X20\\Spaces\\X20\\\\X20\\\\X09\\Tab");
}
