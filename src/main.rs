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
use config::*;

use std::fs::File;
use std::io::Write;
use std::time;
use std::thread;

fn main() -> std::io::Result<()> {

    let header = format!("P6 {} {} {}\n", WIDTH, HEIGHT, 2u32.pow(BITS) - 1);
    
    let mut file = File::create("output.ppm")?; // "?" unpacks the result if Ok and returns the error if not
    file.write_all(header.as_bytes())?;

    let spheres = vec![
        Sphere { // large sphere to act as ground
            c: Vector::new_v3([0.0, -5001.0, 0.0]),
            r: 5000.0,
            mat: Material {
                color: Vector::new_v3([1.0, 1.0, 1.0]),
                spec: 250.0,
                refl: 0.8,
            },
        },
        Sphere {
            c: Vector::new_v3([0.0, -1.0, 3.0]),
            r: 1.0,
            mat: Material {
                color: Vector::new_v3([1.0, 0.0, 0.0]),
                spec: 500.0,
                refl: 0.5,
            },
        },
        Sphere {
            c: Vector::new_v3([2.0, 0.0, 4.0]),
            r: 1.0,
            mat: Material {
                color: Vector::new_v3([0.0, 0.0, 1.0]),
                spec: 500.0,
                refl: 0.5,
            },
        },
        Sphere {
            c: Vector::new_v3([-2.0, 0.0, 4.0]),
            r: 1.0,
            mat: Material {
                color: Vector::new_v3([0.0, 1.0, 0.0]),
                spec: 10.0,
                refl: 0.5,
            },
        },
        Sphere {
            c: Vector::new_v3([0.0, 1.0, 8.0]),
            r: 2.0,
            mat: Material {
                color: Vector::new_v3([1.0, 1.0, 1.0]),
                spec: -1.0,
                refl: 0.95,
            },
        },
    ];

    let lights = vec![
        Light {
            color: Vector::new_v3([0.2, 0.2, 0.2]),
            kind: LightType::Ambient,
        },
        Light {
            color: Vector::new_v3([0.6, 0.6, 0.6]),
            kind: LightType::Point(Vector::new_v3([2.0, 1.0, 0.0])),
        },
        Light {
            color: Vector::new_v3([0.2, 0.2, 0.2]),
            kind: LightType::Directional(Vector::new_v3([1.0, 4.0, 4.0])),
        },
    ];

    // print to stderr so output isn't buffered until the end
    eprintln!("\nrender dimensions: {} x {}", WIDTH, HEIGHT);
    eprintln!("rendering... ");

    // split the render into vertical slices and divide amongst threads
    // (horizontal slices are harder to collapse together)

    // TODO: come back to threading after learning about Arc<T>?
    // since the closure we pass to thread::spawn has static lifetime

    let mut m_parts = vec![];

    let dt = (HEIGHT / THREADS) as i32;
    let start = -(HEIGHT as i32) / 2 + dt / 2;
    
    let clock = time::Instant::now();

    m_parts.push(render(
        (0, start),
        (WIDTH, dt as usize),
        &spheres,
        &lights
    ));

    m_parts.push(render(
        (0, start + dt),
        (WIDTH, dt as usize),
        &spheres,
        &lights
    ));

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

    let mut bvec = vec![];

    for m in m_parts {
        bvec.append(&mut get_bytes(m));
    }

    assert_eq!(bvec.len(), WIDTH * HEIGHT * 3);

    file.write_all(&bvec)?;

    Ok(())
}
