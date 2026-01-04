# Volatility Surface

Demonstrates 3D perspective projection in the terminal using Braille canvas for high-resolution rendering.

To run this demo:

```shell
cargo run -p volatility-surface
```

## What it shows

This example visualizes an implied volatility surface as an interactive 3D wireframe. It demonstrates:

- 3D-to-2D perspective projection with rotation matrices
- Braille canvas rendering for smooth curves (8x resolution vs blocks)
- Real-time animation and interactive controls

## How it works

Objects further away appear smaller—that's perspective. The math is simple: divide x and y coordinates by distance. Add rotation matrices to spin the object in 3D space, and you can explore it from any angle.

The result: a 500-point surface rendered at 30fps using basic trigonometry and perspective division.

## Controls

- **Arrow Keys** - Rotate the surface
- **Z/X** - Zoom in/out
- **P** - Cycle color palettes
- **Space** - Pause/resume
- **Q** - Quit
