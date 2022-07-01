use rand::rngs::SmallRng;
use rand::SeedableRng;
use wassily::prelude::*;
const WIDTH: u32 = 7200;
const HEIGHT: u32 = 9600;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let mut rng = SmallRng::seed_from_u64(23);
    let mut color_wheel = ColorWheel::new(&mut rng, 7, 3);
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let x = i as f32 / WIDTH as f32;
            let y = j as f32 / HEIGHT as f32;
            let c = color_wheel.get_color(&mut rng, x, y);
            canvas.dot(i as f32, j as f32, c);
        }
    }
    canvas.save_png("./wheel4.png");
}
