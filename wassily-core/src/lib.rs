//! # Wassily Core
//! 
//! Core rendering infrastructure for the wassily generative art library.
//! Provides essential drawing and rendering functionality including canvas management,
//! shape building, point utilities, and mathematical operations.

pub mod canvas;
pub mod points;
pub mod shape;
pub mod util;

// Re-export key types and traits for convenience
pub use canvas::*;
pub use points::*;
pub use shape::*;
pub use util::*;

// Re-export commonly used external types
pub use tiny_skia::{
    Color, Paint, PathBuilder, Pixmap, PremultipliedColorU8, Rect, Stroke, Transform,
};