//! Defines all configuration options for the renderer.

use crate::vec::Vector;

/// List of possible formats for render output. PNG is recommended if no other format is needed.
pub enum Format {
    Ppm,
    Png,
}

impl Default for Format {
    fn default() -> Format {
        Format::Png
    }
}

/// Contains information regarding the output format of the image.
#[derive(Default)]
pub struct Output {
    /// Format of the resulting image.
    pub format: Format,
    /// Width of the resulting image.
    pub width: usize,
    /// Height of the resulting image.
    pub height: usize,
    /// Number of bits per channel in the resulting image.
    pub bits: usize,
}

/// Contains parameters for how to render the scene.
#[derive(Default)]
pub struct Render {
    /// Maximum number of reflections possible for ray.
    pub max_reflections: u32,

    /// Number of threads to use to render everything.
    pub threads: usize,
}

/// Contains information on scene information.
#[derive(Default)]
pub struct World {
    /// Position of the camera in the scene.
    pub cam_pos: Vector,
    pub background: Vector,
}

/// Overall struct holding all configuration parameters.
#[derive(Default)]
pub struct Config {
    /// Controls output parameters.
    pub output: Output,
    /// Controls render parameters.
    pub render: Render,
    /// Controls global scene parameters in the render.
    pub world: World,
}
