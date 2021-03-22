use std::ffi::CString;

use pyo3::prelude::*;

mod bindings;
use bindings::levenshtein;

pub fn distance(a: &str, b: &str) -> u32 {
    let a = CString::new(a).unwrap();
    let b = CString::new(b).unwrap();
    unsafe { levenshtein(a.as_ptr(), b.as_ptr()) }
}

#[pymodule(levin)]
fn levin(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "dist")]
    fn dist(_py: Python, a: &str, b: &str) -> u32 {
        distance(a, b)
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_by_1() {
        assert_eq!(1, distance("rahul", "raul"));
    }
    #[test]
    fn test_distance_equal() {
        assert_eq!(0, distance("rahul", "rahul"));
    }
    #[test]
    fn test_distance_not_equal() {
        assert_eq!(5, distance("rahul", "abaaa"));
    }
    #[test]
    fn test_distance_by_2() {
        assert_eq!(2, distance("23232322", "2323231"));
    }
    #[test]
    fn test_distance_not_empty() {
        assert_eq!(7, distance("", "2323231"));
    }
    #[test]
    fn test_distance_empty_1() {
        assert_eq!(5, distance("", "rahul"));
    }
    #[test]
    fn test_distance_empty() {
        assert_eq!(0, distance("", ""));
    }
}
