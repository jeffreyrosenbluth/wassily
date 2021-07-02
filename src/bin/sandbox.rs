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
    color: RGBA,
}

impl SandBox {
    pub fn new(xy: Point, wh: Point) -> Self {
        Self {
            xy,
            wh,
            color: BLACK,
        }
    }

    pub fn color(mut self, color: RGBA) -> Self {
        self.color = color;
        self
    }

    pub fn draw_vertical<T: Sketch>(&mut self, canvas: &mut T) {
        let ns = Noise::<_, 2>::new(self.wh.x, self.wh.y, Perlin::default())
            .set_noise_scales(SCALE, SCALE);
        for i in 0..self.wh.x as u32 {
            let from = point2(self.xy.x + i as f32, self.xy.y);
            let to = point2(self.xy.x + i as f32, self.xy.y + self.wh.y);
            let mut alpha = map_range(ns.noise(from.x, from.y), -1.0, 1.0, 0.2, 0.9);
            alpha = alpha.clamp(0.2, 0.8);
            ShapeBuilder::new()
                .line(from, to)
                .stroke_color(self.color.set_opacity(alpha))
                .build()
                .draw(canvas);
        }
    }

    pub fn draw_horizontal<T: Sketch>(&mut self, canvas: &mut T) {
        let ns = Noise::<_, 2>::new(self.wh.x, self.wh.y, Perlin::default())
            .set_noise_scales(SCALE, SCALE);
        for i in 0..self.wh.y as u32 {
            let from = point2(self.xy.x, self.xy.y + i as f32);
            let to = point2(self.xy.x + self.wh.x, self.xy.y + i as f32);
            let mut alpha = map_range(ns.noise(from.x, from.y), -1.0, 1.0, 0.2, 0.9);
            alpha = alpha.clamp(0.2, 0.8);
            ShapeBuilder::new()
                .line(from, to)
                .stroke_color(self.color.set_opacity(alpha))
                .build()
                .draw(canvas);
        }
    }
}

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let mut palette = Palette::with_img(file_path("fruit.png"), 144);
    palette.sort_by_chroma();
    palette.colors.reverse();
    canvas.fill(RGBA::rgb8(25, 25, 25));
    let mut x = 0.0;
    let mut y = 0.0;
    while x <= WIDTH as f32 - SIZE {
        while y <= HEIGHT as f32 - SIZE {
            let mut sq = SandBox::new(point2(x, y), point2(SIZE, SIZE)).color(palette.next());
            if (x + y) as u32 / SIZE as u32 % 2 == 0 {
                sq.draw_vertical(&mut canvas)
            } else {
                sq.draw_horizontal(&mut canvas)
            }
            y += SIZE;
        }
        y = 0.0;
        x += SIZE;
    }
    canvas.save("sandbox.png");
}
