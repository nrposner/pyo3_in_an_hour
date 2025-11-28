use pyo3::{exceptions::PyTypeError, prelude::*, types::PyModule};
use pyo3_ffi::c_str;
use crate::{employees::Employee, paper::Paper};

// #[pyfunction]
// pub fn dwights_algo(mut paper: Paper, employees: Vec<Bound<'_, Employee>>) -> PyResult<()> { Ok(()) }
//
// #[pyfunction]
// pub fn jims_algo(mut paper: Paper, employees: Vec<Bound<'_, Employee>>) -> PyResult<()> { Ok(()) }

