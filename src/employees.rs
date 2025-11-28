use std::collections::HashSet;

use pyo3::prelude::*;

use crate::paper::Paper;

type DunderMifflinId = u32;

#[derive(Clone)]
pub struct Employee { 
    id: DunderMifflinId,
    name: String, 
    email: String, 
    michaels_notes: String,
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
        self.inbox.insert(paper.contents());
    }
    pub fn id(&self) -> u32 { self.id }
    pub fn name(&self) -> String { self.name.clone() }
    pub fn email(&self) -> String { self.email.clone() }
    pub fn notes(&self) -> String { self.michaels_notes.clone() }
}

