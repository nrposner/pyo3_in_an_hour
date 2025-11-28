use pyo3::{exceptions::PyTypeError, prelude::*, types::PyModule};
use pyo3_ffi::c_str;
use crate::{employees::Employee, paper::Paper};

#[pyfunction]
pub fn dwights_algo(mut paper: Paper, employees: Vec<Bound<'_, Employee>>) -> PyResult<()> {
    let algorithms = c_str!(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/algorithms.py"
    )));

    let result = Python::attach(|py| -> PyResult<Vec<String>> {
        let algos: Py<PyAny> = PyModule::from_code(py, algorithms, c"algorithms.py", c"")?
            .getattr("dwights_ai_decision_algorithm")?
            .into();
        algos.call1(py, (paper.clone(), employees.clone()))?.extract::<Vec<String>>(py)
    });

    let result = match result {
        Ok(res) => res,
        Err(e) => return Err(PyTypeError::new_err(format!("Got unexpected exception {}", e)))
    };

    if result.is_empty() {
        paper.shred();
    } else {
        paper.fax(employees, Some(result));
    };

    Ok(())
}

#[pyfunction]
pub fn jims_algo(mut paper: Paper, employees: Vec<Bound<'_, Employee>>) -> PyResult<()> {
    let algorithms = c_str!(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/algorithms.py"
    )));

    let result = Python::attach(|py| -> PyResult<Vec<String>> {
        let algos: Py<PyAny> = PyModule::from_code(py, algorithms, c"algorithms.py", c"")?
            .getattr("jims_ai_decision_algorithm")?
            .into();
        algos.call1(py, (paper.clone(), employees.clone()))?.extract::<Vec<String>>(py)
    });

    let result = match result {
        Ok(res) => res,
        Err(e) => return Err(PyTypeError::new_err(format!("Got unexpected exception {}", e)))
    };

    // designed to do nothing
    // should we make this one a bit less boring?


    if result.is_empty() {
        paper.shred();
    } else {
        paper.fax(employees, Some(result));
    };

    Ok(())
}


// use std::ffi::CString;
// use pyo3::{exceptions::PyTypeError, prelude::*, types::{IntoPyDict, PyModule}};
// use pyo3_ffi::c_str;
// use crate::{employees::Employee, paper::{Paper, PaperSize}};
//
// fn test_algorithms() -> Result<(), String> {
//
//     let paper = Paper::new(PaperSize::new(6, 8), String::from("bla"));
//
//     let employees = [ 
//         Employee::new(1, "Michael".to_string(), "mscott@dundermifflin.com", "the best"), 
//         Employee::new(0, "Dwight".to_string(), "dschrute@dundermifflin.com", "horrible"), 
//         Employee::new(2, "Pam".to_string(), "pbeesly@dundermifflin.com", "PAM!"),
//         Employee::new(3, "Jim".to_string(), "jhalpert@dundermifflin.com", "lazy"), 
//     ];
//
//     Python::try_attach(|py| {
//         let dunder_mifflin_custom = PyModule::from_code(py, CString::new(r#"
// def dwights_ai_decision_algorithm(paper: Paper, employees: [Employee]):
//     if "DO NOT SHRED" in paper.contents:
//         # we don't shred
//         # we search for the names of employees in the paper
//         employee_names = [employee.name for employee in employees]
//         send_to = employee_names[employee_names in paper.contents]
//         # and then send to all of them
//         paper.fax(paper, send_to)
//     else:
//         # we shred 
//         paper.shred()
//
// def jims_ai_decision_algorithm(paper: Paper, employees: [Employee]):
//     if "DO NOT SHRED" in paper.contents:
//         paper.shred()
//     else:
//         paper.shred()
//         "#).unwrap().as_c_str(), 
//             CString::new("algorithms.py").unwrap().as_c_str(),
//             CString::new("dunder_mifflin_custom").unwrap().as_c_str(),
//         ).unwrap();
//
//
//         // what do we do with functions that do and don't return from python??
//         let _dwight = dunder_mifflin_custom.getattr("dwights_ai_decision_algorithm").unwrap().call1((paper, employees,)).unwrap();
//     });
//
//     Ok(())
// }
//
// fn test_pycall() -> Result<(), String> {
//     Python::attach(|py| {
//         let activators = PyModule::from_code(py, CString::new(r#"
//     def relu(x):
//         return max(0.0, x)
//
//     def leaky_relu(x, slope=0.01):
//         return x if x >= 0 else x * slope
//         "#).unwrap().as_c_str(), 
//         CString::new("activators.py").unwrap().as_c_str(), 
//         CString::new("activators").unwrap().as_c_str()).unwrap();
//
//         let relu_result: f64 = activators.getattr("relu").unwrap().call1((-1.0,)).unwrap().extract().unwrap();
//         assert_eq!(relu_result, 0.0);
//
//         let kwargs = [("slope", 0.2)].into_py_dict(py);
//         let lrelu_result: f64 = activators
//             .getattr("leaky_relu").unwrap().call((-1.0,), Some(&kwargs.unwrap())).unwrap()
//             .extract().unwrap();
//         assert_eq!(lrelu_result, -0.2);
//     });
//
//     Ok(())
// }
//
