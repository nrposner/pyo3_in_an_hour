use pyo3::prelude::*;
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

