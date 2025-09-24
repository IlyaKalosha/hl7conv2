use crate::json_hl7::JsonHl7;
use std::collections::BTreeMap;

fn create_test_json_data() -> Vec<BTreeMap<String, String>> {
    let mut segment1 = BTreeMap::new();
    segment1.insert("segment_name".to_string(), "MSH".to_string());
    segment1.insert("1".to_string(), "^~\\&".to_string());
    segment1.insert("2".to_string(), "ADT1".to_string());
    segment1.insert("3".to_string(), "HOSPITAL".to_string());
    segment1.insert("4".to_string(), "LAB".to_string());
    segment1.insert("5".to_string(), "HOSPITAL".to_string());
    segment1.insert("6".to_string(), "20240101120000".to_string());
    segment1.insert("7".to_string(), "SECURITY".to_string());
    segment1.insert("8.1".to_string(), "ADT".to_string());
    segment1.insert("8.2".to_string(), "A01".to_string());
    segment1.insert("8.3".to_string(), "ADT_A01".to_string());
    segment1.insert("9".to_string(), "MSG00001".to_string());
    segment1.insert("10".to_string(), "T".to_string());
    segment1.insert("11".to_string(), "2.5.1".to_string());

    let mut segment2 = BTreeMap::new();
    segment2.insert("segment_name".to_string(), "EVN".to_string());
    segment2.insert("2".to_string(), "20240101120000".to_string());

    let mut segment3 = BTreeMap::new();
    segment3.insert("segment_name".to_string(), "PID".to_string());
    segment3.insert("1".to_string(), "1".to_string());
    segment3.insert("3.1".to_string(), "PATID1234".to_string());
    segment3.insert("3.2".to_string(), "5".to_string());
    segment3.insert("3.3".to_string(), "M11".to_string());
    segment3.insert("3.4".to_string(), "ADT1".to_string());
    segment3.insert("3.5".to_string(), "MR".to_string());
    segment3.insert("3.6".to_string(), "HOSPITAL".to_string());

    vec![segment1, segment2, segment3]
}

#[test]
fn test_new() {
    let json_data = create_test_json_data();
    let json_hl7 = JsonHl7::new(json_data.clone());
    assert_eq!(json_hl7.json_data.len(), 3);
    assert_eq!(
        json_hl7.json_data[0].get("segment_name"),
        Some(&"MSH".to_string())
    );
}

#[test]
fn test_convert_segment_json_to_hl7() {
    let json_data = create_test_json_data();
    let json_hl7 = JsonHl7::new(json_data);

    let msh_segment = &json_hl7.json_data[0];
    let result = json_hl7._convert_segment_json_to_hl7(msh_segment);

    let expected = "MSH|^~\\&|ADT1|HOSPITAL|LAB|HOSPITAL|20240101120000|SECURITY|ADT^A01^ADT_A01|MSG00001|T|2.5.1";
    assert_eq!(result, expected);
}

#[test]
fn test_convert_segment_with_components() {
    let mut segment = BTreeMap::new();
    segment.insert("segment_name".to_string(), "PID".to_string());
    segment.insert("1".to_string(), "1".to_string());
    segment.insert("3.1".to_string(), "PATID1234".to_string());
    segment.insert("3.2".to_string(), "5".to_string());
    segment.insert("3.3".to_string(), "M11".to_string());

    let json_hl7 = JsonHl7::new(vec![segment.clone()]);
    let result = json_hl7._convert_segment_json_to_hl7(&segment);

    let expected = "PID|1||PATID1234^5^M11";
    assert_eq!(result, expected);
}

#[test]
fn test_convert_json_to_hl7() {
    let json_data = create_test_json_data();
    let json_hl7 = JsonHl7::new(json_data);
    let result = json_hl7._convert_json_to_hl7();

    let lines: Vec<&str> = result.split('\n').collect();
    assert_eq!(lines.len(), 3);

    assert_eq!(lines[0], "MSH|^~\\&|ADT1|HOSPITAL|LAB|HOSPITAL|20240101120000|SECURITY|ADT^A01^ADT_A01|MSG00001|T|2.5.1");
    assert_eq!(lines[1], "EVN||20240101120000");
    assert_eq!(lines[2], "PID|1||PATID1234^5^M11^ADT1^MR^HOSPITAL");
}

#[test]
fn test_convert_single_segment() {
    let mut segment = BTreeMap::new();
    segment.insert("segment_name".to_string(), "MSH".to_string());
    segment.insert("1".to_string(), "^~\\&".to_string());
    segment.insert("2".to_string(), "ADT1".to_string());
    segment.insert("3".to_string(), "HOSPITAL".to_string());

    let json_hl7 = JsonHl7::new(vec![segment]);
    let result = json_hl7._convert_json_to_hl7();

    assert_eq!(result, "MSH|^~\\&|ADT1|HOSPITAL");
}

#[test]
fn test_convert_empty_segments() {
    let json_hl7 = JsonHl7::new(vec![]);
    let result = json_hl7._convert_json_to_hl7();

    assert_eq!(result, "");
}

#[test]
fn test_convert_segment_with_gaps() {
    let mut segment = BTreeMap::new();
    segment.insert("segment_name".to_string(), "TEST".to_string());
    segment.insert("1".to_string(), "value1".to_string());
    segment.insert("3".to_string(), "value3".to_string());
    segment.insert("5".to_string(), "value5".to_string());

    let json_hl7 = JsonHl7::new(vec![segment.clone()]);
    let result = json_hl7._convert_segment_json_to_hl7(&segment);

    assert_eq!(result, "TEST|value1||value3||value5");
}

#[test]
fn test_json_example_format() {
    let mut segment1 = BTreeMap::new();
    segment1.insert("segment_name".to_string(), "MSH".to_string());
    segment1.insert("1".to_string(), "^~\\&".to_string());
    segment1.insert("2".to_string(), "ADT1".to_string());
    segment1.insert("3".to_string(), "GOOD HEALTH HOSPITAL".to_string());
    segment1.insert("4".to_string(), "GHH LAB, INC.".to_string());
    segment1.insert("5".to_string(), "GOOD HEALTH HOSPITAL".to_string());
    segment1.insert("6".to_string(), "198808181126".to_string());
    segment1.insert("7".to_string(), "SECURITY".to_string());
    segment1.insert("8.1".to_string(), "ADT".to_string());
    segment1.insert("8.2".to_string(), "A01".to_string());
    segment1.insert("8.3".to_string(), "ADT_A01".to_string());
    segment1.insert("9".to_string(), "MSG00001".to_string());
    segment1.insert("10".to_string(), "T".to_string());
    segment1.insert("11".to_string(), "2.5.1".to_string());

    let mut segment2 = BTreeMap::new();
    segment2.insert("segment_name".to_string(), "EVN".to_string());
    segment2.insert("1".to_string(), "".to_string());
    segment2.insert("2".to_string(), "198808181126".to_string());
    segment2.insert("3".to_string(), "".to_string());
    segment2.insert("4".to_string(), "".to_string());

    let mut segment3 = BTreeMap::new();
    segment3.insert("segment_name".to_string(), "PID".to_string());
    segment3.insert("1".to_string(), "1".to_string());
    segment3.insert("2".to_string(), "".to_string());
    segment3.insert("3.1".to_string(), "PATID1234".to_string());
    segment3.insert("3.2".to_string(), "5".to_string());
    segment3.insert("3.3".to_string(), "M11".to_string());
    segment3.insert("3.4".to_string(), "ADT1".to_string());
    segment3.insert("3.5".to_string(), "MR".to_string());
    segment3.insert("3.6".to_string(), "GOOD HEALTH HOSPITAL".to_string());
    segment3.insert("4".to_string(), "".to_string());
    segment3.insert("5.1".to_string(), "EVERYMAN".to_string());
    segment3.insert("5.2".to_string(), "ADAM".to_string());
    segment3.insert("5.3".to_string(), "A".to_string());
    segment3.insert("5.4".to_string(), "III".to_string());
    segment3.insert("6".to_string(), "".to_string());
    segment3.insert("7".to_string(), "19610615".to_string());
    segment3.insert("8".to_string(), "M".to_string());
    segment3.insert("9".to_string(), "".to_string());
    segment3.insert("10".to_string(), "".to_string());
    segment3.insert("11.1".to_string(), "2106-3".to_string());
    segment3.insert("12".to_string(), "".to_string());
    segment3.insert("13.1".to_string(), "2222 HOME STREET".to_string());
    segment3.insert("13.2".to_string(), "".to_string());
    segment3.insert("13.3".to_string(), "GREENSBORO".to_string());
    segment3.insert("13.4".to_string(), "NC".to_string());
    segment3.insert("13.5".to_string(), "27401-1020".to_string());

    let json_data = vec![segment1, segment2, segment3];
    let json_hl7 = JsonHl7::new(json_data);
    let result = json_hl7._convert_json_to_hl7();

    let lines: Vec<&str> = result.split('\n').collect();
    assert_eq!(lines.len(), 3);

    assert_eq!(lines[0], "MSH|^~\\&|ADT1|GOOD HEALTH HOSPITAL|GHH LAB, INC.|GOOD HEALTH HOSPITAL|198808181126|SECURITY|ADT^A01^ADT_A01|MSG00001|T|2.5.1");
    assert_eq!(lines[1], "EVN||198808181126||");
    assert_eq!(lines[2], "PID|1||PATID1234^5^M11^ADT1^MR^GOOD HEALTH HOSPITAL||EVERYMAN^ADAM^A^III||19610615|M|||2106-3||2222 HOME STREET^^GREENSBORO^NC^27401-1020");
}
