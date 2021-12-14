use crate::draw::*;
use crate::vec::*;
use crate::mat::*;
use crate::ray::*;

pub fn render(width: usize, height: usize, objs: &[impl RayInteraction], lights: &[Light]) -> Matrix<Color> {
    let view_dist = 0.5; // distance from camera to viewport
    let view_width = 1.0; // width of viewport
    let view_height = 1.0 * height as f32 / width as f32; // height of viewport, transformed to make the viewport square regardless of the output dimensions

    let wi = width as i32;
    let hi = height as i32;

    let pixels = width * height + width + height; // adding an extra row and column to make canvas bounds symmetrical
    let mut buf = Matrix {
        mat: Vec::with_capacity(pixels),
        rlen: width as usize,
        clen: height as usize,
    };

    for _y in 0..height+1 {
        for _x in 0..width+1 {
            buf.mat.push(Color {r: 255, g: 255, b: 255}); // initialize contents of canvas
        }
    }

    for y in -hi/2..hi/2 {
        for x in -wi/2..wi/2 {
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

            for obj in objs {
                if let HitType::Hit(t) = obj.hit(&v_ray, (view_dist, 100.0)) {

                    let p = add(&v_ray.o, &mul(&[t, t, t], &v_ray.d)); // compute intersection point
                    
                    let mut color_v = vec![0.0, 0.0, 0.0];
                    for l in lights {
                        color_v = add(&color_v, &light(obj, &p, l));
                    }

                    // clamp sum of light colors to correct output range and multiply by surface color
                    let color_v = mul(&obj.material(&p).color, &clamp(&color_v, 0.0, 1.0));

                    draw_pixel(&mut buf, x, y, map_color(&color_v));
                }
            }

        }
    }

    buf
}