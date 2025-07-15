//! # Prelude
//!
//! The prelude provides a convenient way to import all the essential functionality
//! from the wassily ecosystem. This is the recommended approach for most users.
//!
//! ## Usage
//!
//! ```rust
//! use wassily::prelude::*;
//!
//! fn main() {
//!     let mut canvas = Canvas::new(400, 400);
//!     canvas.fill(*WHITE);
//!     
//!     Shape::new()
//!         .circle(center(400, 400), 100.0)
//!         .fill_color(*BLUE)
//!         .draw(&mut canvas);
//!         
//!     canvas.save_png("circle.png");
//! }
//! ```
//!
//! ## What's Included
//!
//! The prelude re-exports all public items from:
//! - **wassily-core**: Canvas, Shape, drawing primitives, and utilities
//! - **wassily-color**: Color types, palettes, and color manipulation functions
//! - **wassily-noise**: Noise generation functions and utilities
//! - **wassily-geometry**: Geometric operations, curves, and spatial data structures
//! - **wassily-effects**: Visual effects and procedural textures
//! - **wassily-algorithms**: Specialized rendering algorithms
//!
//! Plus essential external dependencies:
//! - **tiny-skia**: Low-level graphics primitives
//! - **palette**: Advanced color types and color spaces
//! - **rand**: Random number generation
//! - **image**: Image loading and processing
//! - **noise**: Core noise functions

// Re-export core functionality from wassily-core
pub use crate::core::*;

// Re-export color functionality from wassily-color
pub use crate::color::*;

// Re-export algorithms functionality from wassily-algorithms
pub use crate::algorithms::*;

// Re-export effects functionality from wassily-effects
pub use crate::effects::*;

// Re-export geometry functionality from wassily-geometry
pub use crate::geometry::*;

// Re-export noise functionality from wassily-noise
pub use crate::noise::*;

#[allow(ambiguous_glob_reexports)]
pub use image::*;

pub use noise::core::worley::ReturnType;
pub use noise::core::worley::*;
pub use noise::*;
pub use palette;
pub use palette::{
    Hsl, Hsla, Hsluv, Hsluva, Hsv, Hsva, Hwb, Hwba, Lab, Laba, Lch, Lcha, Lighten, Okhsl, Okhsla,
    Okhsv, Okhsva, Okhwb, Okhwba, Oklab, Oklaba, Oklch, Oklcha, ShiftHue, Srgb, Srgba, Xyz, Xyza,
};
pub use rand::{rng, rngs::SmallRng, seq::SliceRandom, Rng, SeedableRng};
pub use rand_distr::{Distribution, Normal, StandardNormal};
pub use tiny_skia::*;
