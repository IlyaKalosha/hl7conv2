use pyo3::prelude::*;

fn init_python() {
    Python::initialize();
}

#[test]
fn pymodule_registers_classes() {
    init_python();
    Python::attach(|py| {
        let module = pyo3::wrap_pymodule!(crate::hl7conv2)(py);
        let hl7_json_class = module.getattr(py, "Hl7Json").unwrap();
        let json_hl7_class = module.getattr(py, "JsonHl7").unwrap();
        assert!(!hl7_json_class.is_none(py));
        assert!(!json_hl7_class.is_none(py));
    });
}

#[test]
fn hl7_json_new_and_hl7_json_getter() {
    init_python();
    Python::attach(|py| {
        let module = pyo3::wrap_pymodule!(crate::hl7conv2)(py);
        let hl7_json_class = module.getattr(py, "Hl7Json").unwrap();
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
    });
}

#[test]
fn hl7_json_hl7_json_property() {
    init_python();
    Python::attach(|py| {
        let module = pyo3::wrap_pymodule!(crate::hl7conv2)(py);
        let hl7_json_class = module.getattr(py, "Hl7Json").unwrap();
        let hl7 = hl7_json_class
            .call1(
                py,
                ("MSH|^~\\&|A|B|C|D|20240101||ADT^A01^ADT_A01|1|T|2.5.1",),
            )
            .unwrap();
        let _json = hl7.getattr(py, "hl7_json").unwrap();
    });
}

#[test]
fn hl7_json_from_file_error_empty() {
    init_python();
    Python::attach(|py| {
        let module = pyo3::wrap_pymodule!(crate::hl7conv2)(py);
        let hl7_json_class = module.getattr(py, "Hl7Json").unwrap();
        let from_file = hl7_json_class.getattr(py, "from_file").unwrap();
        let temp = std::env::temp_dir().join("hl7conv2_empty_test.txt");
        std::fs::write(&temp, "   \n\n").unwrap();
        let result = from_file.call1(py, (temp.to_string_lossy().to_string(),));
        assert!(result.is_err());
    });
}

#[test]
fn hl7_json_from_file_success() {
    init_python();
    Python::attach(|py| {
        let module = pyo3::wrap_pymodule!(crate::hl7conv2)(py);
        let hl7_json_class = module.getattr(py, "Hl7Json").unwrap();
        let from_file = hl7_json_class.getattr(py, "from_file").unwrap();
        let temp = std::env::temp_dir().join("hl7conv2_hl7_from_file_test.txt");
        let content = "MSH|^~\\&|A|B|C|D|20240101||ADT^A01^ADT_A01|1|T|2.5.1|2.5.1\nPID|1||x";
        std::fs::write(&temp, content).unwrap();
        let instance = from_file
            .call1(py, (temp.to_string_lossy().to_string(), true, true, true))
            .unwrap();
        let _json = instance.getattr(py, "hl7_json").unwrap();
    });
}

#[test]
fn hl7_json_convert_without_escaping() {
    init_python();
    Python::attach(|py| {
        let module = pyo3::wrap_pymodule!(crate::hl7conv2)(py);
        let hl7_json_class = module.getattr(py, "Hl7Json").unwrap();
        let hl7 = hl7_json_class
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
        let result = hl7.call_method0(py, "_convert_hl7_to_json").unwrap();
        let list = result.cast_bound::<pyo3::types::PyList>(py).unwrap();
        assert_eq!(list.len(), 1);
    });
}

#[test]
fn hl7_json_convert_with_validation() {
    init_python();
    Python::attach(|py| {
        let module = pyo3::wrap_pymodule!(crate::hl7conv2)(py);
        let hl7_json_class = module.getattr(py, "Hl7Json").unwrap();
        let hl7 = hl7_json_class
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
        let _json = hl7.getattr(py, "hl7_json").unwrap();
    });
}

#[test]
fn hl7_json_validate() {
    init_python();
    Python::attach(|py| {
        let module = pyo3::wrap_pymodule!(crate::hl7conv2)(py);
        let hl7_json_class = module.getattr(py, "Hl7Json").unwrap();
        let hl7 = hl7_json_class
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
        let result = hl7.call_method1(py, "validate", (true, true));
        assert!(result.is_ok());
    });
}

#[test]
fn json_hl7_hl7_string_getter() {
    init_python();
    Python::attach(|py| {
        let module = pyo3::wrap_pymodule!(crate::hl7conv2)(py);
        let json_hl7_class = module.getattr(py, "JsonHl7").unwrap();
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
    });
}

#[test]
fn json_hl7_hl7_string_unescaped_getter() {
    init_python();
    Python::attach(|py| {
        let module = pyo3::wrap_pymodule!(crate::hl7conv2)(py);
        let json_hl7_class = module.getattr(py, "JsonHl7").unwrap();
        let json_data = vec![std::collections::BTreeMap::from([
            ("segment_name".to_string(), "OBX".to_string()),
            ("1".to_string(), "1".to_string()),
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
        let _unescaped: String = instance
            .getattr(py, "hl7_string_unescaped")
            .unwrap()
            .extract(py)
            .unwrap();
    });
}

#[test]
fn json_hl7_from_file() {
    init_python();
    Python::attach(|py| {
        let module = pyo3::wrap_pymodule!(crate::hl7conv2)(py);
        let json_hl7_class = module.getattr(py, "JsonHl7").unwrap();
        let from_file = json_hl7_class.getattr(py, "from_file").unwrap();
        let temp = std::env::temp_dir().join("hl7conv2_json_test.json");
        let content = r#"[{"segment_name":"MSH","1":"^~\\&","2":"A"}]"#;
        std::fs::write(&temp, content).unwrap();
        let instance = from_file
            .call1(py, (temp.to_string_lossy().to_string(),))
            .unwrap();
        let hl7_str: String = instance
            .getattr(py, "hl7_string")
            .unwrap()
            .extract(py)
            .unwrap();
        assert!(hl7_str.starts_with("MSH|"));
    });
}
