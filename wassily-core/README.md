# Wassily Core

Core rendering infrastructure for the wassily generative art library.

This crate provides the fundamental building blocks for creating 2D generative art:
canvas management, shape rendering, drawing primitives, and utility functions.

## Features

- **Canvas**: High-performance 2D rendering surface with PNG export
- **Shape**: Flexible shape builder for paths, rectangles, circles, and polygons  
- **Points**: 2D point operations with vector algebra
- **Utilities**: Color blending, transformations, and helper functions

## Quick Start

```rust
use wassily_core::*;

// Create a canvas
let mut canvas = Canvas::new(800, 600);
canvas.fill(Color::WHITE);

// Draw a circle
Shape::new()
    .circle(pt(400.0, 300.0), 100.0)
    .fill_color(Color::BLUE)
    .stroke_color(Color::BLACK)
    .stroke_weight(2.0)
    .draw(&mut canvas);

// Save as PNG
canvas.save_png("artwork.png");
```

## Integration

This crate is part of the [wassily](https://crates.io/crates/wassily) ecosystem. For the complete generative art toolkit, use the main `wassily` crate which re-exports all functionality.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.