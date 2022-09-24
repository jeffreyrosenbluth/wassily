use wassily::prelude::*;
const WIDTH: u32 = 1080;
const HEIGHT: u32 = 1080;

fn main() {
    let mut drawing = Drawing::new(WIDTH, HEIGHT, 1.5);
    let mut rng = SmallRng::seed_from_u64(23);
    let mut color_wheel = ColorWheel::new(&mut rng, 7, 3);
    for i in 0..drawing.pixmap_width() {
        for j in 0..drawing.pixmap_height() {
            let x = i as f32 / WIDTH as f32;
            let y = j as f32 / HEIGHT as f32;
            let c = color_wheel.get_color(&mut rng, x, y);
            drawing.pixmap_dot(i, j, c);
        }
    }
    drawing.save_png("./wheel6.png");
}
