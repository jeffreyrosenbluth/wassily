//! Specialized rendering algorithms for wassily.
//! 
//! This crate provides specialized algorithms for generative art including
//! endomorphisms and 3D sphere rendering with lighting.

pub mod endo2d;
pub mod sphere;

// Re-export key types and functions for convenience
pub use endo2d::*;
pub use sphere::*;