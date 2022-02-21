#![doc(html_logo_url="http://pngimg.com/uploads/peace_symbol/peace_symbol_PNG56.png")]
#![doc(html_root_url="https://docs.rs/slotmap/0.2.1")]
#![doc(html_playground_url="https://play.rust-lang.org/")]

//! this crate provides functionality for adding things
//! # Example
//! ```
//! use doctest_demo::sum;
//! let work_a = 4;
//! let work_b = 34;
//! let total_work = sum(work_a, work_b);
//! ```

/// Sum two arguments
/// # Example
/// ```
/// assert_eq!(crate::sum(1,1),2);
/// ```

// #[doc(hidden)]
#[cfg(test)]
mod tests {
    #[test]
    fn default_test() {
        let result = crate::sum(2,2);
        assert_eq!(result, 4);
    }
}

pub fn sum(a:i8, b:i8) -> i8 {
    a + b
}