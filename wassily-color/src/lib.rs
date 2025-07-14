//! # Wassily Color
//! 
//! Color utilities and palette management for the wassily generative art library.
//! Provides color space conversions, palette extraction, and color manipulation tools.

pub mod color_names;
pub mod color_palette;
pub mod kolor;

// Re-export key types and functions for convenience
pub use color_names::*;
pub use color_palette::*;
pub use kolor::*;

// Re-export commonly used external types
pub use palette::{
    Hsl, Hsla, Hsluv, Hsluva, Hsv, Hsva, Hwb, Hwba, Lab, Laba, Lch, Lcha, Lighten, Okhsl, Okhsla,
    Okhsv, Okhsva, Okhwb, Okhwba, Oklab, Oklaba, Oklch, Oklcha, ShiftHue, Srgb, Srgba, Xyz, Xyza,
};