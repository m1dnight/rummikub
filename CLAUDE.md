# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Run Commands

- `cargo build` - Build the project
- `cargo run` - Run the main executable
- `cargo test` - Run tests (if any exist)
- `cargo check` - Type check without building

## Architecture Overview

This is a Rust implementation of Rummikub game logic focused on tile placement optimization.

### Core Components

**structures.rs** - Defines the game data structures:
- `Tile` - Individual game tile with color and value
- `Color` - Four tile colors (Blue, Red, Orange, Black) with ANSI display formatting
- `Run` - Consecutive tiles of same color (e.g., Blue 3-4-5-6)
- `Group` - Same-value tiles of different colors (e.g., Red 7, Blue 7, Orange 7)
- `Board` - Contains all runs and groups currently on the table
- `Player` - Contains player's hand of tiles

**mutations.rs** - Implements tile placement logic:
- `place_tile()` - Attempts to place a single tile by extending existing runs
- `place_tiles()` - Places multiple tiles recursively
- `place_tiles_best()` - Finds optimal tile placement using permutations (via itertools crate)

**main.rs** - Entry point with test scenarios demonstrating tile placement on a sample board.


### Key Design Patterns

The codebase uses functional-style mutation with immutable data structures - functions return new `Board` instances rather than modifying in place. The `place_tiles_best` function explores all permutations to find the optimal tile placement strategy.

### Dependencies

- `itertools = "0.14.0"` - Used for generating permutations in placement optimization