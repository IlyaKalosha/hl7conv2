mod errors;
mod hl7_json;
mod json_hl7;
mod segments;
mod utils;

#[cfg(test)]
mod tests;

use pyo3::prelude::*;

#[pymodule]
fn hl7conv2(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<hl7_json::Hl7Json>()?;
    m.add_class::<json_hl7::JsonHl7>()?;
    Ok(())
}
