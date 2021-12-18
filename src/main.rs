#![allow(unused_imports)]
#![allow(dead_code)]

//use vec::*; // "vec" is now valid in the main.rs scope, used when declaring types or concrete functions
mod vec; // triggers a load of the vec module
mod vec_test;
use vec::*;

mod mat;
mod mat_test;
use mat::*;

mod draw;
use draw::*;

mod ray;
use ray::*;

mod render;
use render::*;

mod config;

use std::fs::File;
use std::io::Write;
use std::time;
use std::thread;

fn main() -> std::io::Result<()> {

    let header = format!("P6 {} {} {}\n", config::WIDTH, config::HEIGHT, 2u32.pow(config::BITS) - 1);
    
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
    eprintln!("\nrender dimensions: {} x {}", config::WIDTH, config::HEIGHT);
    eprintln!("rendering... ");
    
    // TODO: need to read concurrency chapter and come back to threading this
    let clock = time::Instant::now();
    let m1 = render((0, -(config::HEIGHT as i32) / 4), (config::WIDTH, config::HEIGHT / 2), &spheres, &lights);
    let m2 = render((0, (config::HEIGHT as i32) / 4), (config::WIDTH, config::HEIGHT / 2), &spheres, &lights);
    let time = clock.elapsed();
    
    eprintln!("done ({}.{} sec)\n", time.as_secs(), time.as_millis());    

    fn get_bytes(m: Matrix<Color>) -> Vec<u8> {
        let size = (m.rlen - 1) * (m.clen - 1) * 3;
        let mut buf: Vec<u8> = Vec::with_capacity(size);

        for y in 1..m.clen {
            for x in 0..m.rlen - 1 {
                let idx = x + y * (m.rlen - 1);
    
                let c = m.mat[idx];
    
                buf.push(c.r);
                buf.push(c.g);
                buf.push(c.b);
            }
        }

        buf
    }

    // TODO: the append trick doesn't work when the width is split up
    let mut bvec = get_bytes(m1);
    bvec.append(&mut get_bytes(m2));

    assert_eq!(bvec.len(), config::WIDTH * config::HEIGHT * 3);

    file.write_all(&bvec)?;

    Ok(())
}
