use crate::canvas::*;
use crate::noises::*;
use crate::prelude::pt;
use crate::shape::*;
use noise::Perlin;
use tiny_skia::Paint;
use tiny_skia::{Color, Point, Rect, Transform};
pub struct SandBox {
    xy: Point,
    wh: Point,
    bg_color: Color,
    color1: Color,
    color2: Color,
    scales: f32,
}

impl SandBox {
    pub fn new(
        xy: Point,
        wh: Point,
        bg_color: Color,
        color1: Color,
        color2: Color,
        scales: f32,
    ) -> Self {
        Self {
            xy,
            wh,
            bg_color,
            color1,
            color2,
            scales,
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

    pub fn draw(&mut self, canvas: &mut Canvas) {
        let noise_opts = NoiseOpts::default()
            .width(self.wh.x)
            .height(self.wh.y)
            .scales(self.scales);
        let nf = Perlin::default();
        let rect = Rect::from_xywh(self.xy.x, self.xy.y, self.wh.x, self.wh.y).unwrap();
        let mut paint = Paint::default();
        paint.set_color(self.bg_color);
        canvas
            .0
            .fill_rect(rect, &paint, Transform::identity(), None);
        for i in 0..self.wh.x as u32 {
            let from = pt(self.xy.x + i as f32, self.xy.y);
            let to = pt(self.xy.x + i as f32, self.xy.y + self.wh.y);
            let alpha = noise2d_01(nf, &noise_opts, from.x, from.y);
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
            let alpha = noise2d_01(nf, &noise_opts, from.x, from.y);
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

impl Default for SandBox {
    fn default() -> Self {
        Self {
            xy: pt(0, 0),
            wh: pt(300, 300),
            bg_color: Color::WHITE,
            color1: Color::BLACK,
            color2: Color::BLACK,
            scales: 1.0,
        }
    }
}
