# Wassily Effects

Visual effects and procedural textures for the wassily generative art library.

This crate provides tools for creating visual effects, procedural textures, and advanced rendering techniques essential for adding polish and artistic flair to generative art projects.

## Features

- **Procedural Textures**: Algorithmic texture generation (stipple, marble, wood, granite)
- **Film Grain**: Realistic grain effects that simulate film photography
- **Low-Discrepancy Sampling**: Superior point distributions for stippling and sampling
- **Domain Warping**: Advanced coordinate transformation for distortion effects
- **Pattern Integration**: Seamless integration with wassily's paint system

## Quick Start

```rust
use wassily_effects::*;
use wassily_core::*;

// Create film grain effect
let grain = Grain::new(800, 600, 0.01, 0.5);
let mut canvas = Canvas::new(800, 600);
canvas.fill(*BLUE);
grain.canvas_grain(&mut canvas);

// Generate low-discrepancy points for stippling  
let points = halton_2d(1000, 800.0, 600.0);
for point in points {
    Shape::new()
        .circle(point, 2.0)
        .fill_color(*BLACK)
        .draw(&mut canvas);
}

// Create procedural texture
let texture = marble(256, 256, *BEIGE, *BROWN, 0.02, 42);
```

## Effect Types

- **Grain Effects**: Film grain, noise overlays, and texture enhancement
- **Sampling Patterns**: Halton sequences, Poisson disk, and uniform distributions  
- **Procedural Textures**: Wood, marble, granite, foam, and stipple patterns
- **Domain Warping**: Non-linear coordinate transformations and distortions

## Applications

- **Film Photography Simulation**: Add authentic grain and texture effects
- **Texture Generation**: Create organic, natural-looking surface patterns
- **Artistic Enhancement**: Add visual interest and professional polish
- **Pattern Creation**: Generate complex, mathematically-based patterns

## Integration

This crate is part of the [wassily](https://crates.io/crates/wassily) ecosystem. For the complete generative art toolkit, use the main `wassily` crate which re-exports all functionality.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.