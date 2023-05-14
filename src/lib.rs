#![doc(
    html_logo_url = "https://raw.githubusercontent.com/jeffreyrosenbluth/wassily/main/assets/logo.png"
)]
/*!
Wassily is both an API and set of tools for creating generative 2D art. It allows you to create
images that can easily be scaled to any size without loss of quality. It is useful for iamges that
are meant to be printed at large sizes or displayed on screen.  Includee are many generative art
algorithms and utilities for dealing with colors and noise.
## Example

<img src="https://raw.githubusercontent.com/jeffreyrosenbluth/wassily/main/assets/schotter.png" alt="Schotter image" width="500" />

 ```rust
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

    // Create a palette to generate the random colors.
    let mut palette = Palette::default();

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
            // Lab color space.
            let mut fill = palette.rand_laba();

            // Increase the opacity by 0.1 so that all rectangles are visible.
            let alpha = fill.alpha() + 0.2;
            fill.set_alpha(alpha);

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

 */

pub mod canvas;
pub mod color_names;
pub mod flow;
pub mod grain;
pub mod grid;
pub mod kolor;
pub mod lines;
pub mod math;
pub mod matrix;
pub mod noises;
pub mod okcolor;
pub mod point_map;
pub mod prelude;
pub mod rectangles;
pub mod shape;
pub mod sketch;
pub mod sphere;
pub mod stipple;
pub mod subdivision;
pub mod textures;
pub mod util;
pub mod warp;
