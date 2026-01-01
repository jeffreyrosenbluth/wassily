# Wassily Color

Color utilities and palette management for the wassily generative art library.

This crate provides comprehensive color manipulation tools including color space conversions, palette extraction from images, procedural color generation, and advanced color operations specifically designed for generative art applications.

## Features

- **Rich Color Spaces**: Support for RGB, HSL, HSV, Okhsl, Lab, and many more
- **Palette Management**: Create, manipulate, and extract color palettes
- **Named Colors**: Complete set of HTML/CSS color names
- **Procedural Generation**: Generate random colors in perceptually uniform spaces
- **Image Extraction**: Extract dominant colors from images
- **Advanced Operations**: Color blending, scaling, and fourier-based palettes

## Quick Start

```rust
use wassily_color::*;

// Use named colors
let blue = *CORNFLOWERBLUE;
let red = *CRIMSON;

// Create random colors in perceptually uniform space
let mut rng = rand::thread_rng();
let random_color = rand_okhsl(&mut rng);

// Create a color palette
let palette = Palette::new(vec![blue, red, random_color]);

// Create a color scale for smooth transitions
let scale = ColorScale::new(blue, red, *WHITE, *BLACK, random_color);
let interpolated = scale.get_color(0.5);
```

## Color Spaces

This crate emphasizes modern, perceptually uniform color spaces like Okhsl/Okhsv and Lab/Lch for better color manipulation in generative art applications.

## Integration

This crate is part of the [wassily](https://crates.io/crates/wassily) ecosystem. For the complete generative art toolkit, use the main `wassily` crate which re-exports all functionality.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.