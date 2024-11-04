//! # Documentation for `second_crate`
//!
//! Here's how we use an example from `first_crate`:
//!
//! ```rust
//! # use first_crate::example_to_embed;
//! example_to_embed();
//! ```


// Declare the nested module
pub mod nested;

#[doc = docif::embed!("src/nested/nested_file.rs", nested_example)]
pub struct SomeItem;


