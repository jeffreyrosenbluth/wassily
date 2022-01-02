use crate::prelude::pt;
use crate::quiet::*;
use crate::shape::*;
use crate::util::*;
use noise::Perlin;
use tiny_skia::Paint;
use tiny_skia::{Color, Pixmap, Point, Rect, Transform};
pub struct SandBox {
    xy: Point,
    wh: Point,
    bg_color: Color,
    color1: Color,
    color2: Color,
    scales: f32,
}

impl SandBox {
    pub fn new(xy: Point, wh: Point) -> Self {
        Self {
            xy,
            wh,
            bg_color: Color::WHITE,
            color1: Color::BLACK,
            color2: Color::BLACK,
            scales: 1.0,
        }
    }

    pub fn set_bg(mut self, color: Color) -> Self {
        self.bg_color = color;
        self
    }

    pub fn set_color1(mut self, color: Color) -> Self {
        self.color1 = color;
        self
    }

    pub fn set_color2(mut self, color: Color) -> Self {
        self.color2 = color;
        self
    }

    pub fn draw(&mut self, canvas: &mut Pixmap) {
        let noise_opts = NoiseOpts::default()
            .width(self.wh.x)
            .height(self.wh.y)
            .scales(self.scales);
        let nf = Perlin::default();
        let rect = Rect::from_xywh(self.xy.x, self.xy.y, self.wh.x, self.wh.y).unwrap();
        let mut paint = Paint::default();
        paint.set_color(self.bg_color);
        canvas.fill_rect(rect, &paint, Transform::identity(), None);
        for i in 0..self.wh.x as u32 {
            let from = pt(self.xy.x + i as f32, self.xy.y);
            let to = pt(self.xy.x + i as f32, self.xy.y + self.wh.y);
            let alpha = map_range(
                noise2d(nf, &noise_opts, from.x, from.y),
                -1.0,
                1.0,
                0.0,
                1.0,
            );
            let mut color1 = self.color1;
            color1.set_alpha(alpha);
            ShapeBuilder::new()
                .line(from, to)
                .stroke_color(color1)
                .build()
                .draw(canvas);
        }
        for i in 0..self.wh.y as u32 {
            let from = pt(self.xy.x, self.xy.y + i as f32);
            let to = pt(self.xy.x + self.wh.x, self.xy.y + i as f32);
            let alpha = map_range(
                noise2d(nf, &noise_opts, from.x, from.y),
                -1.0,
                1.0,
                0.0,
                1.0,
            );
            let mut color2 = self.color2;
            color2.set_alpha(alpha);
            ShapeBuilder::new()
                .line(from, to)
                .stroke_color(color2)
                .build()
                .draw(canvas);
        }
    }
}
