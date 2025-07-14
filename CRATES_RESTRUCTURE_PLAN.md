# Wassily Crates Restructure Plan

## Overview

This document outlines the plan to restructure the wassily project from a single crate into multiple focused crates, making it more modular and easier to publish to crates.io.

## Current Status

- **Dependencies Updated**: All dependencies have been updated to their latest compatible versions
- **Codebase Analyzed**: Module relationships and dependencies have been mapped
- **Ready for Restructure**: Clear separation points identified

## Proposed Crate Structure

### 1. `wassily-core` - Core Rendering Infrastructure
**Purpose**: Essential drawing and rendering functionality  
**Size**: ~4 modules  
**Modules**:
- `canvas.rs` - Core drawing canvas (wraps tiny-skia)
- `shape.rs` - Shape builder system for drawing primitives
- `points.rs` - 2D/3D point utilities and algebra traits
- `util.rs` - Mathematical utilities, Chaiken smoothing, bounding boxes

**Dependencies**: `tiny-skia`, `num-traits`, `num-complex`, `image`, `png`  
**Internal Dependencies**: None (foundation crate)

### 2. `wassily-color` - Color Management
**Purpose**: Color utilities, palettes, and color space operations  
**Size**: ~3 modules  
**Modules**:
- `kolor.rs` - Color utilities, conversions, and traits
- `color_names.rs` - HTML color name constants
- `color_palette.rs` - Color palette extraction and management

**Dependencies**: `palette`, `image`, `color-thief`, `once_cell`  
**Internal Dependencies**: None (standalone color utilities)

### 3. `wassily-noise` - Noise Generation
**Purpose**: All noise-related functionality  
**Size**: ~6 modules  
**Modules**:
- `noises.rs` - Main noise utilities wrapper
- `noises/white.rs` - White noise and pseudorandom generators
- `noises/curl.rs` - Curl noise implementation
- `noises/gabor.rs` - Gabor noise patterns
- `noises/img_noise.rs` - Image-based noise generation
- `noises/sinusoid.rs` - Sinusoidal noise patterns

**Dependencies**: `noise`, `rand`, `rand_distr`  
**Internal Dependencies**: None (standalone noise utilities)

### 4. `wassily-geometry` - Geometric Operations
**Purpose**: Geometric algorithms and spatial data structures  
**Size**: ~6 modules  
**Modules**:
- `curves.rs` - Parametric paths and curve operations
- `lines.rs` - Specialized line drawing (FadeLine, SandLine, DotLine)
- `matrix.rs` - Generic matrix operations
- `subdivision.rs` - Quad and triangle subdivision algorithms
- `quadtree.rs` - Spatial data structure for point queries
- `grid.rs` - Grid of points as quadrilaterals

**Dependencies**: `rand`, `rand_distr`, `num-traits`  
**Internal Dependencies**: `wassily-core` (Point utilities), `wassily-noise` (some algorithms)

### 5. `wassily-effects` - Visual Effects and Textures
**Purpose**: Advanced visual effects and procedural textures  
**Size**: ~4 modules  
**Modules**:
- `textures.rs` - Pattern generation (stipple, marble, wood, granite)
- `grain.rs` - Grain effect generation
- `warp.rs` - Domain warping utilities
- `stipple.rs` - Low-discrepancy sampling (Halton, Poisson disk)

**Dependencies**: `noise`, `rand`, `image`  
**Internal Dependencies**: `wassily-core`, `wassily-noise`, `wassily-color`

### 6. `wassily-algorithms` - Specialized Algorithms
**Purpose**: Specialized rendering algorithms  
**Size**: ~2 modules  
**Modules**:
- `endo2d.rs` - Endomorphisms (unit square transformations)
- `sphere.rs` - 3D sphere rendering with lighting

**Dependencies**: None (just std)  
**Internal Dependencies**: `wassily-core`, `wassily-color`

### 7. `wassily` - Main Integration Crate
**Purpose**: Unified interface maintaining current API  
**Size**: ~2 modules  
**Modules**:
- `lib.rs` - Main library interface
- `prelude.rs` - Unified imports from all sub-crates

**Dependencies**: Re-exports from all sub-crates  
**Internal Dependencies**: All `wassily-*` crates

## Implementation Plan

### Phase 1: Dependency Updates ✅
- [x] Update all dependencies to latest versions
- [x] Verify project builds successfully
- [x] Run existing examples to ensure compatibility

### Phase 2: Core Infrastructure (Week 1)
- [ ] Create `wassily-core` crate workspace
- [ ] Move `canvas.rs`, `shape.rs`, `points.rs`, `util.rs`
- [ ] Configure dependencies and exports
- [ ] Test core functionality

### Phase 3: Standalone Utilities (Week 2)
- [ ] Create `wassily-color` crate
  - Move color-related modules
  - Ensure no dependencies on other wassily modules
- [ ] Create `wassily-noise` crate
  - Move noise generation modules
  - Keep as standalone utility crate

### Phase 4: Geometric Operations (Week 3)
- [ ] Create `wassily-geometry` crate
- [ ] Move geometric algorithms and data structures
- [ ] Update dependencies on `wassily-core` and `wassily-noise`

### Phase 5: Effects and Algorithms (Week 4)
- [ ] Create `wassily-effects` crate
- [ ] Create `wassily-algorithms` crate
- [ ] Establish proper dependency chains

### Phase 6: Integration (Week 5)
- [ ] Create main `wassily` crate
- [ ] Re-export all functionality through prelude
- [ ] Ensure backward compatibility
- [ ] Update documentation and examples

### Phase 7: Publishing Preparation (Week 6)
- [ ] Add proper Cargo.toml metadata for all crates
- [ ] Write individual README files for each crate
- [ ] Add comprehensive documentation
- [ ] Set up CI/CD for workspace
- [ ] Prepare for crates.io publication

## Benefits

### For Users
- **Modular Usage**: Include only needed functionality
- **Faster Compilation**: Smaller dependency graphs
- **Clearer Documentation**: Focused per-crate docs

### For Maintainers
- **Isolated Testing**: Test components independently
- **Focused Development**: Work on specific areas
- **Easier Debugging**: Smaller, focused codebases

### For the Ecosystem
- **Reusable Components**: Other crates can use specific parts
- **Clear Interfaces**: Well-defined boundaries between functionality
- **Standard Structure**: Follows Rust ecosystem patterns

## Workspace Configuration

The project will use a Cargo workspace structure:

```toml
[workspace]
members = [
    "wassily-core",
    "wassily-color", 
    "wassily-noise",
    "wassily-geometry",
    "wassily-effects",
    "wassily-algorithms",
    "wassily"
]
```

## Backward Compatibility

The main `wassily` crate will maintain the current API by re-exporting all functionality through its prelude. Existing code should continue to work without changes:

```rust
use wassily::prelude::*; // Still works as before
```

Advanced users can opt into more specific imports:

```rust
use wassily_core::{Canvas, Shape};
use wassily_color::ColorPalette;
use wassily_noise::NoiseBuilder;
```

## Risk Mitigation

- **Breaking Changes**: Main crate maintains current API
- **Dependency Issues**: Each crate has minimal, focused dependencies  
- **Testing**: Comprehensive test suite for each crate
- **Documentation**: Clear migration guide for advanced use cases

## Success Criteria

- [ ] All current examples build and run without modification
- [ ] Each sub-crate can be used independently
- [ ] Main `wassily` crate maintains full backward compatibility
- [ ] All crates successfully publish to crates.io
- [ ] Documentation is clear and comprehensive
- [ ] CI/CD pipeline validates all crates in workspace

## Timeline

**Total Duration**: 6 weeks  
**Start Date**: [To be determined]  
**Target Completion**: [6 weeks from start]

This restructure will position wassily as a mature, modular ecosystem of generative art tools while maintaining ease of use for beginners and providing advanced capabilities for power users.