use crate::points::*;
use crate::prelude::Point;

#[derive(Debug, Clone)]
pub struct ParametricPath {
    points: Vec<Point>,
    lengths: Vec<f32>,
    total_length: f32,
    params: Vec<f32>,
}

impl ParametricPath {
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

    pub fn point_at(&self, t: f32) -> Point {
        assert!(t >= 0.0 && t <= 1.0);
        if t == 0.0 {
            return self.points[0];
        };
        if t == 1.0 {
            return self.points[self.points.len() - 1];
        };

        let (i, t1) = self.params.iter().enumerate().find(|&p| &t <= p.1).unwrap();
        let t0 = self.params[i - 1];
        self.points[i - 1].lerp(self.points[i], (t - t0) / (t1 - t0))
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::points::pt;

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
