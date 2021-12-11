#![allow(unused_imports)]
#![allow(dead_code)]

//use vec::*; // "vec" is now valid in the main.rs scope, used when declaring types or concrete functions
mod vec; // triggers a load of the vec module
mod vec_test;

mod mat;
mod mat_test;

mod draw;
use draw::*;

use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let width = 1920;
    let height = 1080;
    let bits = 8;

    let header = format!("P6 {} {} {}\n", width, height, 2u32.pow(bits) - 1);
    
    let mut file = File::create("output.ppm")?; // "?" unpacks the result if Ok and returns the error if not
    file.write_all(header.as_bytes())?;

    let pixels = width * height;
    let mut ppm: Vec<Pixel> = Vec::with_capacity(pixels);

    for _y in 0..height {
        for _x in 0..width {
            ppm.push(Pixel {r: 0, g: 0, b: 0});
        }
    }

    let color = Pixel { r: 255, g: 255, b: 255 };
    draw_line(&mut ppm, width, (0, 0), (1919, 1079), color);

    // TODO: 2nd byte copy is pretty wasteful, can get around this with an unsafe block?
    let bytes = pixels * 3;
    let mut ppm_bytes: Vec<u8> = Vec::with_capacity(bytes);

    for pixel in ppm {
        ppm_bytes.push(pixel.r);
        ppm_bytes.push(pixel.g);
        ppm_bytes.push(pixel.b);
    }

    file.write_all(&ppm_bytes)?;

    Ok(())
}
