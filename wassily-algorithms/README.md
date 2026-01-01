# Wassily Algorithms

Specialized rendering algorithms for the wassily generative art library.

This crate provides advanced algorithmic techniques for mathematical art generation, including 2D endomorphisms, 3D sphere rendering, and other specialized mathematical visualization algorithms.

## Features

- **2D Endomorphisms**: Mathematical transformations for creating complex, self-referential patterns
- **3D Sphere Rendering**: Algorithms for rendering spherical objects with proper lighting and shading
- **Mathematical Visualization**: Tools for visualizing complex mathematical functions and equations
- **Artistic Algorithms**: Specialized techniques for creating mathematically-inspired generative art

## Quick Start

```rust
use wassily_algorithms::*;
use wassily_core::*;

// Create a 2D endomorphism pattern
let mut canvas = Canvas::new(800, 800);
canvas.fill(*BLACK);

// Apply mathematical transformation
let endo = some_endomorphism_function;
render_endomorphism(&mut canvas, endo, iterations, color_scheme);

// Render a 3D sphere
let sphere_params = SphereParams {
    center: pt(400.0, 400.0),
    radius: 200.0,
    light_position: pt(600.0, 200.0),
    material_color: *BLUE,
};
render_sphere(&mut canvas, sphere_params);
```

## Algorithm Types

- **Endomorphisms**: 2D mathematical transformations and mappings
- **3D Rendering**: Sphere rendering with lighting and material properties
- **Mathematical Art**: Algorithms inspired by mathematical concepts and equations
- **Specialized Techniques**: Advanced rendering methods for specific artistic effects

## Applications

- **Mathematical Art**: Visualize complex mathematical concepts and equations
- **Academic Visualization**: Create educational materials for mathematical concepts
- **Artistic Exploration**: Experiment with mathematically-inspired generative techniques
- **3D Simulation**: Simple 3D rendering capabilities within a 2D framework

## Integration

This crate is part of the [wassily](https://crates.io/crates/wassily) ecosystem. For the complete generative art toolkit, use the main `wassily` crate which re-exports all functionality.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.