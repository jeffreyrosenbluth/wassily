//! # Wassily Geometry
//!
//! **Advanced geometric operations and spatial data structures for generative art.**
//! This crate provides sophisticated tools for working with curves, grids, spatial
//! data structures, and subdivision algorithms essential for creating complex
//! geometric patterns and algorithmic art.
//!
//! ## Key Features
//!
//! - **Parametric Curves**: Arc-length parameterized paths with interpolation and refinement
//! - **Spatial Data Structures**: Efficient quadtree implementation for point queries
//! - **Subdivision Algorithms**: Recursive quadrilateral and triangle subdivision with noise
//! - **Matrix Operations**: Generic matrix with convolution, multiplication, and linear algebra
//! - **Specialized Lines**: Artistic line effects including fade, sand, and stipple textures
//! - **Grid Systems**: Structured point grids with quadrilateral extraction
//!
//! ## Quick Start
//!
//! ```no_run
//! use wassily_geometry::*;
//! use wassily_core::points::pt;
//!
//! // Create a parametric curve
//! let points = vec![pt(0.0, 0.0), pt(100.0, 50.0), pt(200.0, 0.0)];
//! let curve = ParametricPath::new(points);
//! let midpoint = curve.point_at(0.5);  // Get point at 50% along curve
//!
//! // Subdivision example
//! let quad = Quad::new(pt(0.0, 0.0), pt(0.0, 100.0), pt(100.0, 100.0), pt(100.0, 0.0));
//! let (left, right) = quad.split_v(0.3, 0.7);
//!
//! // Quadtree for spatial queries
//! let mut qtree = QNode::new(vec![pt(10.0, 10.0), pt(90.0, 90.0)], pt(0.0, 0.0), pt(100.0, 100.0));
//! let nearby = qtree.points_in_circle(pt(0.0, 0.0), pt(100.0, 100.0), pt(50.0, 50.0), 25.0);
//! ```
//!
//! ## Core Components
//!
//! ### Curves and Paths
//!
//! - **[`ParametricPath`]**: Arc-length parameterized curves with uniform sampling
//! - **[`curve`]**: Generate smooth curves from mathematical functions
//! - **[`refine`]**: Adaptive curve refinement for optimal approximation quality
//!
//! ### Spatial Data Structures
//!
//! - **[`QNode`]**: Quadtree for efficient spatial queries and nearest neighbor searches
//! - **[`Grid`]**: Structured point grids with quadrilateral mesh extraction
//!
//! ### Subdivision Algorithms
//!
//! - **[`Quad`]**: Quadrilateral subdivision with customizable split strategies
//! - **[`Tri`]**: Triangle subdivision for triangular mesh generation
//! - **[`warp_points`]**: Noise-based point perturbation for organic distortion
//!
//! ### Matrix and Linear Algebra
//!
//! - **[`Matrix`]**: Generic matrix operations including convolution and multiplication
//! - Zero-indexed, row-major storage with comprehensive operation support
//!
//! ### Artistic Line Effects
//!
//! - **[`FadeLine`]**: Lines with variable opacity subdivisions
//! - **[`SandLine`]**: Textured lines with grain-like appearance  
//! - **[`DotLine`]**: Stippled lines with noise-based dot placement
//!
//! ## Applications
//!
//! - **Algorithmic Art**: Recursive subdivision patterns and fractal-like structures
//! - **Mesh Generation**: Quadrilateral and triangular mesh creation with organic distortion
//! - **Spatial Queries**: Efficient point location and nearest neighbor operations
//! - **Curve Manipulation**: Smooth interpolation and adaptive curve approximation
//! - **Pattern Generation**: Grid-based patterns with artistic line rendering
//!
//! ## Examples
//!
//! ### Recursive Subdivision
//!
//! ```no_run
//! use wassily_geometry::*;
//! use wassily_core::points::pt;
//!
//! // Create initial quad
//! let quad = Quad::new(pt(0.0, 0.0), pt(0.0, 100.0), pt(100.0, 100.0), pt(100.0, 0.0));
//! let mut quads = vec![quad];
//!
//! // Recursively subdivide
//! for _ in 0..5 {
//!     quads = quad_divide_vec(
//!         &quads,
//!         |q| q.best_dir(),           // Split along longest dimension
//!         |_| (0.4, 0.6),             // Split ratios
//!     );
//! }
//! ```
//!
//! ### Parametric Curve Interpolation
//!
//! ```no_run
//! use wassily_geometry::*;
//! use wassily_core::points::pt;
//!
//! // Define control points
//! let points = vec![
//!     pt(0.0, 50.0),
//!     pt(25.0, 100.0),
//!     pt(75.0, 0.0),
//!     pt(100.0, 50.0),
//! ];
//!
//! let path = ParametricPath::new(points);
//!
//! // Sample points uniformly along curve length
//! let samples: Vec<_> = (0..=20)
//!     .map(|i| path.point_at(i as f32 / 20.0))
//!     .collect();
//! ```
//!
//! ### Spatial Indexing with Quadtree
//!
//! ```no_run
//! use wassily_geometry::*;
//! use wassily_core::points::pt;
//!
//! // Build quadtree with scattered points
//! let points = vec![
//!     pt(10.0, 10.0), pt(25.0, 80.0), pt(90.0, 30.0),
//!     pt(45.0, 45.0), pt(70.0, 15.0), pt(30.0, 90.0),
//! ];
//!
//! let qtree = QNode::new(points, pt(0.0, 0.0), pt(100.0, 100.0));
//!
//! // Find all points within radius of query point
//! let query_center = pt(50.0, 50.0);
//! let radius = 30.0;
//! let nearby = qtree.points_in_circle(
//!     pt(0.0, 0.0), pt(100.0, 100.0),
//!     query_center, radius
//! );
//! ```

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