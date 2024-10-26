//! # Utilities Documentation
//!
//! This file contains utility functions and examples.
//!
//! Here's how we use an example from the **nested file inside `second_crate`**:
//!
//! ```rust
//! # use second_crate::nested::nested_example;
//! nested_example();
//! ```
//!
//! Now, let's also embed an example from the **first_crate**:
//!
//! ```rust
//! # use first_crate::example_to_embed;
//! example_to_embed();
//! ```

// Embed the example from `nested_file.rs`
#[doc = docify::embed!("src/nested/nested_file.rs", nested_example)]
pub struct NestedExampleInUtils;

// Embed the example from `first_crate`
#[doc = docify::embed!("../first_crate/src/lib.rs", example_to_embed)]
pub struct FirstCrateExampleInUtils;
