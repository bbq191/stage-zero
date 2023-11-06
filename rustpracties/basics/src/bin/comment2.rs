#![allow(unused)]
fn main() {
    // in src/compute.rs

    /// # Panics
    ///
    /// The function panics if the second argument is zero.
    ///
    /// ```rust,should_panic
    /// // panics on division by zero
    doc_comments::compute::div(10, 0);
    /// ```
    pub fn div(a: i32, b: i32) -> i32 {
        if b == 0 {
            panic!("Divide-by-zero error");
        }

        a / b
    }
}
