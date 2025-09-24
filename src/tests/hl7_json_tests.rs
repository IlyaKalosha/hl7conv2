use crate::hl7_json::Hl7Json;
use std::collections::BTreeMap;

fn create_test_hl7_json() -> Hl7Json {
    let hl7_string = "MSH|^~\\&|ADT1|HOSPITAL|LAB|HOSPITAL|20240101120000|SECURITY|ADT^A01^ADT_A01|MSG00001|T|2.5.1\nEVN||20240101120000||\nPID|1||PATID1234^5^M11^ADT1^MR^HOSPITAL||EVERYMAN^ADAM^A^III||19610615|M||2106-3|2222 HOME STREET^^GREENSBORO^NC^27401-1020".to_string();
    Hl7Json::new(hl7_string)
}

#[test]
fn test_new() {
    let hl7_string = "MSH|^~\\&|ADT1|HOSPITAL|LAB|HOSPITAL|20240101120000|SECURITY|ADT^A01^ADT_A01|MSG00001|T|2.5.1".to_string();
    let hl7_json = Hl7Json::new(hl7_string.clone());
    assert_eq!(hl7_json.hl7_string, hl7_string);
}

#[test]
fn test_split_hl7_seg_to_json() {
    let hl7_json = create_test_hl7_json();
    let result = hl7_json._split_hl7_seg_to_json("MSH|^~\\&|ADT1|HOSPITAL|LAB|HOSPITAL|20240101120000|SECURITY|ADT^A01^ADT_A01|MSG00001|T|2.5.1");

    let mut expected = BTreeMap::new();
    expected.insert("segment_name".to_string(), "MSH".to_string());
    expected.insert("1".to_string(), "^~\\&".to_string());
    expected.insert("2".to_string(), "ADT1".to_string());
    expected.insert("3".to_string(), "HOSPITAL".to_string());
    expected.insert("4".to_string(), "LAB".to_string());
    expected.insert("5".to_string(), "HOSPITAL".to_string());
    expected.insert("6".to_string(), "20240101120000".to_string());
    expected.insert("7".to_string(), "SECURITY".to_string());
    expected.insert("8.1".to_string(), "ADT".to_string());
    expected.insert("8.2".to_string(), "A01".to_string());
    expected.insert("8.3".to_string(), "ADT_A01".to_string());
    expected.insert("9".to_string(), "MSG00001".to_string());
    expected.insert("10".to_string(), "T".to_string());
    expected.insert("11".to_string(), "2.5.1".to_string());

    assert_eq!(result, expected);
}

#[test]
fn test_convert_hl7_to_json() {
    let hl7_json = create_test_hl7_json();
    let result = hl7_json._convert_hl7_to_json();

    assert_eq!(result.len(), 3);
    let msh_segment = &result[0];
    assert_eq!(msh_segment.get("segment_name"), Some(&"MSH".to_string()));
    assert_eq!(msh_segment.get("2"), Some(&"ADT1".to_string()));

    let evn_segment = &result[1];
    assert_eq!(evn_segment.get("segment_name"), Some(&"EVN".to_string()));
    let pid_segment = &result[2];
    assert_eq!(pid_segment.get("segment_name"), Some(&"PID".to_string()));
    assert_eq!(pid_segment.get("3.1"), Some(&"PATID1234".to_string()));
}

#[test]
fn test_convert_hl7_to_json_empty() {
    let hl7_json = Hl7Json::new("".to_string());
    let result = hl7_json._convert_hl7_to_json();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].get("segment_name"), Some(&"".to_string()));
}

#[test]
fn test_convert_hl7_to_json_single_segment() {
    let hl7_json = Hl7Json::new("MSH|^~\\&|ADT1|HOSPITAL".to_string());
    let result = hl7_json._convert_hl7_to_json();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].get("segment_name"), Some(&"MSH".to_string()));
    assert_eq!(result[0].get("2"), Some(&"ADT1".to_string()));
}
