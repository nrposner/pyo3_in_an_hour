mod ai_algorithms;
mod employees;
mod machine;
mod paper;

use pyo3::prelude::*;

use paper::Paper;
use employees::Employee;
use machine::search_paper;
use ai_algorithms::*;

#[pymodule]
fn dunder_mifflin(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_class::<Employee>()?;
    // m.add_class::<Paper>()?;
    // m.add_function(wrap_pyfunction!(search_paper, m)?)?;
    // m.add_function(wrap_pyfunction!(dwights_algo, m)?)?;
    // m.add_function(wrap_pyfunction!(jims_algo, m)?)?;
    Ok(())
}
