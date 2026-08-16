//! Contains helper structs to parse the pack metadata file for information relevant for
//! optimization purposes.

mod manifest;
pub use manifest::*;

mod version_constraints;

#[cfg(test)]
mod tests;
