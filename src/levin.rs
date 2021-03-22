mod bindings;

use std::ffi::CString;
use crate::bindings::levenshtein;


pub fn levi_distance(a: &str, b: &str) -> u32 {
    let a = CString::new(a).unwrap();
    let b = CString::new(b).unwrap();
    let distance =  unsafe {
        levenshtein(a.as_ptr(), b.as_ptr())
    };
    distance
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_levi_dist() {
        assert_eq!(levi_distance("hello", "helo"), 1);
    }
}
