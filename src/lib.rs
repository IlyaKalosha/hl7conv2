mod hl7_json;
mod json_hl7;
mod utils;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn hl7conv2(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<hl7_json::Hl7Json>()?;
    // m.add_function(wrap_pyfunction!(hl7_json::hl7_to_dict, m)?)?;
    Ok(())
}
