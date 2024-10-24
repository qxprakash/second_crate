//! This is the documentation for `second_crate`.
//!
//! Here’s how you can use a function from the `first_crate`:
//!
//! ```rust
//! use first_crate::greet;
//!
//! fn main() {
//!     println!("{}", greet());
//! }
//! ```

// Ensure `first_crate` is available as a dependency for documentation purposes
pub fn example_usage() {
    println!("{}", first_crate::greet());
}
