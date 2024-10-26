//! # Documentation for `second_crate`
//!
//! Here's how we use an example from `first_crate`:
//!
//! ```rust
//! # use first_crate::example_to_embed;
//! example_to_embed();
//! ```
//!
//! Now, here's how we use an example from a **nested file inside `second_crate`**:
//!
//! ```rust
//! # use second_crate::nested::nested_example;
//! nested_example();
//! ```

// Declare the nested module
pub mod nested;
pub mod utils;

#[doc = docify::embed!("../first_crate/src/lib.rs", example_to_embed)]
pub struct FirstCrateExample;

#[doc = docify::embed!("src/nested/nested_file.rs", nested_example)]
pub struct NestedExample;
