mod datetime;
mod errors;
mod escape;
mod hl7_json;
mod json_hl7;
mod segments;
mod utils;
mod validation;

#[cfg(test)]
mod tests;

use pyo3::prelude::*;

#[pymodule]
pub(crate) fn hl7conv2(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<hl7_json::Hl7Json>()?;
    m.add_class::<json_hl7::JsonHl7>()?;
    m.add_function(wrap_pyfunction!(datetime::py_parse_hl7_datetime, m)?)?;
    Ok(())
}
