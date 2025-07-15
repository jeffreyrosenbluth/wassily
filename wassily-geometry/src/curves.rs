//! # Parametric Curves and Function Plotting
//!
//! Tools for creating and manipulating parametric curves with arc-length parameterization.
//! This module provides utilities for curve refinement, function plotting, and uniform
//! sampling along curved paths.
//!
//! ## Key Features
//!
//! - **[`ParametricPath`]**: Arc-length parameterized curves for uniform sampling
//! - **[`curve`]**: Generate smooth curves from mathematical functions  
//! - **[`refine`]**: Adaptive curve refinement for optimal quality/performance balance
//!
//! ## Basic Usage
//!
//! ```no_run
//! use wassily_geometry::*;
//! use wassily_core::points::pt;
//!
//! // Create parametric path from control points
//! let points = vec![pt(0.0, 0.0), pt(50.0, 100.0), pt(100.0, 0.0)];
//! let path = ParametricPath::new(points);
//!
//! // Sample uniformly along curve length
//! let midpoint = path.point_at(0.5);    // 50% along the curve
//! let quarter = path.point_at(0.25);    // 25% along the curve
//!
//! // Extract curve section
//! let section = path.section(0.2, 0.8); // Points from 20% to 80%
//! ```

use tiny_skia::Point;
use wassily_core::points::*;

/// **Arc-length parameterized curve for uniform sampling along curved paths.**
///
/// A `ParametricPath` represents a piecewise linear curve that can be sampled uniformly
/// by arc length rather than by point index. This is essential for animations, particle
/// systems, and any application requiring consistent spacing along curved paths.
///
/// ## Key Properties
///
/// - **Arc-length parameterization**: Parameter t ∈ [0,1] represents distance along curve
/// - **Uniform sampling**: Equal parameter differences yield equal arc lengths
/// - **Efficient queries**: Pre-computed cumulative lengths for O(log n) point lookup
/// - **Section extraction**: Extract curve subsections with proper parameterization
///
/// ## Usage
///
/// ```no_run
/// use wassily_geometry::*;
/// use wassily_core::points::pt;
///
/// // Create curve from control points
/// let control_points = vec![
///     pt(0.0, 0.0),   // Start
///     pt(50.0, 100.0), // Peak
///     pt(100.0, 50.0), // End
/// ];
/// let path = ParametricPath::new(control_points);
///
/// // Sample at regular intervals (uniform spacing)
/// let samples: Vec<_> = (0..=10)
///     .map(|i| path.point_at(i as f32 / 10.0))
///     .collect();
/// ```
#[derive(Debug, Clone)]
pub struct ParametricPath {
    /// The control points defining the piecewise linear curve
    pub points: Vec<Point>,
    /// Length of each segment between consecutive points
    pub lengths: Vec<f32>,
    /// Total arc length of the entire curve
    pub total_length: f32,
    /// Cumulative arc length parameters for each point (normalized to [0,1])
    pub params: Vec<f32>,
}

impl ParametricPath {
    /// **Create a new parametric path from a sequence of points.**
    ///
    /// Computes arc-length parameterization by calculating cumulative distances
    /// between consecutive points. Requires at least 2 points.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use wassily_geometry::*;
    /// use wassily_core::points::pt;
    ///
    /// let points = vec![pt(0.0, 0.0), pt(10.0, 0.0), pt(10.0, 10.0)];
    /// let path = ParametricPath::new(points);
    /// assert_eq!(path.total_length, 20.0); // 10 + 10
    /// ```
    pub fn new(points: Vec<Point>) -> Self {
        assert!(points.len() >= 2);
        let mut lengths = Vec::new();
        for p in points.windows(2) {
            lengths.push(p[0].distance(p[1]));
        }
        let total_length = lengths.iter().sum::<f32>();
        let mut params: Vec<f32> = lengths
            .iter()
            .scan(0.0, |state, &length| {
                *state += length;
                Some(*state / total_length)
            })
            .collect();
        params.insert(0, 0f32);
        Self {
            points,
            lengths: lengths.clone(),
            total_length,
            params,
        }
    }

    /// **Sample the curve at parameter t ∈ [0,1] using arc-length parameterization.**
    ///
    /// Returns the point that is t * total_length distance along the curve from the start.
    /// This provides uniform sampling where equal parameter differences correspond to
    /// equal arc lengths along the curve.
    ///
    /// ## Parameters
    /// - `t`: Parameter in range [0,1] where 0 = start, 1 = end, 0.5 = midpoint by arc length
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use wassily_geometry::*;
    /// use wassily_core::points::pt;
    ///
    /// let path = ParametricPath::new(vec![pt(0.0, 0.0), pt(100.0, 0.0)]);
    /// assert_eq!(path.point_at(0.0), pt(0.0, 0.0));   // Start
    /// assert_eq!(path.point_at(1.0), pt(100.0, 0.0)); // End  
    /// assert_eq!(path.point_at(0.5), pt(50.0, 0.0));  // Midpoint
    /// ```
    pub fn point_at(&self, t: f32) -> Point {
        assert!((0.0..=1.0).contains(&t));
        if t == 0.0 {
            return self.points[0];
        };
        if t == 1.0 {
            return self.points[self.points.len() - 1];
        };

        let (i, t1) = self
            .params
            .iter()
            .enumerate()
            .find(|&p| &t <= p.1)
            .unwrap_or_else(|| panic!("Cannot find param >= t ({t})"));
        let t0 = self.params[i - 1];
        self.points[i - 1].lerp(self.points[i], (t - t0) / (t1 - t0))
    }

    /// **Extract a section of the curve between two parameter values.**
    ///
    /// Returns all points along the curve between parameters t0 and t1, including
    /// interpolated points at the exact boundaries if they don't fall on existing
    /// control points.
    ///
    /// ## Parameters
    /// - `t0`: Start parameter (must be ≥ 0.0 and < t1)
    /// - `t1`: End parameter (must be ≤ 1.0 and > t0)
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use wassily_geometry::*;
    /// use wassily_core::points::pt;
    ///
    /// let path = ParametricPath::new(vec![
    ///     pt(0.0, 0.0), pt(50.0, 0.0), pt(100.0, 0.0)
    /// ]);
    ///
    /// // Extract middle 50% of the curve
    /// let middle_section = path.section(0.25, 0.75);
    /// // Returns points from 25% to 75% along the curve length
    /// ```
    pub fn section(&self, t0: f32, t1: f32) -> Vec<Point> {
        assert!(t0 >= 0.0 && t0 < t1 && t1 <= 1.0);
        let idx0 = self.params.iter().position(|t| t >= &t0).unwrap();
        let idx1 = self.params.iter().position(|t| t >= &t1).unwrap();
        let mut points = Vec::new();
        if self.params[idx0] > t0 {
            points.push(self.point_at(t0));
        }
        for i in idx0..idx1 {
            points.push(self.points[i]);
        }
        points.push(self.point_at(t1));
        points
    }
}

/// **Adaptively refine a piecewise linear curve to approximate a continuous function.**
///
/// Recursively subdivides line segments until the piecewise linear approximation
/// is within `eps` distance of the true function values. This creates smooth-looking
/// curves with optimal point density.
///
/// ## Parameters
/// - `pts`: Initial control points defining the piecewise linear approximation
/// - `f`: Function to approximate (takes x coordinate, returns y coordinate)
/// - `eps`: Maximum allowed error between linear segments and function
///
/// ## Algorithm
/// For each segment, tests the midpoint. If the function value at the midpoint
/// differs from the linear interpolation by more than `eps`, inserts the true
/// function point and recursively refines both sub-segments.
///
/// ## Example
///
/// ```no_run
/// use wassily_geometry::*;
/// use wassily_core::points::pt;
/// use std::f32::consts::PI;
///
/// // Create initial coarse approximation of sine curve
/// let initial_points = vec![pt(0.0, 0.0), pt(PI, 0.0)];
///
/// // Refine to approximate sin(x) within 0.1 units
/// let refined = refine(&initial_points, |x| x.sin(), 0.1);
/// // Result: smooth sine curve with adaptive point density
/// ```
pub fn refine(pts: &[Point], f: impl Fn(f32) -> f32, eps: f32) -> Vec<Point> {
    fn refiner(pts: &[Point], f: impl Fn(f32) -> f32, idx: usize, eps: f32) -> Vec<Point> {
        let mut refined_pts = pts.to_vec();
        if idx > refined_pts.len() - 2 {
            refined_pts
        } else {
            let a = refined_pts[idx];
            let b = refined_pts[idx + 1];
            let m = (a + b).scale(0.5);
            let q = pt(m.x, f(m.x));
            if (q.y - m.y).abs() > eps {
                refined_pts.insert(idx + 1, q);
                refiner(&refined_pts, f, idx, eps)
            } else {
                refiner(&refined_pts, f, idx + 1, eps)
            }
        }
    }
    refiner(pts, f, 0, eps)
}

/// **Generate a smooth curve from a mathematical function.**
///
/// Creates a piecewise linear approximation of a function over a specified domain,
/// automatically refined to achieve the desired smoothness. The function is scaled
/// and positioned according to the provided parameters.
///
/// ## Parameters
/// - `f`: Function to plot (maps x ∈ [0, 2π] to y values)
/// - `start`: Bottom-left corner of the plotting area
/// - `width`: Width of the plotting area (domain scaling)
/// - `height`: Height of the plotting area (range scaling)  
/// - `eps`: Maximum error tolerance for curve smoothness
///
/// ## Coordinate Transform
/// The function domain [0, 2π] is mapped to [start.x, start.x + width], and
/// the function range is scaled by `height` and positioned at `start.y`.
///
/// ## Example
///
/// ```no_run
/// use wassily_geometry::*;
/// use wassily_core::points::pt;
/// use std::f32::consts::PI;
///
/// // Generate sine wave curve
/// let sine_curve = curve(
///     |x| x.sin(),              // Sine function
///     pt(0.0, 100.0),           // Start position
///     400.0,                    // Width (domain)
///     50.0,                     // Height (amplitude)
///     0.5                       // Error tolerance
/// );
///
/// // Result: smooth sine wave from (0,100) to (400,100) with amplitude ±50
/// ```
pub fn curve(f: fn(f32) -> f32, start: Point, width: f32, height: f32, eps: f32) -> Vec<Point> {
    let g = |x: f32| start.y + height * f(2.0 * PI * x / width);
    let mut pts = Vec::new();
    for i in 0..=10 {
        let s = width * i as f32 / 10.0;
        pts.push(pt(start.x + s, g(s)));
    }
    refine(&pts, g, eps)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wassily_core::points::pt;

    #[test]
    fn new_test() {
        let points = vec![pt(0, 0), pt(1, 1), pt(2, 3)];
        let pp = ParametricPath::new(points);
        assert_eq!(pp.lengths[0], 2_f32.sqrt());
        assert_eq!(pp.lengths[1], 5_f32.sqrt());
        assert_eq!(pp.total_length, 2_f32.sqrt() + 5_f32.sqrt());
        assert_eq!(pp.params, vec![0.0, 0.3874259, 1.0]);
    }

    #[test]
    fn point_at_test() {
        let points = vec![pt(0, 0), pt(1, 1), pt(2, 3)];
        let pp = ParametricPath::new(points);
        assert_eq!(pp.point_at(0.0), pt(0, 0));
        assert_eq!(pp.point_at(1.0), pt(2, 3));
        assert_eq!(pp.point_at(0.5), pt(1.1837722, 1.3675444));
        assert_eq!(pp.point_at(0.25), pt(0.6452847, 0.6452847));
        assert_eq!(pp.point_at(0.75), pt(1.591886, 2.1837723));
        assert_eq!(pp.point_at(0.3874259), pt(1.0, 1.0));
    }

    #[test]
    fn section_test() {
        let points = vec![pt(0, 0), pt(1, 1), pt(2, 3)];
        let pp = ParametricPath::new(points.clone());
        assert_eq!(pp.section(0.0, 1.0), points);
        assert_eq!(
            pp.section(0.5, 1.0),
            vec![pt(1.1837722, 1.3675444), pt(2, 3)]
        );
        assert_eq!(
            pp.section(0.25, 0.5),
            vec![
                pt(0.6452847, 0.6452847),
                pt(1.0, 1.0),
                pt(1.1837722, 1.3675444)
            ]
        );
        assert_eq!(
            pp.section(0.25, 0.75),
            vec![
                pt(0.6452847, 0.6452847),
                pt(1.0, 1.0),
                pt(1.591886, 2.1837723)
            ]
        )
    }
}
