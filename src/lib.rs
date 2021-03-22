use std::ffi::CString;

use pyo3::prelude::*;

mod bindings;
use crate::bindings::levenshtein;

pub fn levi_distance(a: &str, b: &str) -> u32 {
    let a = CString::new(a).unwrap();
    let b = CString::new(b).unwrap();
    let distance =  unsafe {
        levenshtein(a.as_ptr(), b.as_ptr())
    };
    distance
}


#[pymodule(levin)]
fn levin(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "dist")]
    fn dist(_py: Python, a: &str, b: &str) -> PyResult<u32> {
      Ok(levi_distance(a, b))
    }
Ok(())
}
