[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue)](#license)
[![Crate](https://img.shields.io/crates/v/wassily.svg)](https://crates.io/crates/wassily)

# Wassily

**Wassily** is both an API and set of tools for creating generative 2D art. It allows you to create
images that can easily be scaled to any size without loss of quality. It is useful for images that
are meant to be printed at large sizes or displayed on screen. Included are many generative art
algorithms and utilities for dealing with colors and noise.

## Architecture

Wassily is built as a modular system comprising several focused crates:

- **`wassily-core`**: Core rendering infrastructure including canvas, shapes, and drawing primitives
- **`wassily-color`**: Color utilities, palettes, and color space operations
- **`wassily-noise`**: Noise generation functions and utilities optimized for generative art
- **`wassily-geometry`**: Geometric operations, curves, spatial data structures, and subdivision algorithms
- **`wassily-effects`**: Visual effects, textures, and procedural generation tools
- **`wassily-algorithms`**: Specialized rendering algorithms and advanced techniques

You can use individual crates for specific functionality, or import everything through the unified
prelude for maximum convenience.

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

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/new-noise-algorithm`)
3. Commit your Changes (`git commit -m 'Add Worley noise implementation'`)
4. Push to the Branch (`git push origin feature/new-noise-algorithm`)
5. Open a Pull Request

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be licensed under MIT or Apache-2.0 at the recipient's choice, without any additional terms or conditions.

Released under [MIT](/LICENSE) or [Apache-2.0](/LICENSE-APACHE) by [@jeffreyrosenbluth](https://github.com/jeffreyrosenbluth).
