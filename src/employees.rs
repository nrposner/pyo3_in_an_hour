use std::collections::HashSet;

use pyo3::prelude::*;

use crate::paper::Paper;

pub type EmployeeList = Vec<Employee>;

type DunderMifflinId = u32;

#[derive(Clone)]
#[pyclass]
pub struct Employee { 
    id: DunderMifflinId,
    name: String, 
    email: String, 
    michaels_notes: String,
    // inbox: HashSet<Paper>
    inbox: HashSet<String>
}

impl Employee { 
    pub fn new(id: DunderMifflinId, name: String, email: &str, michaels_notes: &str) -> Self { 
        Self { 
            id,
            name: name.to_string(), 
            email: email.to_string(), 
            michaels_notes: michaels_notes.to_string(),
            inbox: HashSet::new(),
        } 
    }
    pub fn send(&mut self, paper: Paper) {
        // no need for the bool here
        self.inbox.insert(paper.contents());
    }
    pub fn email(&self) -> String {
        self.email.clone()
    }
}


#[pymethods]
impl Employee {
    #[new]
    // pub fn __init__(id: u32, name: &str, email: &str, michaels_notes: &str) -> Self { 
    //     Employee::new(id, name, email, michaels_notes) 
    // }
    pub fn __init__(id: u32, name: String, email: String, michaels_notes: String) -> Self { 
        Employee::new(id, name, &email, &michaels_notes) 
    }
    pub fn inbox(&self) -> HashSet<String> {
        self.inbox.clone()
    }
}

