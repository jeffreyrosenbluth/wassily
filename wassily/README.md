# Wassily

**Wassily** is both an API and set of tools for creating generative 2D art. It allows you to create images that can easily be scaled to any size without loss of quality, perfect for printed art or high-resolution displays.

## Architecture

Wassily is built as a modular system comprising several focused crates:

- **`wassily-core`**: Core rendering infrastructure including canvas, shapes, and drawing primitives
- **`wassily-color`**: Color utilities, palettes, and color space operations  
- **`wassily-noise`**: Noise generation functions and utilities optimized for generative art
- **`wassily-geometry`**: Geometric operations, curves, spatial data structures, and subdivision algorithms
- **`wassily-effects`**: Visual effects, textures, and procedural generation tools
- **`wassily-algorithms`**: Specialized rendering algorithms and advanced techniques

## Quick Start

```rust
use wassily::prelude::*;

fn main() {
    // Create a canvas
    let mut canvas = Canvas::new(800, 600);
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

## Examples

The crate includes numerous examples demonstrating various techniques. Run them with:

```bash
cargo run --example schotter
cargo run --example sphere
cargo run --example saturn
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.