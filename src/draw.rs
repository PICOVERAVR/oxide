//! Contains functions for manipulating the final image.

use crate::mat::Matrix;
use crate::vec::*;

/// An RGB type with 8 bits per channel.
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Checks if a pixel can be drawn on the current canvas. Returns the index to write to if the check passes.
fn check_pixel(ppm: &Matrix<Color>, pixel: (i32, i32)) -> Option<usize> {
    // convert bounds from [-n/2, n/2] to [0, n]
    let ax = pixel.0 + ppm.rlen as i32 / 2;
    let ay = -pixel.1 + ppm.clen as i32 / 2; // y direction needs to be flipped because the canvas y direction goes top to bottom

    if ay < 0 || ay >= ppm.clen as i32 {
        return None;
    }

    if ax < 0 || ax >= ppm.rlen as i32 {
        return None;
    }

    Some(ay as usize * ppm.rlen + ax as usize)
}

/// Draws a pixel on `ppm`.
/// `pixel` contains coordinates going from `-ppm.rlen/2` to `ppm.rlen/2` and `-ppm.clen/2` to `ppm.clen/2` respectively.
/// Calls where `pixel.0` or `pixel.1` maps to a value outside the corresponding limit in `ppm` will be silently ignored.
pub fn draw_pixel(ppm: &mut Matrix<Color>, pixel: (i32, i32), color: Color) {
    if let Some(idx) = check_pixel(ppm, pixel) {
        ppm.mat[idx] = color;
    }
}

/// Maps a float vector to a concrete color type.
pub fn map_color(c: Vector) -> Color {
    Color {
        r: (c.get()[0] * u8::max_value() as f32) as u8,
        g: (c.get()[1] * u8::max_value() as f32) as u8,
        b: (c.get()[2] * u8::max_value() as f32) as u8,
    }
}

/*
/// Draw a line from start to end using Bresenham's line algorithm.
pub fn draw_line(buf: &mut Vec<Color>, rlen: usize, start: (i32, i32), end: (i32, i32), color: Color) {
    let dx = i32::abs(end.0 - start.0);
    let dy = -i32::abs(end.1 - start.1);

    let sx = if start.0 < end.0 {
        1
    } else {
        -1
    };

    let sy = if start.1 < end.1 {
        1
    } else {
        -1
    };

    let mut err = dx + dy;
    let mut x = start.0;
    let mut y = start.1;

    loop {
        let cidx = y as usize * rlen + x as usize;

        buf[cidx] = color;

        if x == end.0 && y == end.1 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }

        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}
*/
