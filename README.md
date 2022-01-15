# oxide

A simple CPU renderer in Rust based on [Computer Graphics from Scratch](https://gabrielgambetta.com/computer-graphics-from-scratch).

## Usage
Run `$ oxide test_scene.toml` to render a test scene. The resulting render will be named `<scene config name>.ppm`.

## Features
- .ppm output
- Phong lighting (ambient, diffuse, and specular lighting)
- Directional, point, and ambient lights
- Reflections of arbitrary depth
- Controllable via the `scene.toml` file

Example renders over time are shown in the `outputs` directory.
