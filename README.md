# Conway's Game of Life - Lab 2
## Author
David Dominguez - 23712

## GIF
![GIF-CGOL-DDOM](https://github.com/user-attachments/assets/5655a1a7-3180-4e15-8ff3-4bdab42b1487)


## Description
This project implements Conway's Game of Life, a cellular automaton devised by mathematician John Horton Conway. The simulation follows four simple rules that determine the evolution of cells across generations:

1. Any live cell with fewer than two live neighbors dies (underpopulation)
2. Any live cell with two or three live neighbors lives on to the next generation
3. Any live cell with more than three live neighbors dies (overpopulation)
4. Any dead cell with exactly three live neighbors becomes a live cell (reproduction)

## Features
- Custom framebuffer implementation for rendering
- Support for multiple classic Game of Life patterns:
  - Gliders
  - Lightweight spaceships (LWSS)
  - Pulsars
  - Pentadecathlons
  - Various still lifes (blocks, boats, beehives, etc.)
- Adjustable simulation speed
- Window scaling to maintain aspect ratio

## Project Structure
```
daviddominguez-11-lab2_conwaysgameoflife/
├── README.md
└── lab2_cgol/
    ├── Cargo.toml
    └── src/
        ├── framebuffer.rs
        ├── line.rs
        └── main.rs
```

## File Descriptions

### `framebuffer.rs`
Implements a custom framebuffer with:
- Color buffer management
- Pixel manipulation (point drawing)
- Buffer clearing
- Rendering to window with proper scaling
- Image export capability

### `line.rs`
Contains Bresenham's line algorithm implementation (currently unused in main simulation)

### `main.rs`
Main program logic including:
- Pattern definitions (gliders, spaceships, pulsars, etc.)
- Game of Life rules implementation
- Grid management
- Main simulation loop

## Patterns Implemented
The simulation includes several well-known Game of Life patterns:
- **Gliders**: Small, moving patterns that travel diagonally
- **Lightweight Spaceships (LWSS)**: Larger moving patterns
- **Pulsars**: Oscillators with period 3
- **Pentadecathlons**: Oscillators with period 15
- **Still lifes**: Stable patterns that don't change
  - Blocks
  - Tubs
  - Boats
  - Loaves
  - Beehives

## Building and Running
1. Ensure you have Rust and Cargo installed
2. Clone the repository
3. Navigate to the project directory:
   ```bash
   cd daviddominguez-11-lab2_conwaysgameoflife/lab2_cgol
   ```
4. Build and run:
   ```bash
   cargo run --release
   ```

## Controls
- The simulation runs automatically
- Close the window to exit

## Dependencies
- [raylib](https://www.raylib.com/) (v5.5.1) for rendering

## Configuration
The simulation can be adjusted by modifying:
- Window size in `main.rs`
- Framebuffer size in `main.rs`
- Simulation speed (change the duration in `thread::sleep`)

## Performance Notes
The project is configured with optimizations for development builds:
```toml
[profile.dev]
opt-level = 3
debug = false
```

For best performance, run with `--release` flag.

## Author
David Dominguez - 23712
