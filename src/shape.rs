//! A builder for creating shapes. `Shape` is the main way in which paths are build
//! and drawn to the canvas. The basid idea is to provide a list of points that
//! are connected by lines and curves. The shape can then be filled and/or stroked.
//! finally invoke the draw method to draw the shape to a  `Canvas`.
/*!


use wassily::prelude::*;

fn main() {
    let mut canvas = Canvas::new(500, 500);
    canvas.fill(*CORNFLOWERBLUE);
    let pos = center(500, 500);
    Shape::new()
        .star(pos, 100.0, 175.0, 8)
        .fill_color(*GREENYELLOW)
        .stroke_color(*MIDNIGHTBLUE)
        .stroke_weight(3.0)
        .draw(&mut canvas);
    canvas.save_png("./star.png");
}

<img src="https://raw.githubusercontent.com/jeffreyrosenbluth/wassily/main/assets/star.png" alt="Star image" width="500" />

 */

use crate::{
    canvas::Canvas,
    kolor::ConvertColor,
    prelude::{chaiken, pt, Trail, TAU},
};
use log::error;
use num_traits::AsPrimitive;
use rand::RngCore;
use rand_distr::{Distribution, Normal};
use tiny_skia::{
    FillRule, LineCap, LineJoin, Paint, PathBuilder, Point, Rect, Stroke, StrokeDash, Transform,
};

/// Shape types
#[derive(Debug, Clone, Copy)]
enum ShapeType {
    Poly,
    PolyQuad,
    PolyCubic,
    Rect,
    Circle,
    Line,
    Ellipse,
}

/// ShapeData holds the data required to draw a shape.
#[derive(Debug, Clone)]
struct ShapeData<'a> {
    points: Vec<Point>,
    fill_paint: Box<Option<Paint<'a>>>,
    stroke: Stroke,
    stroke_paint: Box<Option<Paint<'a>>>,
    shape: ShapeType,
    fillrule: FillRule,
    transform: Transform,
}

impl<'a> ShapeData<'a> {
    pub(crate) fn new(
        points: Vec<Point>,
        fill_paint: Box<Option<Paint<'a>>>,
        stroke: Stroke,
        stroke_paint: Box<Option<Paint<'a>>>,
        shape: ShapeType,
        fillrule: FillRule,
        transform: Transform,
    ) -> Self {
        Self {
            points,
            fill_paint,
            stroke,
            stroke_paint,
            shape,
            fillrule,
            transform,
        }
    }

    fn draw(&self, canvas: &mut Canvas) {
        let shape = self.shape;
        match shape {
            ShapeType::Poly => self.draw_poly(canvas),
            ShapeType::PolyQuad => self.draw_quad(canvas),
            ShapeType::PolyCubic => self.draw_cubic(canvas),
            ShapeType::Rect => self.draw_rect(canvas),
            ShapeType::Circle => self.draw_circle(canvas),
            ShapeType::Line => self.draw_line(canvas),
            ShapeType::Ellipse => self.draw_ellipse(canvas),
        }
    }

    // Draw a polygon. Must have at least 2 points.
    fn draw_poly(&self, canvas: &mut Canvas) {
        if self.points.len() < 2 {
            error!(
                "Cannot draw a polygonal curve with less than 2 points, only {} points provided.",
                self.points.len()
            );
            return;
        }
        let mut pb = PathBuilder::new();
        let head = self.points[0];
        let tail = &self.points[1..];
        pb.move_to(head.x, head.y);
        for p in tail {
            pb.line_to(p.x, p.y);
        }
        if self.fill_paint.is_some() {
            pb.close();
        }
        let path = pb.finish().unwrap();
        if let Some(mut fp) = *self.fill_paint.clone() {
            canvas.fill_path(&path, &mut fp, self.fillrule, self.transform, None);
        }
        if let Some(mut sp) = *self.stroke_paint.clone() {
            canvas.stroke_path(&path, &mut sp, &self.stroke, self.transform, None);
        }
    }

    // Draw a quadratic bezier curve. Must have at least 3 points.
    fn draw_quad(&self, canvas: &mut Canvas) {
        if self.points.len() < 3 {
            error!(
                "Cannot draw a quadratic bezier curve with less than 3 points, only {} points provided.",
                self.points.len()
            );
            return;
        }
        let mut pb = PathBuilder::new();
        let head = self.points[0];
        pb.move_to(head.x, head.y);
        let tail = self.points[1..].chunks_exact(2);
        for t in tail {
            let control = t[0];
            let p = t[1];
            pb.quad_to(control.x, control.y, p.x, p.y);
        }
        if self.fill_paint.is_some() {
            pb.close();
        }
        let path = pb.finish().unwrap();
        if let Some(mut fp) = *self.fill_paint.clone() {
            canvas.fill_path(&path, &mut fp, self.fillrule, self.transform, None);
        }
        if let Some(mut sp) = *self.stroke_paint.clone() {
            canvas.stroke_path(&path, &mut sp, &self.stroke, self.transform, None);
        }
    }

    // Draw a cubic bezier curve. Must have at least 4 points. Note that the
    // first and last points are the endpoints, the middle two points are control
    // points for every group of 4 points.
    fn draw_cubic(&self, canvas: &mut Canvas) {
        if self.points.len() < 4 {
            error!(
                "Cannot draw a cubic bezier curve with less than 4 points, only {} points provided.",
                self.points.len()
            );
            return;
        }
        let mut pb = PathBuilder::new();
        let head = self.points[0];
        pb.move_to(head.x, head.y);
        let tail = self.points[1..].chunks_exact(3);
        for t in tail {
            let control1 = t[0];
            let control2 = t[1];
            let p = t[2];
            pb.cubic_to(control1.x, control1.y, control2.x, control2.y, p.x, p.y);
        }
        if self.fill_paint.is_some() {
            pb.close();
        }
        let path = pb.finish().unwrap();
        if let Some(mut fp) = *self.fill_paint.clone() {
            canvas.fill_path(&path, &mut fp, self.fillrule, self.transform, None);
        }
        if let Some(mut sp) = *self.stroke_paint.clone() {
            canvas.stroke_path(&path, &mut sp, &self.stroke, self.transform, None);
        }
    }

    // Draw a rectangle. Must have at least 2 points, top left and bottom right.
    fn draw_rect(&self, canvas: &mut Canvas) {
        if self.points.len() < 2 {
            error!(
                "Cannot draw a rectangle with less than 2 points, only {} points provided.",
                self.points.len()
            );
            return;
        }
        let left = self.points[0].x;
        let top = self.points[0].y;
        let right = self.points[1].x;
        let bottom = self.points[1].y;
        let rect = Rect::from_ltrb(left, top, right, bottom).unwrap();
        let path = PathBuilder::from_rect(rect);
        if let Some(mut fp) = *self.fill_paint.clone() {
            canvas.fill_path(&path, &mut fp, self.fillrule, self.transform, None);
        }
        if let Some(mut sp) = *self.stroke_paint.clone() {
            canvas.stroke_path(&path, &mut sp, &self.stroke, self.transform, None);
        }
    }

    // Draw a circle. Must have at least 2 points, center and a point whose x coordinate is the radius.
    fn draw_circle(&self, canvas: &mut Canvas) {
        if self.points.len() < 2 {
            error!(
                "Cannot draw a circle with less than 2 points, only {} points provided.",
                self.points.len()
            );
            return;
        }
        let cx = self.points[0].x;
        let cy = self.points[0].y;
        let w = self.points[1].x;
        let circle = PathBuilder::from_circle(cx, cy, w).unwrap();
        if let Some(mut fp) = *self.fill_paint.clone() {
            canvas.fill_path(&circle, &mut fp, self.fillrule, self.transform, None);
        }
        if let Some(mut sp) = *self.stroke_paint.clone() {
            canvas.stroke_path(&circle, &mut sp, &self.stroke, self.transform, None);
        }
    }

    // Draw an ellipse. Must have at least 2 points, center and a point whose x coordinate is the
    // width and y coordinate is the height.
    fn draw_ellipse(&self, canvas: &mut Canvas) {
        if self.points.len() < 2 {
            error!(
                "Cannot draw a ellipse with less than 2 points, only {} points provided.",
                self.points.len()
            );
            return;
        }
        let cx = self.points[0].x;
        let cy = self.points[0].y;
        let w = self.points[1].x;
        let h = self.points[1].y;
        let rect = Rect::from_xywh(cx - w / 2.0, cy - h / 2.0, w, h).unwrap();
        let ellipse = PathBuilder::from_oval(rect).unwrap();
        if let Some(mut fp) = *self.fill_paint.clone() {
            canvas.fill_path(&ellipse, &mut fp, self.fillrule, self.transform, None);
        }
        if let Some(mut sp) = *self.stroke_paint.clone() {
            canvas.stroke_path(&ellipse, &mut sp, &self.stroke, self.transform, None);
        }
    }

    // Draw a line. Must have at least 2 points. Any more points are ignored.
    fn draw_line(&self, canvas: &mut Canvas) {
        if self.points.len() < 2 {
            error!(
                "Cannot draw a line with less than 2 points, only {} points provided.",
                self.points.len()
            );
            return;
        }
        let x0 = self.points[0].x;
        let y0 = self.points[0].y;
        let x1 = self.points[1].x;
        let y1 = self.points[1].y;
        let mut pb = PathBuilder::new();
        pb.move_to(x0, y0);
        pb.line_to(x1, y1);
        let path = pb.finish().unwrap();
        if let Some(mut sp) = *self.stroke_paint.clone() {
            canvas.stroke_path(&path, &mut sp, &self.stroke, self.transform, None);
        }
    }
}

/// A builder for creating shapes. `Shape` is the main way in which paths are build
/// and drawn to the canvas. The basid idea is to provide a list of points that
/// are connected by lines and curves. The shape can then be filled and/or stroked.
/// finally invoke the draw method to draw the shape to a  `Canvas`.
#[derive(Debug, Clone)]
pub struct Shape<'a> {
    fill_paint: Option<Paint<'a>>,
    stroke_paint: Option<Paint<'a>>,
    stroke_width: f32,
    miter_limit: f32,
    line_cap: LineCap,
    line_join: LineJoin,
    stroke_dash: Option<StrokeDash>,
    points: Vec<Point>,
    shape: ShapeType,
    fillrule: FillRule,
    transform: Transform,
}

impl Default for Shape<'_> {
    fn default() -> Self {
        let fill = Paint {
            anti_alias: true,
            ..Default::default()
        };
        let stroke = Paint {
            anti_alias: true,
            ..Default::default()
        };
        Self {
            fill_paint: Some(fill),
            stroke_paint: Some(stroke),
            stroke_width: 1.0,
            miter_limit: Default::default(),
            line_cap: Default::default(),
            line_join: Default::default(),
            stroke_dash: None,
            points: vec![],
            shape: ShapeType::Poly,
            fillrule: FillRule::Winding,
            transform: Transform::identity(),
        }
    }
}

impl<'a> Shape<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the fill color of the shape.
    pub fn fill_color(mut self, color: impl ConvertColor) -> Self {
        let mut paint = Paint {
            anti_alias: true,
            ..Default::default()
        };
        paint.set_color(color.to_color());
        self.fill_paint = Some(paint);
        self
    }

    /// Set the fill paint of the shape. This allows for more complex fills
    /// such as gradients and patterns.
    pub fn fill_paint(mut self, texture: &'a Paint) -> Self {
        self.fill_paint = Some(texture.clone());
        self
    }

    /// Don't fill the shape. If this is not called the shape will not be closed.
    pub fn no_fill(mut self) -> Self {
        self.fill_paint = None;
        self
    }
    /// Don't stroke the shape.
    pub fn no_stroke(mut self) -> Self {
        self.stroke_paint = None;
        self
    }

    /// Set the stroke color of the shape.
    pub fn stroke_color(mut self, color: impl ConvertColor) -> Self {
        let mut paint = Paint {
            anti_alias: true,
            ..Default::default()
        };
        paint.set_color(color.to_color());
        self.stroke_paint = Some(paint);
        self
    }

    /// Set the stroke paint of the shape. This allows for more complex fills
    /// such as gradients and patterns.
    pub fn stroke_paint(mut self, paint: &'a Paint) -> Self {
        self.stroke_paint = Some(paint.clone());
        self
    }

    /// Set the stroke width of the shape.
    pub fn stroke_weight(mut self, weight: f32) -> Self {
        self.stroke_width = weight;
        self
    }

    /// Set the line cap of the shape.
    pub fn line_cap(mut self, cap: LineCap) -> Self {
        self.line_cap = cap;
        self
    }

    /// Set the line join of the shape.
    pub fn line_join(mut self, join: LineJoin) -> Self {
        self.line_join = join;
        self
    }

    /// Set the stroke dash of the shape.
    pub fn stroke_dash(mut self, dash: StrokeDash) -> Self {
        self.stroke_dash = Some(dash);
        self
    }

    /// Set the points that determin the shape.
    pub fn points(mut self, pts: &[Point]) -> Self {
        let points = pts.to_vec();
        self.points = points;
        self
    }

    /// Interpret the points as a rectangle. The first point is the top left corner
    /// and the second point is the bottom right corner.
    pub fn rect_ltrb(mut self, lt: Point, rb: Point) -> Self {
        self.shape = ShapeType::Rect;
        self.points = vec![lt, rb];
        self
    }

    /// Inerpret the points as a rectangle. The first point is the top left corner
    /// and the second point is the size of the rectangle.
    pub fn rect_xywh(mut self, xy: Point, wh: Point) -> Self {
        self.shape = ShapeType::Rect;
        self.points = vec![xy, pt(xy.x + wh.x, xy.y + wh.y)];
        self
    }

    /// Inerpret the points as a rectangle. The first point is the center of the rectangle
    /// and the second point is the size of the rectangle.
    pub fn rect_cwh(mut self, c: Point, wh: Point) -> Self {
        self.shape = ShapeType::Rect;
        let w2 = wh.x / 2.0;
        let h2 = wh.y / 2.0;
        let p = pt(c.x - w2, c.y - h2);
        self.rect_xywh(p, wh)
    }

    /// Create circle from it's center and radius.
    pub fn circle(mut self, center: Point, radius: f32) -> Self {
        self.shape = ShapeType::Circle;
        self.points = vec![center, pt(radius, radius)];
        self
    }

    /// Create ellispse from it's center, width and heighr.
    pub fn ellipse(mut self, center: Point, width: f32, height: f32) -> Self {
        self.shape = ShapeType::Ellipse;
        self.points = vec![center, pt(width, height)];
        self
    }

    /// Create a polygon from it's center, radius and number of sides.
    pub fn polygon(mut self, center: Point, radius: f32, sides: u32) -> Self {
        self.shape = ShapeType::Poly;
        let mut theta = 0.0;
        let delta = TAU / sides as f32;
        let mut pts = vec![];
        while theta < TAU {
            pts.push(pt(
                center.x + radius * theta.cos(),
                center.y + radius * theta.sin(),
            ));
            theta += delta;
        }
        self.points = pts;
        self
    }

    /// Create a star from it's center, inner radius, outer radius and number of sides.
    pub fn star(mut self, center: Point, radius1: f32, radius2: f32, sides: u32) -> Self {
        self.shape = ShapeType::Poly;
        let mut theta = 0.0;
        let delta = TAU / sides as f32;
        let half_delta = delta / 2.0;
        let mut pts = vec![];
        while theta < TAU {
            pts.push(pt(
                center.x + radius2 * theta.cos(),
                center.y + radius2 * theta.sin(),
            ));
            pts.push(pt(
                center.x + radius1 * (theta + half_delta).cos(),
                center.y + radius1 * (theta + half_delta).sin(),
            ));
            theta += delta;
        }
        self.points = pts;
        self
    }

    /// A `pearl` is a deformed polygon using the normal distribution to alter the
    /// location of each point. The `a` and `b` parameters control the width and height
    /// of the pearl. The `sides` parameter controls the number of sides of the polygon.
    /// The `smoothness` parameter controls the smoothnes of the polygon.
    /// using the Chaiken algorithm.
    pub fn pearl<R: RngCore>(
        mut self,
        center: Point,
        a: f32,
        b: f32,
        sides: u32,
        smoothness: u32,
        rng: &mut R,
    ) -> Self {
        self.shape = ShapeType::Poly;
        let mut points = vec![];
        for i in 0..sides {
            let normal = Normal::new(0.0, 0.25 * a.min(b)).unwrap();
            let dx = normal.sample(rng);
            let dy = normal.sample(rng);
            let u = TAU * i as f32 / sides as f32;
            let x1 = a * u.cos() + center.x + dx;
            let y1 = b * u.sin() + center.y + dy;
            points.push(pt(x1, y1));
        }
        self.points = chaiken(&points, smoothness, Trail::Closed)
            .into_iter()
            .collect();
        self
    }

    /// Interpret the points as a quadratic bezier curve. The first point is the start point,
    /// the second point is the control point and the third point is the end point and so on.
    pub fn quad(mut self) -> Self {
        self.shape = ShapeType::PolyQuad;
        self
    }

    /// Interpret the points as a cubic bezier curve. The first point is the start point,
    /// the second point is the first control point, the third point is the second control point
    /// and the fourth point is the end point and so on.
    pub fn cubic(mut self) -> Self {
        self.shape = ShapeType::PolyCubic;
        self
    }

    /// Create a line from it's start and end points.
    pub fn line(mut self, from: Point, to: Point) -> Self {
        self.points = vec![from, to];
        self.shape = ShapeType::Line;
        self
    }

    /// Set the fill rule for the shape.
    pub fn fill_rule(mut self, fillrule: FillRule) -> Self {
        self.fillrule = fillrule;
        self
    }

    /// Set the transform for the shape.
    pub fn transform(mut self, transform: &Transform) -> Self {
        let t = self.transform.post_concat(*transform);
        self.transform = t;
        self
    }

    /// Interpret points as cartiesian coordinates with center at (0, 0).
    pub fn cartesian<T: AsPrimitive<f32>>(mut self, width: T, height: T) -> Self {
        self.transform = self
            .transform
            .post_scale(1.0, -1.0)
            .post_translate(width.as_() / 2.0, height.as_() / 2.0);
        self
    }

    /// Draw the shape to the canvas.
    pub fn draw(self, canvas: &mut Canvas) {
        let mut fill_paint: Box<Option<Paint<'_>>> = Box::new(None);
        let mut stroke_paint: Box<Option<Paint<'_>>> = Box::new(None);
        if let Some(fs) = self.fill_paint {
            fill_paint = Box::new(Some(fs));
        };
        if let Some(ss) = self.stroke_paint {
            stroke_paint = Box::new(Some(ss));
        };
        let stroke = Stroke {
            width: self.stroke_width,
            miter_limit: self.miter_limit,
            line_cap: self.line_cap,
            line_join: self.line_join,
            dash: self.stroke_dash,
        };
        ShapeData::new(
            self.points,
            fill_paint,
            stroke,
            stroke_paint,
            self.shape,
            self.fillrule,
            self.transform,
        )
        .draw(canvas);
    }
}
