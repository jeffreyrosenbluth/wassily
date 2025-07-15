#![doc(
    html_logo_url = "https://raw.githubusercontent.com/jeffreyrosenbluth/wassily/main/assets/logo.png"
)]
/*!
**Wassily** is both an API and set of tools for creating generative 2D art. It allows you to create
images that can easily be scaled to any size without loss of quality. It is useful for images that
are meant to be printed at large sizes or displayed on screen.  Included are many generative art
algorithms and utilities for dealing with colors and noise.

## Architecture

Wassily is built as a modular system comprising several focused crates:

- **[`wassily-core`](crate::core)**: Core rendering infrastructure including canvas, shapes, and drawing primitives
- **[`wassily-color`](crate::color)**: Color utilities, palettes, and color space operations
- **[`wassily-noise`](crate::noise)**: Noise generation functions and utilities optimized for generative art
- **[`wassily-geometry`](crate::geometry)**: Geometric operations, curves, spatial data structures, and subdivision algorithms
- **[`wassily-effects`](crate::effects)**: Visual effects, textures, and procedural generation tools
- **[`wassily-algorithms`](crate::algorithms)**: Specialized rendering algorithms and advanced techniques

You can use individual crates for specific functionality, or import everything through the unified 
[`prelude`] for maximum convenience.

## Getting Started

Add wassily to your `Cargo.toml`:

```toml
[dependencies]
wassily = "0.2.0"
```

Then import the prelude to get started:

```rust
use wassily::prelude::*;

fn main() {
    // Create a canvas
    let mut canvas = Canvas::new(800, 600);
    
    // Set background color
    canvas.fill(*WHITE);
    
    // Draw a simple shape
    Shape::new()
        .circle(center(800, 600), 100.0)
        .fill_color(*BLUE)
        .stroke_color(*BLACK)
        .stroke_weight(2.0)
        .draw(&mut canvas);
    
    // Save the result
    canvas.save_png("output.png");
}
```

## Features

- **Vector Graphics**: High-quality scalable graphics using tiny-skia
- **Rich Color System**: Advanced color spaces, palettes, and color manipulation
- **Noise Generation**: Comprehensive noise functions optimized for generative art
- **Geometric Tools**: Curves, subdivision, spatial data structures, and geometric algorithms
- **Visual Effects**: Textures, grain, distortion, and procedural effects
- **Specialized Algorithms**: Advanced rendering techniques and artistic algorithms

## Usage Patterns

### All-in-One Import (Recommended)
```rust
use wassily::prelude::*;
// Everything you need is now available
```

### Selective Module Usage
```rust
use wassily::core::{Canvas, Shape};
use wassily::color::*;
use wassily::noise::*;
```

### Individual Crate Usage
```rust
// For more focused dependencies
use wassily_core::Canvas;
use wassily_color::rand_okhsl;
```

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
    let mut rng = SmallRng::seed_from_u64(42);

    // Set the background color.
    canvas.fill(*WHITESMOKE);

    for y in 0..ROWS {
        for x in 0..COLS {
            let size = SQUARESIZE as f32;
            let half_size = size / 2.0;

            // Use factor to increase random displacement and angle as we move
            // down the rows.
            let factor = y as f32 / ROWS as f32;

            // Randomly displace the square.
            let x_offset = factor * rng.gen_range(-0.5..0.5);
            let y_offset = factor * rng.gen_range(-0.5..0.5);

            // Calculate the position of the center of the square.
            let pos_x = (x * SQUARESIZE + MARGIN) as f32 + x_offset + half_size;
            let pos_y = (y * SQUARESIZE + MARGIN) as f32 + y_offset + half_size;

            // A random angle to rotate the square.
            let angle = factor * rng.gen_range(-45.0..45.0);

            // Create a rotation transform.
            let rotation = Transform::from_rotate_at(angle, pos_x as f32, pos_y as f32);

            // Choose a random color using Okhsl color space, and set it's opacity to 0.75.
            let mut fill = rand_okhsl(&mut rng);
            fill.set_alpha(0.85);

            // Draw the rectangle.
            Shape::new()
                .rect_cwh(pt(pos_x, pos_y), pt(size, size))
                .fill_color(fill)
                .no_stroke()
                .transform(&rotation)
                .draw(&mut canvas);

            // Draw a random pearl shape in the middle of the square 25% of the time.
            if rng.gen_bool(0.25) {
                Shape::new()
                    // pearl(center, width, height, sides, chaiken iterations, rng)
                    .pearl(pt(pos_x, pos_y), size / 5.0, size / 5.0, 8, 4, &mut rng)
                    .fill_color(rand_okhsl(&mut rng))
                    .no_stroke()
                    .draw(&mut canvas);
            }
        }
    }
    canvas.save_png("./outputs/schotter.png");
}
```

## More Examples

For additional examples demonstrating various features:

- **Color Scaling**: Demonstrates color palette generation and noise-based coloring
- **Geometric Shapes**: Shows various shape creation and transformation techniques  
- **Noise Patterns**: Explores different noise functions and their artistic applications
- **Visual Effects**: Demonstrates texture generation and visual effects
- **Advanced Algorithms**: Showcases specialized rendering techniques

Visit the [examples directory](https://github.com/jeffreyrosenbluth/wassily/tree/main/examples) 
in the repository for complete source code of these and other examples.

 */

// Re-export all wassily crates
pub use wassily_algorithms as algorithms;
pub use wassily_color as color;
pub use wassily_core as core;
pub use wassily_effects as effects;
pub use wassily_geometry as geometry;
pub use wassily_noise as noise;

pub mod prelude;
