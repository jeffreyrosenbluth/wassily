//! A matrix of points that can be viewed as a grid of quadrilaterals.
use crate::matrix::*;
use tiny_skia::Point;

#[derive(Debug)]
pub struct Grid {
    pub grid: Matrix<Point>,
}

/// [bottom left, top left, top right, bottom right]
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

    /// Returns a vector of all the quadilateralss in the grid.
    pub fn quads(&self) -> Vec<Quadrilateral> {
        let mut qs = Vec::new();
        for i in 0..self.grid.rows() - 1 {
            for j in 0..self.grid.cols() - 1 {
                qs.push(self.get_quad(i, j));
            }
        }
        qs
    }

    /// Returns a vector of all the quadilateralss in the grid that are inside the given width and height.
    pub fn quads_inside(&self, width: f32, height: f32) -> Vec<Quadrilateral> {
        let point_inside = |p: &Point| p.x > 0.0 && p.x < width && p.y > 0.0 && p.y < height;
        let quad_inside = |quad: &&[Point; 4]| quad.iter().any(point_inside);
        let quads = self.quads();
        let qs = quads.iter().filter(quad_inside).cloned();
        qs.collect()
    }
}
