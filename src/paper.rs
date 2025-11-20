use pyo3::{exceptions::PyValueError, prelude::*};

use crate::employees::EmployeeList;

/// A single sheet of paper that can be placed into the Shredder
#[pyclass]
// for some reason no longer need FromPyObject implementation
#[derive(Hash, Clone, PartialEq, Eq)]
pub struct Paper {
    paper_type: PaperType,
	  paper_size: PaperSize,
    #[pyo3(get, set)]
	  contents: String,
}

impl Paper {
    pub fn new(
        paper_type: PaperType,
	      paper_size: PaperSize,
	      contents: String,
    ) -> Self {
        Self {
            paper_type,
            paper_size,
            contents,
        }
    }
    pub fn shred(&mut self) -> Scraps { 
        Scraps {}
    } 
    pub fn recycle(&mut self) -> Option<Scraps> { 
        match self.paper_type { 
            PaperType::Bad(_) | PaperType::Okay(_) => { 
                Some(Scraps {}) 
            } 
            PaperType::Nice(_) | PaperType::SuperNice(_) => { 
                None 
            } 
        } 
    }

    pub fn contents(&self) -> String { self.contents.clone() }
}

// the student will make this 

#[pymethods]
impl Paper {
    // or should this be init, not new?
    #[new]
    pub fn __init__(
        quality: PaperType, 
        width: u32,
        height: u32,
        contents: String,
    ) -> Self {
        Paper::new(quality, PaperSize::new(width, height), contents)
    }

    pub fn __repr__(&self) -> String { self.contents.clone() }


    /// adding these papers to the inbox of these employees
    /// defaults to sending to everyone in the office
    #[pyo3(signature = (office, emails_opt=None))]
    pub fn fax(&self, office: EmployeeList, emails_opt: Option<Vec<String>>) -> EmployeeList { 
        // let employee_emails = office.iter().map(|employee| {
        //     employee.email()
        // }).collect();

        if let Some(emails) = emails_opt { 
            // if emails are specified, just send to those employees

            // emails.iter.for_each(|address| {
            //
            //     if employee_emails.contains(address) {
            //         employee.send(paper.clone()) 
            //
            //     }
            // })
            // this way would be simpler and excludes the above error case
            // but we want to have the error case :P
            office.clone().iter_mut().map(|employee| { 
                if emails.contains(&employee.email()) { 
                    employee.send(self.clone()) 
                } 
                employee.clone()
            }).collect()
        } else {
            // else just send to everyone in the office
            office.clone().iter_mut().map(|employee| { 
                employee.send(self.clone());
                employee.clone()
            }).collect() 
        }
    } 
}

enum Output { 
    Scraps, 
    Papers,
}

pub struct Scraps {}

// nice paper should be recycled, bad and okay paper should not
#[derive(Clone, Hash, PartialEq, Eq)]
pub enum PaperType { 
    Bad(usize), 
    Okay(usize), 
    Nice(usize), 
    SuperNice(usize),
}


// the student would have to make this, but it's probably 
// too verbose for the time being

impl<'py, 'a> FromPyObject<'py, 'a> for PaperType {
    // not quite right I'm sure
    type Error = PyErr;

    fn extract(obj: Borrowed<'py, 'a, PyAny>) -> Result<Self, Self::Error> {
        let en: String = obj.extract()?;
        match en.to_lowercase().as_str() {
            "bad" => Ok(Self::Bad(0)),
            "ok" | "okay" => Ok(Self::Okay(0)),
            "nice" => Ok(Self::Nice(0)),
            "supernice" => Ok(Self::SuperNice(0)),
            _ => Err(PyValueError::new_err("Valid paper types include 'bad', 'ok', 'nice' and 'supernice'"))
        }
    }
}


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

