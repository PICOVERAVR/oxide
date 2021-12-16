#![allow(unused_imports)]
#![allow(dead_code)]

//use vec::*; // "vec" is now valid in the main.rs scope, used when declaring types or concrete functions
mod vec; // triggers a load of the vec module
mod vec_test;
use vec::*;

mod mat;
mod mat_test;
use mat::Matrix;

mod draw;
use draw::*;

mod ray;
use ray::*;

mod render;
use render::*;

use std::fs::File;
use std::io::Write;
use std::time;

fn main() -> std::io::Result<()> {
    let width = 1200usize;
    let height = 800usize;
    let bits = 8;

    let header = format!("P6 {} {} {}\n", width, height, 2u32.pow(bits) - 1);
    
    let mut file = File::create("output.ppm")?; // "?" unpacks the result if Ok and returns the error if not
    file.write_all(header.as_bytes())?;

    let spheres = vec![
        Sphere { // large sphere to act as ground
            c: vec![0.0, -5001.0, 0.0],
            r: 5000.0,
            mat: Material {
                color: vec![1.0, 1.0, 1.0],
                spec: 250.0,
                refl: 0.8,
            },
        },
        Sphere {
            c: vec![0.0, -1.0, 3.0],
            r: 1.0,
            mat: Material {
                color: vec![1.0, 0.0, 0.0],
                spec: 500.0,
                refl: 0.5,
            },
        },
        Sphere {
            c: vec![2.0, 0.0, 4.0],
            r: 1.0,
            mat: Material {
                color: vec![0.0, 0.0, 1.0],
                spec: 500.0,
                refl: 0.5,
            },
        },
        Sphere {
            c: vec![-2.0, 0.0, 4.0],
            r: 1.0,
            mat: Material {
                color: vec![0.0, 1.0, 0.0],
                spec: 10.0,
                refl: 0.5,
            },
        },
        Sphere {
            c: vec![0.0, 1.0, 8.0],
            r: 2.0,
            mat: Material {
                color: vec![1.0, 1.0, 1.0],
                spec: -1.0,
                refl: 0.95,
            },
        },
    ];

    let lights = vec![
        Light {
            color: vec![0.2, 0.2, 0.2],
            kind: LightType::Ambient,
        },
        Light {
            color: vec![0.6, 0.6, 0.6],
            kind: LightType::Point(vec![2.0, 1.0, 0.0]),
        },
        Light {
            color: vec![0.2, 0.2, 0.2],
            kind: LightType::Directional(vec![1.0, 4.0, 4.0]),
        },
    ];

    // print to stderr so output isn't buffered until the end
    eprintln!("\nrender dimensions: {} x {}", width, height);
    eprintln!("rendering... ");
    
    let clock = time::Instant::now();
    let ppm = render(width, height, &spheres, &lights);
    let time = clock.elapsed();
    
    eprintln!("done ({}.{} sec)\n", time.as_secs(), time.as_millis());

    // NOTE: buffer copy here to transform buffer from Vec<Color> to Vec<u8> in a safe way
    let bytes = ppm.mat.len() * 3;
    let mut ppm_bytes: Vec<u8> = Vec::with_capacity(bytes);

    for pixel in ppm.mat {
        ppm_bytes.push(pixel.r);
        ppm_bytes.push(pixel.g);
        ppm_bytes.push(pixel.b);
    }

    file.write_all(&ppm_bytes)?;

    Ok(())
}
