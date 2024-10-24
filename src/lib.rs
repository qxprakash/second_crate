//! # Documentation for `second_crate`
//!
//! Here's how we use an example from `first_crate`:
//!
//! ```rust
//! # use first_crate::example_to_embed;
//! example_to_embed();
//! ```

#[doc = docify::embed!("../first_crate/src/lib.rs", example_to_embed)]
pub struct SomeItem;
