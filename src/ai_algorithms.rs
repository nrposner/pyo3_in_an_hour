use pyo3::{prelude::*, types::{IntoPyDict, PyModule}};
use std::ffi::CString;

use crate::{employees::{Employee, EmployeeList}, paper::{Paper, PaperSize, PaperType}};


fn test_algorithms() -> Result<(), String> {

    let paper = Paper::new(PaperType::Nice(0), PaperSize::new(6, 8), String::from("bla"));

    let employees = EmployeeList::from( 
        [ 
            Employee::new(1, "Michael", "mscott@dundermifflin.com", "the best"), 
            Employee::new(0, "Dwight", "dschrute@dundermifflin.com", "horrible"),
            Employee::new(2, "Pam", "pbeesly@dundermifflin.com", "PAM!"),
            Employee::new(3, "Jim", "jhalpert@dundermifflin.com", "lazy"),
        ]
    );

    Python::try_attach(|py| {
        let dunder_mifflin_custom = PyModule::from_code(py, CString::new(r#"
def dwights_ai_decision_algorithm(paper: Paper, employees: [Employee]):
    if "DO NOT SHRED" in paper.contents:
        # we don't shred
        # we search for the names of employees in the paper
        employee_names = [employee.name for employee in employees]
        send_to = employee_names[employee_names in paper.contents]
        # and then send to all of them
        paper.fax(paper, send_to)
    else:
        # we shred 
        paper.shred()

def jims_ai_decision_algorithm(paper: Paper, employees: [Employee]):
    if "DO NOT SHRED" in paper.contents:
        paper.shred()
    else:
        paper.shred()
        "#).unwrap().as_c_str(), 
            CString::new("algorithms.py").unwrap().as_c_str(),
            CString::new("dunder_mifflin_custom").unwrap().as_c_str(),
        ).unwrap();


        // what do we do with functions that do and don't return from python??
        let _dwight = dunder_mifflin_custom.getattr("dwights_ai_decision_algorithm").unwrap().call1((paper, employees,)).unwrap();
    });

    Ok(())
}

fn test_pycall() -> Result<(), String> {
    Python::attach(|py| {
        let activators = PyModule::from_code(py, CString::new(r#"
    def relu(x):
        return max(0.0, x)

    def leaky_relu(x, slope=0.01):
        return x if x >= 0 else x * slope
        "#).unwrap().as_c_str(), 
        CString::new("activators.py").unwrap().as_c_str(), 
        CString::new("activators").unwrap().as_c_str()).unwrap();

        let relu_result: f64 = activators.getattr("relu").unwrap().call1((-1.0,)).unwrap().extract().unwrap();
        assert_eq!(relu_result, 0.0);

        let kwargs = [("slope", 0.2)].into_py_dict(py);
        let lrelu_result: f64 = activators
            .getattr("leaky_relu").unwrap().call((-1.0,), Some(&kwargs.unwrap())).unwrap()
            .extract().unwrap();
        assert_eq!(lrelu_result, -0.2);
    });

    Ok(())
}

