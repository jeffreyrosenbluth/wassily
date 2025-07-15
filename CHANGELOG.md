# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-07-15

### Added
- **Modular Architecture**: Split wassily into 7 focused crates:
  - `wassily-core`: Core rendering infrastructure (canvas, shape, points, util)
  - `wassily-color`: Color utilities and palette management
  - `wassily-noise`: Noise generation and utilities
  - `wassily-geometry`: Geometric operations and spatial data structures
  - `wassily-effects`: Visual effects and procedural textures
  - `wassily-algorithms`: Specialized rendering algorithms
  - `wassily`: Main integration crate that re-exports all functionality

- **Workspace Configuration**: Centralized dependency management across all crates
- **Backward Compatibility**: Unified prelude maintains full API compatibility
- **Organized Examples**: All example outputs now saved to `outputs/` directory

### Changed
- **Dependencies Updated**:
  - `rand` upgraded to 0.9.1 
  - `rand_distr` upgraded to 0.5.1
  - `itertools` upgraded to 0.14.0
- **API Improvements**: Fixed deprecated `gen_range` calls throughout codebase
- **File Organization**: Cleaned up project structure, removed duplicate files

### Fixed
- **Saturn Example**: Resolved Standard distribution import conflict
- **Doc Tests**: Fixed import issues in curves.rs and other modules
- **Test Infrastructure**: Updated all tests to work with new modular structure

### Technical Details
- Each sub-crate can be used independently or together via the main `wassily` crate
- Workspace dependencies ensure version consistency across all crates  
- Path-based dependencies with version specifications for local development
- All examples verified to work with new modular architecture

### Migration Guide
For users upgrading from 0.1.0:
- No code changes required - the `wassily::prelude::*` import continues to work
- All existing functionality remains available at the same API locations
- Optional: Individual crates can now be imported for specific functionality

## [0.1.0] - Previous Release

Initial release with monolithic structure containing:
- Canvas and shape rendering
- Color utilities and palettes
- Noise generation functions
- Geometric operations
- Visual effects and textures
- Specialized algorithms
- Example generative art programs