use pyo3::prelude::*;

fn init_python() {
    Python::initialize();
}

#[test]
fn pyo3_api_tests() {
    init_python();
    Python::attach(|py| {
        let module = pyo3::wrap_pymodule!(crate::hl7conv2)(py);
        let hl7_json_class = module.getattr(py, "Hl7Json").unwrap();
        let json_hl7_class = module.getattr(py, "JsonHl7").unwrap();

        assert!(!hl7_json_class.is_none(py));
        assert!(!json_hl7_class.is_none(py));

        let hl7 = hl7_json_class
            .call1(
                py,
                ("MSH|^~\\&|A|B|C|D|20240101||ADT^A01^ADT_A01|1|T|2.5.1",),
            )
            .unwrap();
        let result = hl7.call_method0(py, "_convert_hl7_to_json").unwrap();
        let list = result.cast_bound::<pyo3::types::PyList>(py).unwrap();
        assert_eq!(list.len(), 1);
        let seg = list.get_item(0).unwrap();
        let seg_dict = seg.cast::<pyo3::types::PyDict>().unwrap();
        let name = seg_dict.get_item("segment_name").unwrap().unwrap();
        let name_str: String = name.extract().unwrap();
        assert_eq!(name_str, "MSH");

        let _json = hl7.getattr(py, "hl7_json").unwrap();

        let from_file = hl7_json_class.getattr(py, "from_file").unwrap();
        let temp_empty = std::env::temp_dir().join("hl7conv2_empty_test.txt");
        std::fs::write(&temp_empty, "   \n\n").unwrap();
        let result_empty = from_file.call1(py, (temp_empty.to_string_lossy().to_string(),));
        assert!(result_empty.is_err());

        let temp_ok = std::env::temp_dir().join("hl7conv2_hl7_from_file_test.txt");
        let content_ok = "MSH|^~\\&|A|B|C|D|20240101||ADT^A01^ADT_A01|1|T|2.5.1|2.5.1\nPID|1||x";
        std::fs::write(&temp_ok, content_ok).unwrap();
        let instance_from_file = from_file
            .call1(
                py,
                (temp_ok.to_string_lossy().to_string(), true, true, true),
            )
            .unwrap();
        let _ = instance_from_file.getattr(py, "hl7_json").unwrap();

        let hl7_no_escape = hl7_json_class
            .call1(
                py,
                (
                    "MSH|^~\\&|A|B|C|D|20240101||ADT^A01^ADT_A01|1|T|2.5.1|2.5.1",
                    false,
                    false,
                    false,
                ),
            )
            .unwrap();
        let result_no_escape = hl7_no_escape
            .call_method0(py, "_convert_hl7_to_json")
            .unwrap();
        let list_no_escape = result_no_escape
            .cast_bound::<pyo3::types::PyList>(py)
            .unwrap();
        assert_eq!(list_no_escape.len(), 1);

        let hl7_with_validation = hl7_json_class
            .call1(
                py,
                (
                    "MSH|^~\\&|A|B|C|D|20240101||ADT^A01^ADT_A01|1|T|2.5.1|2.5.1\nPID|1||x",
                    true,
                    true,
                    true,
                ),
            )
            .unwrap();
        let _ = hl7_with_validation.getattr(py, "hl7_json").unwrap();

        let hl7_validate = hl7_json_class
            .call1(
                py,
                (
                    "MSH|^~\\&|A|B|C|D|20240101||ADT^A01^ADT_A01|1|T|2.5.1|2.5.1\nPID|1||x",
                    true,
                    true,
                    true,
                ),
            )
            .unwrap();
        let result_validate = hl7_validate.call_method1(py, "validate", (true, true));
        assert!(result_validate.is_ok());

        let json_data = vec![std::collections::BTreeMap::from([
            ("segment_name".to_string(), "MSH".to_string()),
            ("1".to_string(), "^~\\&".to_string()),
            ("2".to_string(), "A".to_string()),
        ])];
        let mut dicts = Vec::new();
        for m in json_data {
            let dict = pyo3::types::PyDict::new(py);
            for (k, v) in m {
                dict.set_item(k, v).unwrap();
            }
            dicts.push(dict);
        }
        let json_list = pyo3::types::PyList::new(py, dicts).unwrap();
        let instance = json_hl7_class.call1(py, (json_list,)).unwrap();
        let hl7_str: String = instance
            .getattr(py, "hl7_string")
            .unwrap()
            .extract(py)
            .unwrap();
        assert!(hl7_str.starts_with("MSH|"));

        let json_data_unescaped = vec![std::collections::BTreeMap::from([
            ("segment_name".to_string(), "OBX".to_string()),
            ("1".to_string(), "1".to_string()),
        ])];
        let mut dicts_unescaped = Vec::new();
        for m in json_data_unescaped {
            let dict = pyo3::types::PyDict::new(py);
            for (k, v) in m {
                dict.set_item(k, v).unwrap();
            }
            dicts_unescaped.push(dict);
        }
        let json_list_unescaped = pyo3::types::PyList::new(py, dicts_unescaped).unwrap();
        let instance_unescaped = json_hl7_class.call1(py, (json_list_unescaped,)).unwrap();
        let _: String = instance_unescaped
            .getattr(py, "hl7_string_unescaped")
            .unwrap()
            .extract(py)
            .unwrap();

        let from_file_json = json_hl7_class.getattr(py, "from_file").unwrap();
        let temp_json = std::env::temp_dir().join("hl7conv2_json_test.json");
        let content_json = r#"[{"segment_name":"MSH","1":"^~\\&","2":"A"}]"#;
        std::fs::write(&temp_json, content_json).unwrap();
        let instance_json = from_file_json
            .call1(py, (temp_json.to_string_lossy().to_string(),))
            .unwrap();
        let hl7_str_json: String = instance_json
            .getattr(py, "hl7_string")
            .unwrap()
            .extract(py)
            .unwrap();
        assert!(hl7_str_json.starts_with("MSH|"));
    });
}
