//! # Wassily Core
//! 
//! Core rendering infrastructure for the wassily generative art library.
//! This crate provides the fundamental building blocks for creating generative art:
//! canvas management, shape building, point utilities, and mathematical operations.
//!
//! ## Key Components
//!
//! - **[`Canvas`]**: The drawing surface that manages scaling and output
//! - **[`Shape`]**: A builder for creating geometric shapes with fills and strokes
//! - **[`points`]**: 2D point utilities and operations
//! - **[`util`]**: Mathematical utilities and helper functions
//!
//! ## Quick Start
//!
//! ```no_run
//! use wassily_core::*;
//! use tiny_skia::Color;
//!
//! let mut canvas = Canvas::new(400, 400);
//! canvas.fill(Color::from_rgba8(255, 255, 255, 255)); // White background
//!
//! // Draw a blue circle
//! Shape::new()
//!     .circle(center(400, 400), 50.0)
//!     .fill_color(Color::from_rgba8(0, 0, 255, 255))
//!     .draw(&mut canvas);
//!
//! canvas.save_png("output.png");
//! ```
//!
//! ## Features
//!
//! - **High-Quality Rendering**: Built on tiny-skia for precise vector graphics
//! - **Scalable Output**: Create images at any resolution using scale factors
//! - **Shape Builder**: Fluent API for creating complex geometric shapes
//! - **Multiple Formats**: Save as PNG, JPEG, and other image formats
//! - **Mathematical Utilities**: Point operations, transformations, and more
//!
//! ## Architecture
//!
//! This crate is designed to be the foundation layer for more specialized wassily crates.
//! It provides low-level primitives that other crates build upon for colors, noise,
//! effects, and advanced algorithms.

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