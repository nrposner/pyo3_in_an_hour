mod paper;
mod employees;
mod ai_algorithms;

use pyo3::prelude::*;

use paper::Paper;
use employees::Employee;

#[pymodule]
fn dunder_mifflin(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Employee>()?;
    m.add_class::<Paper>()?;
    Ok(())
}
