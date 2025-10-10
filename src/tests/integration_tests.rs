use crate::json_hl7::JsonHl7;
use crate::segments::Hl7Segment;
use crate::utils;
use std::collections::BTreeMap;

#[test]
fn test_full_hl7_to_json_complex_adt() {
    let hl7_content = include_str!("test_data/complex_adt_a01.hl7");
    let hl7_string = utils::replace_eof(hl7_content.to_string());

    let lines: Vec<&str> = hl7_string.lines().collect();
    let segments: Vec<Hl7Segment> = lines
        .iter()
        .map(|line| Hl7Segment::from_string(line, None))
        .collect();

    let json_result: Vec<BTreeMap<String, String>> =
        segments.iter().map(|seg| seg.to_json()).collect();

    assert!(!json_result.is_empty());
    assert_eq!(json_result[0].get("segment_name").unwrap(), "MSH");
    assert_eq!(json_result[1].get("segment_name").unwrap(), "EVN");
    assert_eq!(json_result[2].get("segment_name").unwrap(), "PID");

    let msh = &json_result[0];
    assert_eq!(msh.get("1").unwrap(), "^~\\&");
    assert_eq!(msh.get("2").unwrap(), "ADT1");
    assert_eq!(msh.get("8.1").unwrap(), "ADT");
    assert_eq!(msh.get("8.2").unwrap(), "A01");

    let pid = &json_result[2];
    assert_eq!(pid.get("3[0].1").unwrap(), "PATID1234");
    assert_eq!(pid.get("3[0].2").unwrap(), "5");
    assert_eq!(pid.get("3[1].1").unwrap(), "123456789");
    assert_eq!(pid.get("3[2].1").unwrap(), "987654321");

    assert_eq!(pid.get("5.1").unwrap(), "EVERYMAN");
    assert_eq!(pid.get("5.2").unwrap(), "ADAM");
    assert_eq!(pid.get("5.3").unwrap(), "A");
    assert_eq!(pid.get("5.4").unwrap(), "III");

    assert_eq!(pid.get("11[0].1").unwrap(), "2222 HOME STREET");
    assert_eq!(pid.get("11[0].3").unwrap(), "GREENSBORO");
    assert_eq!(pid.get("11[0].4").unwrap(), "NC");
    assert_eq!(pid.get("11[1].1").unwrap(), "3333 WORK STREET");
}

#[test]
fn test_full_hl7_to_json_complex_oru() {
    let hl7_content = include_str!("test_data/complex_oru_r01.hl7");
    let hl7_string = utils::replace_eof(hl7_content.to_string());

    let lines: Vec<&str> = hl7_string.lines().collect();
    let segments: Vec<Hl7Segment> = lines
        .iter()
        .map(|line| Hl7Segment::from_string(line, None))
        .collect();

    let json_result: Vec<BTreeMap<String, String>> =
        segments.iter().map(|seg| seg.to_json()).collect();

    assert!(!json_result.is_empty());
    assert_eq!(json_result[0].get("segment_name").unwrap(), "MSH");

    let obx_segments: Vec<&BTreeMap<String, String>> = json_result
        .iter()
        .filter(|seg| seg.get("segment_name").unwrap() == "OBX")
        .collect();

    assert_eq!(obx_segments.len(), 8);

    assert_eq!(obx_segments[0].get("3.1").unwrap(), "GLU");
    assert_eq!(obx_segments[0].get("3.2").unwrap(), "Glucose Lvl");
    assert_eq!(obx_segments[0].get("5").unwrap(), "mg/dL");
    assert_eq!(obx_segments[0].get("6.1").unwrap(), "65-99");

    assert_eq!(obx_segments[1].get("3.1").unwrap(), "NA");
    assert_eq!(obx_segments[1].get("3.2").unwrap(), "Sodium");

    let nte_segments: Vec<&BTreeMap<String, String>> = json_result
        .iter()
        .filter(|seg| seg.get("segment_name").unwrap() == "NTE")
        .collect();

    assert_eq!(nte_segments.len(), 2);
    assert_eq!(
        nte_segments[0].get("3").unwrap(),
        "Patient was fasting for 12 hours prior to test"
    );
}

#[test]
fn test_full_hl7_to_json_lipid_panel() {
    let hl7_content = include_str!("test_data/lipid_panel_oru.hl7");
    let hl7_string = utils::replace_eof(hl7_content.to_string());

    let lines: Vec<&str> = hl7_string.lines().collect();
    let segments: Vec<Hl7Segment> = lines
        .iter()
        .map(|line| Hl7Segment::from_string(line, None))
        .collect();

    let json_result: Vec<BTreeMap<String, String>> =
        segments.iter().map(|seg| seg.to_json()).collect();

    assert!(!json_result.is_empty());

    let pid = &json_result[1];
    assert_eq!(pid.get("segment_name").unwrap(), "PID");
    assert_eq!(pid.get("5.1").unwrap(), "LASTNAME");
    assert_eq!(pid.get("5.2").unwrap(), "FIRSTNAME");

    let obx_segments: Vec<&BTreeMap<String, String>> = json_result
        .iter()
        .filter(|seg| seg.get("segment_name").unwrap() == "OBX")
        .collect();

    assert_eq!(obx_segments.len(), 4);

    assert_eq!(obx_segments[0].get("3.1").unwrap(), "13457-7");
    assert_eq!(obx_segments[0].get("3.2").unwrap(), "LDL (CALCULATED)");
    assert_eq!(obx_segments[0].get("5").unwrap(), "49.000");

    assert_eq!(obx_segments[3].get("3.1").unwrap(), "2571-8");
    assert_eq!(obx_segments[3].get("3.2").unwrap(), "TRIGLYCERIDES");
    assert_eq!(obx_segments[3].get("5").unwrap(), "324.000");
}

#[test]
fn test_full_hl7_to_json_edge_case_repetitions() {
    let hl7_content = include_str!("test_data/edge_case_repetitions.hl7");
    let hl7_string = utils::replace_eof(hl7_content.to_string());

    let lines: Vec<&str> = hl7_string.lines().collect();
    let segments: Vec<Hl7Segment> = lines
        .iter()
        .map(|line| Hl7Segment::from_string(line, None))
        .collect();

    let json_result: Vec<BTreeMap<String, String>> =
        segments.iter().map(|seg| seg.to_json()).collect();

    assert!(!json_result.is_empty());

    let pid = json_result
        .iter()
        .find(|s| s.get("segment_name").map(|n| n.as_str()) == Some("PID"));
    assert!(pid.is_some(), "PID segment not found");
    let pid = pid.unwrap();

    assert_eq!(
        pid.get("3[0]")
            .expect("Field 3[0] not found in PID segment"),
        "12345"
    );
    assert_eq!(
        pid.get("3[1]")
            .expect("Field 3[1] not found in PID segment"),
        "67890"
    );
    assert_eq!(
        pid.get("3[2]")
            .expect("Field 3[2] not found in PID segment"),
        "ABCDE"
    );

    assert_eq!(pid.get("5.1").unwrap(), "DOE");
    assert_eq!(pid.get("5.2").unwrap(), "JOHN");
    assert_eq!(pid.get("5.3").unwrap(), "MIDDLE");
    assert_eq!(pid.get("5.4").unwrap(), "JR");
    assert_eq!(pid.get("5.5").unwrap(), "DR");

    assert_eq!(
        pid.get("11[0]").expect("Field 11[0] not found"),
        "123 MAIN ST"
    );
    assert_eq!(pid.get("11[1]").expect("Field 11[1] not found"), "APT 4B");
    assert_eq!(
        pid.get("11[2].1").expect("Field 11[2].1 not found"),
        "BUILDING C"
    );
    assert_eq!(pid.get("11[2].3").expect("Field 11[2].3 not found"), "CITY");
    assert_eq!(pid.get("11[2].4").expect("Field 11[2].4 not found"), "ST");

    let obx = json_result.iter().find(|s| {
        s.get("segment_name").map(|n| n.as_str()) == Some("OBX")
            && s.get("1").map(|n| n.as_str()) == Some("1")
    });
    assert!(obx.is_some(), "OBX segment with ID 1 not found");
    let obx = obx.unwrap();
    assert_eq!(obx.get("5[0]").unwrap(), "Value1");
    assert_eq!(obx.get("5[1]").unwrap(), "Value2");
    assert_eq!(obx.get("5[2]").unwrap(), "Value3");
}

#[test]
fn test_json_to_hl7_roundtrip_complex() {
    let original_hl7 = include_str!("test_data/lipid_panel_oru.hl7");
    let hl7_string = utils::replace_eof(original_hl7.to_string());

    let lines: Vec<&str> = hl7_string.lines().collect();
    let segments: Vec<Hl7Segment> = lines
        .iter()
        .map(|line| Hl7Segment::from_string(line, None))
        .collect();

    let json_result: Vec<BTreeMap<String, String>> =
        segments.iter().map(|seg| seg.to_json()).collect();

    let json_hl7 = JsonHl7::new(json_result);
    let converted_hl7 = json_hl7._convert_json_to_hl7();

    let converted_lines: Vec<&str> = converted_hl7.lines().collect();
    let roundtrip_segments: Vec<Hl7Segment> = converted_lines
        .iter()
        .map(|line| Hl7Segment::from_string(line, None))
        .collect();

    assert_eq!(segments.len(), roundtrip_segments.len());

    for (i, (orig, conv)) in segments.iter().zip(roundtrip_segments.iter()).enumerate() {
        let orig_json = orig.to_json();
        let conv_json = conv.to_json();

        assert_eq!(
            orig_json.get("segment_name"),
            conv_json.get("segment_name"),
            "Segment {} has different names",
            i
        );

        for (key, value) in &orig_json {
            if key == "segment_name" || value.is_empty() {
                continue;
            }
            assert!(
                conv_json.contains_key(key),
                "Segment {} missing key: {}",
                i,
                key
            );
            assert!(
                !conv_json.get(key).unwrap().is_empty(),
                "Segment {} has empty value for key {}, expected: {}",
                i,
                key,
                value
            );
        }
    }
}

#[test]
fn test_json_to_hl7_roundtrip_edge_cases() {
    let original_hl7 = include_str!("test_data/edge_case_repetitions.hl7");
    let hl7_string = utils::replace_eof(original_hl7.to_string());

    let lines: Vec<&str> = hl7_string.lines().collect();
    let segments: Vec<Hl7Segment> = lines
        .iter()
        .map(|line| Hl7Segment::from_string(line, None))
        .collect();

    let json_result: Vec<BTreeMap<String, String>> =
        segments.iter().map(|seg| seg.to_json()).collect();

    let json_hl7 = JsonHl7::new(json_result);
    let converted_hl7 = json_hl7._convert_json_to_hl7();

    let converted_lines: Vec<&str> = converted_hl7.lines().collect();
    let roundtrip_segments: Vec<Hl7Segment> = converted_lines
        .iter()
        .map(|line| Hl7Segment::from_string(line, None))
        .collect();

    assert_eq!(segments.len(), roundtrip_segments.len());

    for (i, (orig, conv)) in segments.iter().zip(roundtrip_segments.iter()).enumerate() {
        let orig_json = orig.to_json();
        let conv_json = conv.to_json();

        assert_eq!(
            orig_json.get("segment_name"),
            conv_json.get("segment_name"),
            "Segment {} has different names",
            i
        );

        for (key, value) in &orig_json {
            if key == "segment_name" || value.is_empty() {
                continue;
            }
            assert!(
                conv_json.contains_key(key),
                "Segment {} missing key: {}",
                i,
                key
            );
            assert!(
                !conv_json.get(key).unwrap().is_empty(),
                "Segment {} has empty value for key {}, expected: {}",
                i,
                key,
                value
            );
        }
    }
}

#[test]
fn test_json_to_hl7_from_existing_example() {
    let original_hl7 = include_str!("../../examples/hl7_example.txt");
    let hl7_string = utils::replace_eof(original_hl7.to_string());

    let lines: Vec<&str> = hl7_string.lines().collect();
    let segments: Vec<Hl7Segment> = lines
        .iter()
        .map(|line| Hl7Segment::from_string(line, None))
        .collect();

    let json_result: Vec<BTreeMap<String, String>> =
        segments.iter().map(|seg| seg.to_json()).collect();

    let json_hl7 = JsonHl7::new(json_result);
    let converted_hl7 = json_hl7._convert_json_to_hl7();

    let converted_lines: Vec<&str> = converted_hl7.lines().collect();
    let roundtrip_segments: Vec<Hl7Segment> = converted_lines
        .iter()
        .map(|line| Hl7Segment::from_string(line, None))
        .collect();

    assert_eq!(segments.len(), roundtrip_segments.len());

    for (i, (orig, conv)) in segments.iter().zip(roundtrip_segments.iter()).enumerate() {
        let orig_json = orig.to_json();
        let conv_json = conv.to_json();

        assert_eq!(
            orig_json.get("segment_name"),
            conv_json.get("segment_name"),
            "Segment {} has different names",
            i
        );

        for (key, value) in &orig_json {
            if key == "segment_name" || value.is_empty() {
                continue;
            }
            assert!(
                conv_json.contains_key(key),
                "Segment {} missing key: {}",
                i,
                key
            );
            assert!(
                !conv_json.get(key).unwrap().is_empty(),
                "Segment {} has empty value for key {}, expected: {}",
                i,
                key,
                value
            );
        }
    }
}
