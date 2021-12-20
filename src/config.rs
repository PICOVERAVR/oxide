// output options
// output image dimensions in pixels
pub const WIDTH: usize = 1200;
pub const HEIGHT: usize = 800;
pub const BITS: u32 = 8; // number of bits per pixel, values besides 8 are unsupported

// render options
pub const MAX_REFLECTIONS: u32 = 3; // maximum number of reflections off an object
pub const THREADS: usize = 2;

// scene options
pub const CAM_POS: (f32, f32, f32) = (0.0, 0.0, 0.0);
