# oxide

A simple CPU renderer in Rust based on [Computer Graphics from Scratch](https://gabrielgambetta.com/computer-graphics-from-scratch).

## Usage
Run `$ oxide test_scene.toml` to render a test scene. The resulting render will be named `test_scene.ppm`.

## Features
- .ppm output
- Phong lighting (ambient, diffuse, and specular lighting)
- Directional, point, and ambient lights
- Reflections of arbitrary depth
- Support for spheres and planes
- Controllable through a TOML configuration file
- Parallel execution

Example renders over time are shown in the `outputs` directory.

## TODOs
- MSAA
- refraction
- faster execution
  - cross platform SIMD is in nightly
  - platform specific SIMD requires macro stuff and basically having two copies of vec.rs
