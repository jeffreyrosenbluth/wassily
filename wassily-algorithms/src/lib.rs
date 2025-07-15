//! # Wassily Algorithms
//! 
//! Specialized rendering algorithms and advanced techniques for generative art.
//! This crate provides mathematical transformations, 3D rendering capabilities,
//! and specialized algorithms that extend the core wassily functionality with
//! advanced artistic and computational techniques.
//!
//! ## Key Components
//!
//! - **[`endo2d`]**: 2D endomorphisms and mathematical transformations
//! - **[`sphere`]**: 3D sphere rendering with lighting and texture mapping
//!
//! ## Mathematical Transformations
//!
//! The endomorphism functions provide various mathematical transformations
//! commonly used in generative art and computational graphics:
//!
//! ```no_run
//! use wassily_algorithms::*;
//! use wassily_core::pt;
//!
//! let point = pt(0.5, 0.3);
//! 
//! // Apply various transformations
//! let swirled = swirl(point);
//! let polar = to_polar(point);
//! let heart_shaped = heart(point);
//! ```
//!
//! ## 3D Rendering
//!
//! The sphere rendering system provides realistic 3D sphere rendering with:
//!
//! - **Texture Mapping**: Apply 2D textures to 3D surfaces
//! - **Lighting**: Multiple light sources with diffuse and specular reflection
//! - **Rotation**: 3D rotation around multiple axes
//! - **Perspective**: Realistic perspective projection
//!
//! ```no_run
//! use wassily_algorithms::*;
//! use wassily_core::*;
//!
//! // Create a texture canvas
//! let texture = Canvas::new(256, 256);
//! 
//! // Set up 3D scene
//! let scene = SphereScene::basic(pt3(0.0, 0.0, 100.0), &texture);
//!
//! // Render to output canvas
//! let mut output = Canvas::new(800, 600);
//! render_sphere(&scene, &mut output);
//! ```
//!
//! ## Applications
//!
//! These algorithms are particularly useful for:
//!
//! - **Fractal Systems**: IFS (Iterated Function Systems) with endomorphisms
//! - **3D Visualization**: Realistic rendering of 3D objects
//! - **Mathematical Art**: Visualization of complex mathematical functions
//! - **Advanced Effects**: Specialized transformations for artistic effects
//!
//! ## Performance Considerations
//!
//! These algorithms are computationally intensive and are best used for:
//! - High-quality final renders
//! - Specialized mathematical visualizations
//! - Cases where the advanced features justify the computational cost

pub mod endo2d;
pub mod sphere;

// Re-export key types and functions for convenience
pub use endo2d::*;
pub use sphere::*;