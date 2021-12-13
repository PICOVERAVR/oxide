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

use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let width = 1200i32;
    let height = 800i32;
    let bits = 8;

    let header = format!("P6 {} {} {}\n", width, height, 2u32.pow(bits) - 1);
    
    let mut file = File::create("output.ppm")?; // "?" unpacks the result if Ok and returns the error if not
    file.write_all(header.as_bytes())?;

    let wu = width as usize;
    let hu = height as usize;
    let pixels = wu * hu + wu + hu; // adding an extra row and column to make canvas bounds symmetrical
    let mut ppm = Matrix {
        mat: Vec::with_capacity(pixels),
        rlen: width as usize,
        clen: height as usize,
    };

    for _y in 0..height+1 {
        for _x in 0..width+1 {
            ppm.mat.push(Color {r: 255, g: 255, b: 255}); // initialize contents of canvas
        }
    }

    // https://gabrielgambetta.com/computer-graphics-from-scratch

    let view_dist = 1.0; // distance from camera to viewport
    let view_width = 1.0; // width of viewport
    let view_height = 1.0 * height as f32 / width as f32; // height of viewport, transformed to make the viewport square regardless of the output dimensions

    let spheres = vec![
        Sphere {
            c: vec![0.0, 0.0, 4.0],
            r: 0.5,
            color: vec![1.0, 0.0, 0.0],
        },
        Sphere {
            c: vec![1.0, 1.0, 5.0],
            r: 0.5,
            color: vec![0.0, 1.0, 0.0],
        },
        Sphere {
            c: vec![-1.0, 1.0, 5.0],
            r: 0.5,
            color: vec![0.0, 0.0, 1.0],
        },
    ];

    let plane = Plane {
        n: vec![0.0, 1.0, 0.0],
        color: vec![0.5, 0.5, 0.5],
    };

    let lights = vec![
        Light {
            color: vec![1.0, 1.0, 1.0],
            pos: vec![0.0, 0.0, 10.0],
            kind: LightType::Point,
        },
    ];

    for y in -height/2..height/2 {
        for x in -width/2..width/2 {
            let xf = x as f32;
            let yf = y as f32;

            // transform canvas coordinates to viewport coordinates
            // note that the viewport axis and scale is the same of the canvas, so the transform is just a scaling op
            let view_coord = vec![xf * view_width / width as f32, yf * view_height / height as f32, view_dist];

            // determine color seen by viewport square

            let d = norm(&view_coord);

            let v_ray = Ray {
                o: view_coord,
                d,
            };

            // very basic rendering for plane first
            match plane.hit(&v_ray, (view_dist, 100.0)) {
                HitType::Hit(_t) => draw_pixel(&mut ppm, x, y, map_color(&plane.color)),
                HitType::Miss() => (),
            }

            for s in &spheres {
                if let HitType::Hit(t) = s.hit(&v_ray, (view_dist, 100.0)) {

                    let p = add(&v_ray.o, &mul(&[t, t, t], &v_ray.d)); // compute intersection point

                    let mut color_v = vec![0f32, 0f32, 0f32];
                    for l in &lights {
                        color_v = add(&color_v, &s.light(&v_ray, l, &p));
                    }

                    // clamp sum of light colors to correct output range and multiply by surface color
                    let color_v = mul(&s.color, &clamp(&color_v, 0.0, 1.0));

                    draw_pixel(&mut ppm, x, y, map_color(&color_v));
                }
            }

        }
    }

    // NOTE: buffer copy here to transform buffer from Vec<Color> to Vec<u8> in a safe way
    let bytes = pixels * 3;
    let mut ppm_bytes: Vec<u8> = Vec::with_capacity(bytes);

    for pixel in ppm.mat {
        ppm_bytes.push(pixel.r);
        ppm_bytes.push(pixel.g);
        ppm_bytes.push(pixel.b);
    }

    file.write_all(&ppm_bytes)?;

    Ok(())
}
