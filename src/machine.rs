use pyo3::prelude::*;
use crate::paper::Paper;

/// Search the contents of the paper for key words
#[pyfunction(signature=(paper, keys, any=false))]
pub fn search_paper(paper: Paper, keys: Vec<String>, any: bool) -> Vec<bool> {
    let contents = paper.contents();
    let words: Vec<&str> = contents.split_whitespace().collect();

    let bools: Vec<bool> = keys.iter().map(|key| {
        words.contains(&key.as_str())
    }).collect();

    if !any {
        bools
    } else {
        vec![bools.iter().any(|b| *b)]
    }
}


