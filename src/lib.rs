//! # Documentation for `second_crate`
//!
//! Here's how we use an example from `first_crate`:
//!
//! ```rust
//! # use first_crate::example_to_embed;
//! example_to_embed();
//! ```

// Set up git fallback
docif::set_git_fallback!("https://gitlab.com/prakashh21/first_crate.git", "src/lib.rs");

#[doc = docif::embed!("first_crate/src/lib.rs", example_to_embed)]
pub struct SomeItem;


