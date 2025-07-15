//! # Structured Point Grids and Quadrilateral Extraction
//!
//! Tools for working with structured grids of points and extracting quadrilateral
//! meshes from them. Useful for creating regular tessellations, distorted grids,
//! and structured patterns in generative art.
//!
//! ## Key Features
//!
//! - **[`Grid`]**: Structured grid of points with quadrilateral extraction
//! - **[`Quadrilateral`]**: Four-point geometric primitive for mesh generation
//! - **Spatial filtering**: Extract quads within specific boundary conditions
//!
//! ## Basic Usage
//!
//! ```no_run
//! use wassily_geometry::*;
//! use wassily_core::points::pt;
//!
//! // Create a matrix of points
//! let points_matrix = Matrix::generate(3, 3, |i, j| {
//!     pt(j as f32 * 10.0, i as f32 * 10.0)
//! });
//!
//! let grid = Grid { grid: points_matrix };
//!
//! // Extract all quadrilaterals
//! let quads = grid.quads();
//! assert_eq!(quads.len(), 4); // (3-1) × (3-1) = 4 quads
//!
//! // Filter quads within bounds
//! let bounded_quads = grid.quads_inside(50.0, 50.0);
//! ```

use crate::matrix::*;
use tiny_skia::Point;

/// **Structured grid of points for quadrilateral mesh generation.**
///
/// A `Grid` represents a structured arrangement of points in a matrix format,
/// where adjacent points can be connected to form quadrilateral elements.
/// This is essential for creating regular tessellations, deformed grids,
/// and structured geometric patterns.
///
/// ## Structure
///
/// Points are arranged in a matrix where:
/// - `grid[i][j]` represents the point at row i, column j
/// - Each 2×2 submatrix of points defines one quadrilateral
/// - Grid with m×n points produces (m-1)×(n-1) quadrilaterals
///
/// ## Example
///
/// ```no_run
/// use wassily_geometry::*;
/// use wassily_core::points::pt;
///
/// // Create 3×3 regular grid
/// let grid_matrix = Matrix::generate(3, 3, |i, j| {
///     pt(j as f32 * 20.0, i as f32 * 20.0)
/// });
/// let grid = Grid { grid: grid_matrix };
///
/// // Extract 2×2 = 4 quadrilaterals
/// let quads = grid.quads();
/// ```
#[derive(Debug)]
pub struct Grid {
    /// Matrix of points defining the grid structure
    pub grid: Matrix<Point>,
}

/// **Four-point quadrilateral in counter-clockwise order.**
///
/// Represents a quadrilateral defined by four corner points arranged as:
/// `[bottom_left, top_left, top_right, bottom_right]`
///
/// This ordering ensures consistent winding for rendering and geometric operations.
pub type Quadrilateral = [Point; 4];

impl Grid {
    fn get_quad(&self, i: usize, j: usize) -> Quadrilateral {
        assert!(
            i < self.grid.rows() && j < self.grid.cols(),
            "Quad index out of bounds"
        );
        let bl = self.grid[i + 1][j];
        let tl = self.grid[i][j];
        let tr = self.grid[i][j + 1];
        let br = self.grid[i + 1][j + 1];
        [bl, tl, tr, br]
    }

    /// **Extract all quadrilaterals from the grid.**
    ///
    /// Returns a vector containing all possible quadrilaterals that can be formed
    /// from adjacent 2×2 submatrices of grid points. For an m×n grid, this produces
    /// (m-1)×(n-1) quadrilaterals.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use wassily_geometry::*;
    /// use wassily_core::points::pt;
    ///
    /// let grid_matrix = Matrix::generate(4, 3, |i, j| {
    ///     pt(j as f32 * 10.0, i as f32 * 10.0)
    /// });
    /// let grid = Grid { grid: grid_matrix };
    ///
    /// let quads = grid.quads();
    /// assert_eq!(quads.len(), 6); // (4-1) × (3-1) = 3 × 2 = 6
    /// ```
    pub fn quads(&self) -> Vec<Quadrilateral> {
        let mut qs = Vec::new();
        for i in 0..self.grid.rows() - 1 {
            for j in 0..self.grid.cols() - 1 {
                qs.push(self.get_quad(i, j));
            }
        }
        qs
    }

    /// **Extract quadrilaterals that intersect with a rectangular boundary.**
    ///
    /// Returns only the quadrilaterals that have at least one vertex inside the
    /// specified rectangular region (0, 0) to (width, height). Useful for
    /// filtering grid elements to a specific viewport or drawing area.
    ///
    /// ## Parameters
    /// - `width`: Right boundary of the filtering rectangle
    /// - `height`: Top boundary of the filtering rectangle
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use wassily_geometry::*;
    /// use wassily_core::points::pt;
    ///
    /// // Create grid extending beyond bounds
    /// let grid_matrix = Matrix::generate(5, 5, |i, j| {
    ///     pt(j as f32 * 15.0 - 10.0, i as f32 * 15.0 - 10.0)
    /// });
    /// let grid = Grid { grid: grid_matrix };
    ///
    /// // Filter to quads within 50×50 region
    /// let visible_quads = grid.quads_inside(50.0, 50.0);
    /// // Only includes quads with at least one vertex in [0,50] × [0,50]
    /// ```
    pub fn quads_inside(&self, width: f32, height: f32) -> Vec<Quadrilateral> {
        let point_inside = |p: &Point| p.x > 0.0 && p.x < width && p.y > 0.0 && p.y < height;
        let quad_inside = |quad: &&[Point; 4]| quad.iter().any(point_inside);
        let quads = self.quads();
        let qs = quads.iter().filter(quad_inside).cloned();
        qs.collect()
    }
}
