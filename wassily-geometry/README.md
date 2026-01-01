# Wassily Geometry

Geometric operations and spatial data structures for the wassily generative art library.

This crate provides sophisticated tools for working with curves, grids, spatial data structures, and subdivision algorithms essential for creating complex geometric patterns and algorithmic art.

## Features

- **Parametric Curves**: Arc-length parameterized paths with interpolation and refinement
- **Spatial Data Structures**: Efficient quadtree implementation for point queries
- **Subdivision Algorithms**: Recursive quadrilateral and triangle subdivision with noise
- **Matrix Operations**: Generic matrix with convolution, multiplication, and linear algebra
- **Specialized Lines**: Artistic line effects including fade, sand, and stipple textures
- **Grid Systems**: Structured point grids with quadrilateral extraction

## Quick Start

```rust
use wassily_geometry::*;
use wassily_core::points::pt;

// Create a parametric curve
let points = vec![pt(0.0, 0.0), pt(100.0, 50.0), pt(200.0, 0.0)];
let curve = ParametricPath::new(points);
let midpoint = curve.point_at(0.5);  // Get point at 50% along curve

// Subdivision example
let quad = Quad::new(pt(0.0, 0.0), pt(0.0, 100.0), pt(100.0, 100.0), pt(100.0, 0.0));
let (left, right) = quad.split_v(0.3, 0.7);

// Quadtree for spatial queries
let qtree = QNode::new(vec![pt(10.0, 10.0), pt(90.0, 90.0)], pt(0.0, 0.0), pt(100.0, 100.0));
let nearby = qtree.points_in_circle(pt(0.0, 0.0), pt(100.0, 100.0), pt(50.0, 50.0), 25.0);
```

## Applications

- **Algorithmic Art**: Recursive subdivision patterns and fractal-like structures
- **Mesh Generation**: Quadrilateral and triangular mesh creation with organic distortion
- **Spatial Queries**: Efficient point location and nearest neighbor operations
- **Curve Manipulation**: Smooth interpolation and adaptive curve approximation

## Integration

This crate is part of the [wassily](https://crates.io/crates/wassily) ecosystem. For the complete generative art toolkit, use the main `wassily` crate which re-exports all functionality.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.