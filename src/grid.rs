use crate::matrix::*;
use crate::subdivision::*;
use tiny_skia::Point;

/// A matrix of points that can be viewed as a set of quadrilaterals.
#[derive(Debug)]
pub struct Grid(pub Matrix<Point>);

/// [bottom left, top left, top right, bottom right]
pub type Quadrilateral = [Point; 4];

impl Grid {
    pub fn perspective_grid(
        left_top: Point,
        left_bottom: Point,
        middle_top: Point,
        middle_bottom: Point,
        right_top: Point,
        right_bottom: Point,
        vert_stops: &[f32],
        left_stops: &[f32],
        right_stops: &[f32],
    ) -> Self {
        let rows = vert_stops.len();
        let cols = left_stops.len() + right_stops.len();
        // let mut mat: Matrix<Point> = Matrix::fill(rows, cols, Algebra::ZERO);
        let mut data: Vec<Point> = Vec::new();
        let left_vert = ray_points(left_top, left_bottom, vert_stops);
        let middle_vert = ray_points(middle_top, middle_bottom, vert_stops);
        let right_vert = ray_points(right_top, right_bottom, vert_stops);
        for (i, middle) in middle_vert.iter().enumerate() {
            data.extend(ray_points(left_vert[i], *middle, left_stops));
            data.extend(ray_points(*middle, right_vert[i], right_stops));
        }
        Grid(Matrix::new(rows, cols, data))
    }
    
    pub fn get_quad(&self, i: usize, j: usize) -> Quadrilateral {
        assert!(i <= self.0.rows() - 1 && j <= self.0.cols() - 1, "Quad index out of bounds");
        let bl = self.0[i + 1][j];
        let tl = self.0[i][j];
        let tr = self.0[i][j + 1];
        let br = self.0[i + 1][j + 1];
        [bl, tl, tr, br]
    }

    pub fn quads(&self) -> Vec<Quadrilateral> {
        let mut qs = Vec::new();
        for i in 0..self.0.rows() - 1 {
            for j in 0..self.0.cols() - 1 {
                qs.push(self.get_quad(i, j));
            }
        }
        qs
    }

    pub fn quads_inside(&self, width: f32, height: f32) -> Vec<Quadrilateral> {
        let point_inside = |p: &Point| {
            p.x > 0.0 && p.x < width && p.y > 0.0 && p.y < height
        };
        let quad_inside = |quad: &&[Point; 4]| {
            quad.iter().any(point_inside)
        };
        let quads = self.quads();
        let qs = quads.iter().filter(quad_inside).cloned();
        qs.collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::pt;
    #[test]
    fn perspective_grid_test() {
        let vert_stops = vec![0.0, 0.2, 0.5, 1.0];
        let left_stops = vec![0.0, 0.4, 1.0];
        let right_stops = vec![0.8, 1.0];
        let grid = Grid::perspective_grid(
            pt(0, 0),
            pt(0, 100),
            pt(150, 0),
            pt(150, 100),
            pt(300, 0),
            pt(300, 100),
            &vert_stops,
            &left_stops,
            &right_stops,
        );
        dbg!(grid.quads());
    }
}
