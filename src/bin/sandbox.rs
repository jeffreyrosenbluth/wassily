use noise::Perlin;
use wassily::prelude::*;
use wassily::skia::Canvas;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 1000;
const SIZE: f32 = 100.0;
const SCALE: f32 = 1.0;

pub struct SandBox {
    xy: Point,
    wh: Point,
    bg_color: RGBA,
    color1: RGBA,
    color2: RGBA,
}

impl SandBox {
    pub fn new(xy: Point, wh: Point) -> Self {
        Self {
            xy,
            wh,
            bg_color: WHITE,
            color1: BLACK,
            color2: BLACK,
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
        let ns = Noise::<_, 2>::new(self.wh.x, self.wh.y, Perlin::default())
            .set_noise_scales(SCALE, SCALE);
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
            let alpha = map_range(ns.noise(from.x, from.y), -1.0, 1.0, 0.0, 1.0);
            ShapeBuilder::new()
                .line(from, to)
                .stroke_color(self.color1.set_opacity(alpha))
                .build()
                .draw(canvas);
        }
        for i in 0..self.wh.y as u32 {
            let from = point2(self.xy.x, self.xy.y + i as f32);
            let to = point2(self.xy.x + self.wh.x, self.xy.y + i as f32);
            let alpha = map_range(ns.noise(from.x, from.y), -1.0, 1.0, 0.0, 1.0);
            ShapeBuilder::new()
                .line(from, to)
                .stroke_color(self.color2.set_opacity(alpha))
                .build()
                .draw(canvas);
        }
    }
}

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let mut palette = Palette::with_img(file_path("orange.png"), 2000);
    palette.sort_by_chroma();
    let mut palette1 = palette.clone();
    palette1.rotate_hue(120.0);
    let mut palette2 = palette.clone();
    palette2.rotate_hue(240.0);
    canvas.fill(RGBA::rgb8(150, 150, 150));
    let mut x = 0.0;
    let mut y = 0.0;
    while x <= WIDTH as f32 - SIZE {
        while y <= HEIGHT as f32 - SIZE {
            let mut sq = SandBox::new(point2(x, y), point2(SIZE, SIZE))
                .set_color1(palette1.rand_color())
                .set_color2(palette2.rand_color());
            sq.draw(&mut canvas);
            y += SIZE;
        }
        y = 0.0;
        x += SIZE;
    }
    canvas.save("sandbox.png");
}
