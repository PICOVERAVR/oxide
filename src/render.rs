use crate::draw::*;
use crate::vec::*;
use crate::mat::*;
use crate::ray::*;

// iterate through all objects in objs and return the index of the closest object and the hit point
// (or None if nothing hits)
pub fn closest_hit(r: &Ray, objs: &[impl RayInteraction], lim: (f32, f32)) -> Option<(usize, Vec<f32>)> {
    let mut best_t = f32::INFINITY;
    let mut best = None;

    for (i, obj) in objs.iter().enumerate() {
        if let HitType::Hit(t) = obj.hit(r, lim) {
            let p = add(&r.o, &mul(&[t, t, t], &r.d));
            if t < best_t {
                best = Some((i, p));
                best_t = t;
            }
        }
    }

    best
}

// iterate through all objects in objs and return the index of the first object to hit and the hit point
// (or None if nothing hits)
pub fn any_hit(r: &Ray, objs: &[impl RayInteraction], lim: (f32, f32)) -> Option<(usize, Vec<f32>)> {
    for (i, obj) in objs.iter().enumerate() {
        if let HitType::Hit(t) = obj.hit(r, lim) {
            let p = add(&r.o, &mul(&[t, t, t], &r.d));
            return Some((i, p))
        }
    }

    None
}

pub fn render(width: usize, height: usize, objs: &[impl RayInteraction], lights: &[Light]) -> Matrix<Color> {
    let view_dist = 0.5; // distance from camera to viewport
    let view_width = 1.0; // width of viewport
    let view_height = 1.0 * height as f32 / width as f32; // height of viewport, transformed to make the viewport square regardless of the output dimensions

    let wi = width as i32;
    let hi = height as i32;

    let pixels = width * height + width + height; // adding an extra row and column to make canvas bounds symmetrical
    let mut buf = Matrix {
        mat: vec![Color {r: 255, g: 255, b: 255}; pixels],
        rlen: width as usize,
        clen: height as usize,
    };

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

            if let Some((i, p)) = closest_hit(&v_ray, objs, (view_dist, f32::INFINITY)) {
                let mut color_v = vec![0.0, 0.0, 0.0];
                for l in lights {
                    color_v = add(&color_v, &light(&objs[i], objs, &p, l, 3));
                }

                // clamp sum of light colors to correct output range and multiply by surface color
                let color_v = mul(&objs[i].material(&p).color, &clamp(&color_v, 0.0, 1.0));

                draw_pixel(&mut buf, x, y, map_color(&color_v));
            }
        }
    }

    buf
}