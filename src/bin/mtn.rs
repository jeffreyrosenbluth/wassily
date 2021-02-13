use colorous;
use noise::NoiseFn;
use rand_distr::{Distribution, Geometric};
use rand::{thread_rng, Rng};
use tiny_skia::*;
use wassily::util::*;
use wassily::grid::*;
use wassily::shape::*;

const WIDTH: u32 = 8_000;
const HEIGHT: u32 = 6_000;

const GRID_SPACING: f32 = 6.0;
const LENGTH: usize = 1000;
const K: f64 = 0.025;
const LINES: usize = 700;
const STEP: f32 = 70.0;
fn main() {
    let w = WIDTH as f32;
    let h = HEIGHT as f32;
    let mut rng = thread_rng();

    let mut pixmap = Pixmap::new(WIDTH, HEIGHT).unwrap();
    let mut canvas = Canvas::from(pixmap.as_mut());

    let geo = Geometric::new(0.5).unwrap();
    let colors = colorous::REDS;
    let c = colors.eval_rational(2, 10);
    let kolor = Color::from_rgba8(c.r, c.g, c.b, 255);
    background(&mut canvas, WIDTH, HEIGHT, kolor);

    let nn = noise::BasicMulti::new();

    let grid = Grid::new(1.1 * w, 1.1 * h, GRID_SPACING, |x, y| {
        nn.get([K * x as f64, K * y as f64]) as f32 * TAU
    });

    for l in 0..LINES {
        let mut loc = pt2(-200.0, rng.gen_range(0..HEIGHT) as f32);
        let mut points = vec![];
        for _i in 0..LENGTH {
            points.push(loc);
            let angle = &grid.get(loc.x, loc.y);
            let angle = map_range(*angle, 0.0, TAU, 0.0, PI);
            loc.x += STEP * angle.cos();
            loc.y += STEP * angle.sin();
            if loc.x > w {
                break;
            };
        }
        let c = colors.eval_rational(l % 100, 100);
        let kolor = Color::from_rgba8(c.r, c.g, c.b, 255);
        let sw =  8.0 * geo.sample(&mut rand::thread_rng()) as f32;

        let poly = ShapeBuilder::new().points(&points).fill_color(kolor).stroke_color(Color::BLACK).stroke_weight(sw).build();
        poly.draw(&mut canvas);
    }
    pixmap.save_png("mtn.png").unwrap();
}
