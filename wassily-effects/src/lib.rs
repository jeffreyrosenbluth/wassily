//! # Wassily Effects
//!
//! Visual effects and procedural textures for generative art applications.
//! This crate provides advanced visual effects, pattern generation, domain warping,
//! and sampling algorithms that add sophisticated visual treatments to your artwork.
//!
//! ## Key Features
//!
//! - **Procedural Textures**: Generate complex textures algorithmically
//! - **Visual Effects**: Grain, stippling, and surface effects
//! - **Domain Warping**: Transform coordinate spaces for distortion effects
//! - **Low-Discrepancy Sampling**: Superior point distribution algorithms
//! - **Pattern Generation**: Create repeating patterns and fills
//!
//! ## Modules
//!
//! - **[`textures`]**: Procedural texture generation and pattern creation
//! - **[`grain`]**: Film grain and noise overlay effects
//! - **[`stipple`]**: Low-discrepancy sampling and stippling algorithms
//! - **[`warp`]**: Domain warping and coordinate transformation effects
//!
//! ## Quick Start
//!
//! ### Adding Grain Effect
//! ```no_run
//! use wassily_effects::*;
//! use wassily_core::*;
//!
//! // Create a grain effect
//! let grain = Grain::new(800, 600, 0.01, 0.5);
//! let grain_paint = grain.paint();
//!
//! // Apply to a canvas
//! let mut canvas = Canvas::new(800, 600);
//! canvas.fill_paint(&grain_paint);
//! ```
//!
//! ### Creating Stipple Patterns
//! ```no_run
//! use wassily_effects::*;
//! use wassily_core::*;
//!
//! // Generate low-discrepancy point distribution
//! let points = halton_2d(1000, 800.0, 600.0);
//!
//! // Draw stipple pattern
//! let mut canvas = Canvas::new(800, 600);
//! for point in points {
//!     canvas.dot(point.x, point.y, *BLACK);
//! }
//! ```
//!
//! ### Procedural Textures
//! ```no_run
//! use wassily_effects::*;
//! use wassily_core::*;
//!
//! // Create a stipple texture
//! let texture = stipple_texture(256, 256, *BLACK, 10.0);
//!
//! // Use as pattern fill
//! let pattern_paint = PatternPaint::new(&texture, bbox, 256, 256);
//! ```
//!
//! ## Applications
//!
//! - **Artistic Effects**: Film grain, stippling, hatching
//! - **Texture Generation**: Procedural surface textures
//! - **Sampling**: Better-than-random point distributions
//! - **Image Processing**: Domain warping and distortion
//! - **Pattern Creation**: Repeating patterns and fills

pub mod grain;
pub mod stipple;
pub mod textures;
pub mod warp;

// Re-export key types and functions for convenience
pub use grain::*;
pub use stipple::*;
pub use textures::*;
pub use warp::*;