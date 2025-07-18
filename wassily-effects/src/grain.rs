//! # Grain Effects
//!
//! Film grain and noise overlay effects for adding texture and visual interest
//! to digital artwork. This module provides tools for creating realistic grain
//! effects that simulate film photography or add organic texture to clean digital art.
//!
//! ## Key Features
//!
//! - **Film Grain**: Simulates the texture of film photography
//! - **Configurable Parameters**: Control grain size, intensity, and distribution
//! - **Blend Modes**: Uses overlay blending for natural grain appearance
//! - **Performance Optimized**: Efficient noise-based grain generation
//!
//! ## Basic Usage
//!
//! ```no_run
//! use wassily_effects::*;
//! use wassily_core::*;
//!
//! // Create grain effect
//! let grain = Grain::new(800, 600, 0.01, 0.5);
//! let grain_paint = grain.paint();
//!
//! // Apply to existing artwork
//! let mut canvas = Canvas::new(800, 600);
//! canvas.fill(*BLUE);  // Base color
//! canvas.fill_paint(&grain_paint);  // Add grain overlay
//! ```
//!
//! ## Parameters
//!
//! - **Scale**: Controls the grain size (smaller = finer grain)
//! - **Factor**: Controls the grain intensity (higher = more visible)
//! - **Blend Mode**: Uses overlay mode for natural grain appearance
//! - **Opacity**: Semi-transparent grain for subtle effect
//!
//! ## Applications
//!
//! - **Film Photography Simulation**: Add authentic film grain
//! - **Texture Enhancement**: Add organic texture to flat colors
//! - **Artistic Effects**: Create vintage or aged appearance
//! - **Visual Interest**: Break up large areas of flat color
use noise::{Fbm, Perlin};
use tiny_skia::{BlendMode, FilterQuality, Paint, Pattern, SpreadMode, Transform};
use wassily_color::rgb;
use wassily_core::{
    canvas::{paint_shader, Canvas},
    points::pt,
    shape::Shape,
};
use wassily_noise::{noise2d, NoiseOpts};

pub struct Grain(Canvas);

/// Create a grain effect
impl Grain {
    pub fn new(width: u32, height: u32, scale: f32, factor: f32) -> Self {
        let mut canvas = Canvas::new(width, height);
        let noise_opts = NoiseOpts::default().scales(scale).factor(factor);
        let nf = Fbm::<Perlin>::default();
        for i in 0..width {
            for j in 0..height {
                let n = noise2d(&nf, &noise_opts, i as f32, j as f32);
                let n1 = (n + 1.0) / 2.0;
                let n2 = n1.clamp(0.0, 1.0);
                let mut c = rgb(n2, n2, n2);
                c.set_alpha(0.4);
                let mut paint = Paint::default();
                paint.set_color(c);
                paint.blend_mode = BlendMode::Overlay;
                Shape::new()
                    .rect_xywh(pt(i, j), pt(1, 1))
                    .fill_paint(&paint)
                    .no_stroke()
                    .draw(&mut canvas);
            }
        }
        Grain(canvas)
    }

    /// Create a `Paint` with the grain to use as an overlay on a shape or path.
    pub fn grain(&self) -> Paint {
        let pattern = Pattern::new(
            (self.0).pixmap.as_ref(),
            SpreadMode::Repeat,
            FilterQuality::Bicubic,
            1.0,
            Transform::identity(),
        );
        let mut p = paint_shader(pattern);
        p.blend_mode = BlendMode::Overlay;
        p
    }

    /// Apply the grain to the entire `Canvas`.
    pub fn canvas_grain(&self, canvas: &mut Canvas) {
        let paint = self.grain();
        Shape::new()
            .rect_xywh(pt(0, 0), pt(canvas.w_f32(), canvas.h_f32()))
            .fill_paint(&paint)
            .draw(canvas);
    }
}
