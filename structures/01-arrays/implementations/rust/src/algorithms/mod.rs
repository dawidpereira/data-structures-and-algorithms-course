//! Array-specific algorithm implementations.
//!
//! This module contains algorithms optimized for array data structures.

pub mod binary_search;
pub mod linear_search;
pub mod jump_search;
pub mod bubble_sort;

// Re-export commonly used traits
pub use binary_search::BinarySearchable;
pub use linear_search::LinearSearchable;
pub use jump_search::JumpSearchable;