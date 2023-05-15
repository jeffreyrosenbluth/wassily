// Title: Schotter. Inspired by Georg Nees.
use wassily::prelude::*;

const ROWS: u32 = 12;
const COLS: u32 = 12;
const SQUARESIZE: u32 = 50;
const MARGIN: u32 = 35;
const WIDTH: u32 = COLS * SQUARESIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SQUARESIZE + 2 * MARGIN;

fn main() {
    // Create a canvas to draw on.
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    // We will need a random number generator. 'SmallRng' is reexported from
    // the 'rand' crate.
    let mut rng = SmallRng::from_entropy();

    // Set the background color.
    canvas.fill(*WHITESMOKE);

    for y in 0..ROWS {
        for x in 0..COLS {
            // Use factor to increase random displacement and angle as we move
            // down the rows.
            let factor = y as f32 / ROWS as f32;

            // Randomly displace the square.
            let x_offset = factor * rng.gen_range(-0.5..0.5);
            let y_offset = factor * rng.gen_range(-0.5..0.5);

            // Calculate the position of the rectangle.
            let pos_x = (x * SQUARESIZE + MARGIN) as f32 + x_offset;
            let pos_y = (y * SQUARESIZE + MARGIN) as f32 + y_offset;

            // A random angle to rotate the square.
            let angle = factor * rng.gen_range(-45.0..45.0);

            // Create a rotation transform.
            let rotation = Transform::from_rotate_at(angle, pos_x as f32, pos_y as f32);

            // Choose a random color from the palette with a random opacity using
            // Okhsl color space and set it's opacity to 0.75.
            let mut fill = rand_okhsl(&mut rng);
            fill.set_alpha(0.75);

            // Draw the rectangle.
            Shape::new()
                .rect_xywh(pt(pos_x, pos_y), pt(SQUARESIZE, SQUARESIZE))
                .fill_color(fill)
                .no_stroke()
                .transform(&rotation)
                .draw(&mut canvas);
        }
    }
    canvas.save_png("./schotter.png");
}
