//! Geometric operations and spatial data structures for wassily.
//! 
//! This crate provides geometric algorithms, spatial data structures, and
//! specialized line drawing utilities for generative art.

pub mod curves;
pub mod grid;
pub mod lines;
pub mod matrix;
pub mod quadtree;
pub mod subdivision;

// Re-export key types and functions for convenience
pub use curves::*;
pub use grid::*;
pub use lines::*;
pub use matrix::*;
pub use quadtree::*;
pub use subdivision::*;