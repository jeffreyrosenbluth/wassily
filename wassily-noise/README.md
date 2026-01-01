# Wassily Noise

Noise generation and utilities for the wassily generative art library.

This crate provides comprehensive noise functions optimized for generative art, including traditional Perlin and Simplex noise, specialized artistic noise generators, and utility functions for creating organic, natural-looking patterns.

## Features

- **Multiple Noise Types**: Perlin, Simplex, Fbm, Ridged, Worley, and more
- **2D/3D Support**: Functions for both 2D and 3D noise generation  
- **Artistic Utilities**: Gabor noise, curl noise, and sinusoidal patterns
- **Image-based Noise**: Generate noise textures and sample from images
- **Optimized Performance**: Efficient implementations for real-time generation
- **Flexible Scaling**: Easy coordinate transformation and scaling options

## Quick Start

```rust
use wassily_noise::*;
use noise::{Perlin, Fbm};

// Create noise generator
let noise_gen = Fbm::<Perlin>::default();
let opts = NoiseOpts::default().scales(0.01).factor(1.0);

// Generate 2D noise values
let value = noise2d(&noise_gen, &opts, 100.0, 50.0);
let normalized = noise2d_01(&noise_gen, &opts, 100.0, 50.0); // [0,1] range

// Generate noise with custom parameters
let custom_opts = NoiseOpts::new(800.0, 600.0, 0.005, 0.005, 0.005, 2.0);
let artistic_noise = noise2d(&noise_gen, &custom_opts, x, y);
```

## Noise Types

- **Perlin/Simplex**: Classic gradient noise for natural textures
- **Fbm**: Fractal Brownian motion for complex, layered patterns  
- **Ridged**: Sharp, ridge-like noise patterns
- **Worley**: Cellular/bubble-like patterns
- **Gabor**: Oriented texture synthesis
- **Curl**: Divergence-free vector fields

## Integration

This crate is part of the [wassily](https://crates.io/crates/wassily) ecosystem. For the complete generative art toolkit, use the main `wassily` crate which re-exports all functionality.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.