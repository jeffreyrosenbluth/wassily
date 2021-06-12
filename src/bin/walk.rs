use wassily::prelude::*;
use wassily::skia::Canvas;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 900;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    canvas.fill(WHITE);
    let mut sand = SandPainter::new(BLACK, 64, 0.0, 30.0);
    for x in 0..1200 {
        let p = point2(x as f32, 600.0);
        sand.render(&mut canvas, p)
    }

    canvas.save("walk.png");
}

struct SandPainter {
    c: RGBA,
    g: f32,
    rando: Rand,
    grains: u32,
    dx: f32,
    dy: f32,
}

impl SandPainter {
    fn new(c: RGBA, grains: u32, dx: f32, dy: f32) -> Self {
        let mut rando = Rand::new(3);
        let g = rando.rand_range(0.01, 0.1);
        Self {
            c,
            g,
            rando,
            grains,
            dx,
            dy,
        }
    }

    fn render(&mut self, canvas: &mut Canvas, p: Point) {
        self.g += self.rando.rand_range(-0.05, 0.05);
        // clamp g to 0..1 with a 0.05 in the opposite direction
        if self.g < 0.0 {
            self.g = 0.05
        }
        if self.g > 1.0 {
            self.g = 0.95
        }
        let w = self.g / (self.grains - 1) as f32;
        let mut delta = 1.0;
        for i in 0..self.grains {
            let a = 0.1 - i as f32 / (10.0 * self.grains as f32);
            let x = p.x + delta * self.dx * (i as f32 * w);
            let y = p.y + delta * self.dy * (i as f32 * w);
            delta *= -1.0;
            pixel(x, y, self.c.set_opacity(a), canvas);
        }
    }
}
