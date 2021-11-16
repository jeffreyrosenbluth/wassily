use crate::base::*;
use crate::color_names::*;
use crate::quiet::*;
use crate::shape::*;
use crate::util::*;
use noise::Perlin;
pub struct SandBox {
    xy: Point,
    wh: Point,
    bg_color: RGBA,
    color1: RGBA,
    color2: RGBA,
    scales: f32,
}

impl SandBox {
    pub fn new(xy: Point, wh: Point) -> Self {
        Self {
            xy,
            wh,
            bg_color: WHITE,
            color1: BLACK,
            color2: BLACK,
            scales: 1.0,
        }
    }

    pub fn set_bg(mut self, color: RGBA) -> Self {
        self.bg_color = color;
        self
    }

    pub fn set_color1(mut self, color: RGBA) -> Self {
        self.color1 = color;
        self
    }

    pub fn set_color2(mut self, color: RGBA) -> Self {
        self.color2 = color;
        self
    }

    pub fn draw<T: Sketch>(&mut self, canvas: &mut T) {
        let noise_opts = NoiseOpts::default().width(self.wh.x).height(self.wh.y).scales(self.scales);
        let nf = Perlin::default();
        canvas.fill_rect(
            self.xy.x,
            self.xy.y,
            self.wh.x,
            self.wh.y,
            &Texture::solid_color(self.bg_color),
        );
        for i in 0..self.wh.x as u32 {
            let from = point2(self.xy.x + i as f32, self.xy.y);
            let to = point2(self.xy.x + i as f32, self.xy.y + self.wh.y);
            let alpha = map_range(noise2d(nf, &noise_opts, from.x, from.y), -1.0, 1.0, 0.0, 1.0);
            ShapeBuilder::new()
                .line(from, to)
                .stroke_color(self.color1.opacity(alpha))
                .build()
                .draw(canvas);
        }
        for i in 0..self.wh.y as u32 {
            let from = point2(self.xy.x, self.xy.y + i as f32);
            let to = point2(self.xy.x + self.wh.x, self.xy.y + i as f32);
            let alpha = map_range(noise2d(nf, &noise_opts, from.x, from.y), -1.0, 1.0, 0.0, 1.0);
            ShapeBuilder::new()
                .line(from, to)
                .stroke_color(self.color2.opacity(alpha))
                .build()
                .draw(canvas);
        }
    }
}
