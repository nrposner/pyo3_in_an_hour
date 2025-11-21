use pyo3::{exceptions::PyWarning, prelude::*};

use crate::employees::{Employee, EmployeeList};

/// A single sheet of paper that can be placed into the Shredder
#[pyclass]
// for some reason no longer need FromPyObject implementation
#[derive(Hash, Clone, PartialEq, Eq)]
pub struct Paper {
	  paper_size: PaperSize,
    #[pyo3(get, set)]
	  contents: String,
    // paper_type: PaperType,
}

impl Paper {
    pub fn new(
	      paper_size: PaperSize,
	      contents: String,
        // paper_type: PaperType,
    ) -> Self {
        Self {
            paper_size,
            contents,
            // paper_type,
        }
    }
    pub fn shred(&mut self) -> Scraps { 
        Scraps {}
    } 
    // pub fn recycle(&mut self) -> Option<Scraps> { 
    //     match self.paper_type { 
    //         PaperType::Bad(_) | PaperType::Okay(_) => { 
    //             Some(Scraps {}) 
    //         } 
    //         PaperType::Nice(_) | PaperType::SuperNice(_) => { 
    //             None 
    //         } 
    //     } 
    // }

    pub fn contents(&self) -> String { self.contents.clone() }
}

// the student will make this 

// judgment call: do we allow signed integers and coerce them, or do we error out?
// this is what the student is writing, they get to choose... but they can't pass all tests,
// they've gotta side with either Michael or Dwight
#[pymethods]
impl Paper {
    // option 1
    #[new]
    pub fn __init__(
        width: u32,
        height: u32,
        contents: String,
        // quality: PaperType, 
    ) -> Self {
        // Paper::new(PaperSize::new(width, height), contents, quality)
        Paper::new(PaperSize::new(width, height), contents)
    }

    // // option 2
    // #[new]
    // pub fn __init__(
    //     width: i32,
    //     height: i32,
    //     contents: String,
    //     // quality: PaperType, 
    // ) -> PyResult<Self> {
    //     if width < 0 || height < 0 {
    //         Err(PyWarning::new_err("Please stop, making negative widths"))
    //     } else {
    //         Ok(Paper::new(PaperSize::new(width.unsigned_abs(), height.unsigned_abs()), contents))
    //     }
    // }

    pub fn __repr__(&self) -> String { self.contents.clone() }

    //todo:
    // do we want to instead use a Bound<Pylist or similar?> or would we really want numpy for
    // that?

    /// adding these papers to the inbox of these employees
    /// defaults to sending to everyone in the office
    #[pyo3(signature = (office, emails_opt=None))]
    pub fn fax(&self, office: Vec<Bound<'_, Employee>>, emails_opt: Option<Vec<String>>) {
        if let Some(emails) = emails_opt { 
            // if emails are specified, just send to those employees
            for employee_handle in office {

                // we borrow the Rust struct *inside* the Python object mutably
                let mut employee_ref = employee_handle.borrow_mut();
                if emails.contains(&employee_ref.email()) { 
                    // Perform the mutation
                    employee_ref.send(self.clone());
                } 
            }
        } else {
            for employee_handle in office {
                // We borrow the Rust struct *inside* the Python object mutably
                let mut employee_ref = employee_handle.borrow_mut();
                
                // Perform the mutation
                employee_ref.send(self.clone());
            }
        }
    } 
}

enum Output { 
    Scraps, 
    Papers,
}

pub struct Scraps {}

// // nice paper should be recycled, bad and okay paper should not
// #[derive(Clone, Hash, PartialEq, Eq)]
// pub enum PaperType { 
//     Bad(usize), 
//     Okay(usize), 
//     Nice(usize), 
//     SuperNice(usize),
// }
//
//
// // the student would have to make this, but it's probably 
// // too verbose for the time being
//
// impl<'py, 'a> FromPyObject<'py, 'a> for PaperType {
//     // not quite right I'm sure
//     type Error = PyErr;
//
//     fn extract(obj: Borrowed<'py, 'a, PyAny>) -> Result<Self, Self::Error> {
//         let en: String = obj.extract()?;
//         match en.to_lowercase().as_str() {
//             "bad" => Ok(Self::Bad(0)),
//             "ok" | "okay" => Ok(Self::Okay(0)),
//             "nice" => Ok(Self::Nice(0)),
//             "supernice" => Ok(Self::SuperNice(0)),
//             _ => Err(PyValueError::new_err("Valid paper types include 'bad', 'ok', 'nice' and 'supernice'"))
//         }
//     }
// }


// gotta take into account that users might try to create negative-width pages
// or that Michael and Dwight have been arguing about tuples for the last two years, and one will never use them and the other uses them exclusively
// you need to support both
#[derive(FromPyObject, Hash, Clone, PartialEq, Eq)]
pub struct PaperSize { 
    width: u32, 
    height: u32, 
    size: (u32, u32),
}

impl PaperSize {
    pub fn new(
        width: u32, 
        height: u32
    ) -> Self {
        Self {
            width, 
            height,
            size: (width, height),
        }
    }
}


// create a function to hack the paper shredder
// for corporate espionage purposes
// obviously
pub fn hack() {}

