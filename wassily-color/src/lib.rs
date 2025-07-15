//! # Wassily Color
//! 
//! Color utilities and palette management for the wassily generative art library.
//! This crate provides comprehensive color manipulation tools including color space
//! conversions, palette extraction from images, procedural color generation, and
//! advanced color operations specifically designed for generative art applications.
//!
//! ## Key Features
//!
//! - **Rich Color Spaces**: Support for RGB, HSL, HSV, Okhsl, Lab, and many more
//! - **Palette Management**: Create, manipulate, and extract color palettes
//! - **Named Colors**: Complete set of HTML/CSS color names
//! - **Procedural Generation**: Generate random colors in perceptually uniform spaces
//! - **Image Extraction**: Extract dominant colors from images
//! - **Advanced Operations**: Color blending, scaling, and fourier-based palettes
//!
//! ## Modules
//!
//! - **[`color_names`]**: Pre-defined HTML/CSS color constants
//! - **[`color_palette`]**: Palette management and color scaling functions
//! - **[`color`]**: Core color utilities, conversions, and generation functions
//!
//! ## Quick Start
//!
//! ```no_run
//! use wassily_color::*;
//! use tiny_skia::Color;
//!
//! // Use named colors
//! let blue = *CORNFLOWERBLUE;
//! let red = *CRIMSON;
//!
//! // Create random colors in perceptually uniform space
//! let mut rng = rand::thread_rng();
//! let random_color = rand_okhsl(&mut rng);
//!
//! // Create a color palette
//! let palette = Palette::new(vec![blue, red, random_color]);
//!
//! // Create a color scale for smooth transitions
//! let scale = ColorScale::new(blue, red, *WHITE, *BLACK, random_color);
//! let interpolated = scale.get_color(0.5);
//! ```
//!
//! ## Color Spaces
//!
//! This crate emphasizes modern, perceptually uniform color spaces:
//!
//! - **Okhsl/Okhsv**: Perceptually uniform HSL/HSV variants
//! - **Lab/Lch**: Perceptually uniform lightness and chroma
//! - **Traditional**: RGB, HSL, HSV for compatibility
//!
//! ## Advanced Features
//!
//! - **Fourier Color Scales**: Mathematically smooth color transitions
//! - **Image Color Extraction**: Dominant color extraction from images
//! - **Procedural Color Generation**: Noise-based and algorithmic color creation
//! - **Color Harmony**: Tools for creating harmonious color relationships

pub mod color_names;
pub mod color_palette;
pub mod color;

// Re-export key types and functions for convenience
pub use color_names::*;
pub use color_palette::*;
pub use color::*;

// Re-export commonly used external types
pub use palette::{
    Hsl, Hsla, Hsluv, Hsluva, Hsv, Hsva, Hwb, Hwba, Lab, Laba, Lch, Lcha, Lighten, Okhsl, Okhsla,
    Okhsv, Okhsva, Okhwb, Okhwba, Oklab, Oklaba, Oklch, Oklcha, ShiftHue, Srgb, Srgba, Xyz, Xyza,
};