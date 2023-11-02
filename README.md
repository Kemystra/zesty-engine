# Zesty 3D Engine

*A minimal raster 3D engine for learning purposes*

The Zesty Engine is a CPU-based 3D engine that uses rasterization, a very common technique for realtime 3D rendering. The output is rendered through [Softbuffer](https://github.com/rust-windowing/softbuffer) and [winit](https://github.com/rust-windowing/winit).

## Aim

At the start of this project, the original aim was to produce a 3D engine based on rasterization, before moving on to ray-tracing. However, I might turn this into a game engine later with Python as its scripting language. This project is not intended for commercial use though, it is simply a learning project.

## Conventions Used
1. All coordinates are left-handed, with X-axis to the right, Y-axis going up, and Z-axis going forward (into the screen).
2. Following Wavefront OBJ convention, mesh triangle vertices are in counter-clockwise order.

## Acknowledgement

Most of the theories I got from [scratchapixel.com](https://www.scratchapixel.com/), go check them out!
