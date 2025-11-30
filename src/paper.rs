use pyo3::{prelude::*, exceptions::PyValueError};

use crate::employees::Employee;

/// A single sheet of paper that can be placed into the Fax-Machine/Shredder
#[pyclass]
#[derive(Clone)]
pub struct Paper {
	  paper_size: PaperSize,
    #[pyo3(get)]
	  contents: String,
}

impl Paper {
    pub fn new(
	      paper_size: PaperSize,
	      contents: String,
    ) -> Self {
        Self {
            paper_size,
            contents,
        }
    }
    /// 'Shred' the Paper by turning it into a 0x0, contentless object
    pub fn shred(&mut self) { 
        self.paper_size = PaperSize::new(0, 0);
        self.contents = "".to_string();
    } 

    pub fn contents(&self) -> String { self.contents.clone() }
}

// judgment call: do we allow signed integers and coerce them, or do we error out?
// this is what the student is writing, they get to choose... but they can't pass all tests,
// they've gotta side with either Michael or Dwight
#[pymethods]
impl Paper {
    // #[new]
    // pub fn __init__(
    //     width: i32,
    //     height: i32,
    //     contents: String,
    // ) -> Self {
    //     Paper::new(
    //         PaperSize::new(
    //             width.unsigned_abs(), 
    //             height.unsigned_abs()
    //         ), 
    //         contents
    //     )
    // }
    //option 2, with the warning
    #[new]
    pub fn __init__(
        width: i32,
        height: i32,
        contents: String,
    ) -> PyResult<Self> {
        if width < 0 || height < 0 {
            Err(PyValueError::new_err("Please stop, making negative widths"))
        } else {
            Ok(Paper::new(PaperSize::new(width.unsigned_abs(), height.unsigned_abs()), contents))
        }
    }

    pub fn __repr__(&self) -> String { self.contents.clone() }

    /// adding these papers to the inbox of these employees
    /// defaults to sending to everyone in the office
    #[pyo3(signature = (employees, emails_opt=None))]
    pub fn fax(&self, employees: Vec<Bound<'_, Employee>>, emails_opt: Option<Vec<String>>) {
        if let Some(emails) = emails_opt { 
            // if emails are specified, just send to those employees
            for employee_handle in employees {

                // we borrow the Rust struct *inside* the Python object mutably
                let mut employee_ref = employee_handle.borrow_mut();
                if emails.contains(&employee_ref.email()) { 
                    // Perform the mutation
                    employee_ref.send(self.clone());
                } 
            }
        } else {
            for employee_handle in employees {
                // We borrow the Rust struct *inside* the Python object mutably
                let mut employee_ref = employee_handle.borrow_mut();
                
                // Perform the mutation
                employee_ref.send(self.clone());
            }
        }
    } 
}


// gotta take into account that users might try to create negative-width pages
// or that Michael and Dwight have been arguing about tuples for the last two years, and one will never use them and the other uses them exclusively
// you need to support both
#[derive(Hash, Clone, PartialEq, Eq)]
pub struct PaperSize { 
    width: u32, 
    height: u32, 
}

impl PaperSize {
    pub fn new(
        width: u32, 
        height: u32
    ) -> Self {
        Self {
            width, 
            height,
        }
    }
}

