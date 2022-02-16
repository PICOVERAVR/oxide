//! Contains overall render logic.

use crate::draw::*;
use crate::mat::*;
use crate::opts::*;
use crate::ray::*;
use crate::vec::*;

/// Iterates through all objects in objs and return the index of the _closest_ object and the hit point.
/// Returns `None` if nothing hits.
pub fn closest_hit(
    r: &Ray,
    objs: &[Box<dyn RayInteraction>],
    lim: (f32, f32),
) -> Option<(usize, Vector)> {
    let mut best_t = f32::INFINITY;
    let mut best = None;

    for (i, obj) in objs.iter().enumerate() {
        if let HitType::Hit(t) = obj.hit(r, lim) {
            let p = r.o + Vector::from_s(t, 3) * r.d;
            if t < best_t {
                best = Some((i, p));
                best_t = t;
            }
        }
    }

    best
}

/// Ierates through all objects in objs and return the index of the _first_ object to hit and the hit point.
/// Returns `None` if nothing hits.
pub fn any_hit(
    r: &Ray,
    objs: &[Box<dyn RayInteraction>],
    lim: (f32, f32),
) -> Option<(usize, Vector)> {
    for (i, obj) in objs.iter().enumerate() {
        if let HitType::Hit(t) = obj.hit(r, lim) {
            let p = r.o + Vector::from_s(t, 3) * r.d;
            return Some((i, p));
        }
    }

    None
}

/// Runs lighting calculations at point `p` for the object at index `i`.
/// `num_refl` determines the maximum recursion depth reflections.
pub fn light(
    idx: usize,
    set: &[Box<dyn RayInteraction>],
    p: &Vector,
    l: &Light,
    num_refl: u32,
) -> Vector {
    let lc = l.color;

    // calculate vector going _to_ the light source
    let lv = match &l.kind {
        LightType::Point(lp) => *lp - *p,
        LightType::Directional(ldir) => *ldir,
        LightType::Ambient => return lc,
    };

    let max = match l.kind {
        LightType::Directional(_) => f32::INFINITY, // no max t for directional lights
        _ => 0.99, // don't test for shadows beyond the light origin for point lights
    };

    let obj = &set[idx];

    let m = obj.material(p);
    let mut color = m.color;

    // if the ray going to the light hits another object, point p is in shadow
    // avoid edge case where object hits itself by using a small offset from 0 for t
    if let Some((_, _)) = any_hit(&Ray { o: *p, d: lv }, set, (0.01, max)) {
        return Vector::zero(3); // no light contribution if in shadow
    }

    let i = lv.norm();
    let n = obj.normal(p);

    let diff = n.dot(i).max(0.0);
    let diff_v = Vector::from_s(diff, 3);
    color = diff_v * color * l.color;

    let r = Vector::refl(lv, n); // calculate reflected vector off normal
    let np = -*p; // negative p, or a vector to the camera
    let spec_dot = r.norm().dot(np.norm());
    if m.spec > 0.0 && spec_dot > 0.0 {
        // diff * color + spec
        let spec_v = Vector::from_s(spec_dot.powf(m.spec), 3);
        let specc = spec_v * lc; // specular color depends on light source
        color = color + specc;
    }

    color.clamp(0.0, 1.0); // clamp color to proper range

    if num_refl > 0 && m.refl > 0.0 {
        let r = Vector::refl(lv, obj.normal(p));
        let ref_ray = Ray { o: *p, d: r };

        // if object is reflective and we can recurse more, calculate lighting on reflection
        if let Some((i2, p2)) = closest_hit(&ref_ray, set, (0.01, f32::INFINITY)) {
            let ref_l = Light {
                color,
                kind: LightType::Point(*p),
            };

            let ref_color = light(i2, set, &p2, &ref_l, num_refl - 1);

            // if we reflect off the object and hit something, compute the light for that:
            // color = color * (1 - refl) + ref_color * refl
            color = ref_color * Vector::from_s(m.refl, 3) + color * Vector::from_s(1.0 - m.refl, 3);
        }
    }

    color
}

/// Renders a scene containing objects in `objs`, lights in `lights`, and configuration information in `cfg`.
/// Returns a Matrix of colors representing RGB values of the final image.
pub fn render(
    start: (i32, i32),
    dims: (usize, usize),
    set: &[Box<dyn RayInteraction>],
    lights: &[Light],
    cfg: &Config,
) -> Matrix<Color> {
    let view_dist = 0.5; // distance from camera to viewport
    let view_width = 1.0; // width of viewport
    let view_height = 1.0 * dims.1 as f32 / dims.0 as f32; // height of viewport, transformed to make the viewport square regardless of the output dimensions

    let di = (dims.0 as i32, dims.1 as i32);

    // adding an extra row and column to make canvas bounds symmetrical
    let pixels = dims.0 * dims.1 + dims.0 + dims.1 + 1;
    let mut buf = Matrix {
        mat: vec![
            Color {
                r: 255,
                g: 255,
                b: 255
            };
            pixels
        ],
        rlen: dims.0 as usize + 1, // write increased bounds to matrix dimensions as well, since we don't use it here
        clen: dims.1 as usize + 1,
    };

    for y in -di.1 / 2..di.1 / 2 {
        for x in -di.0 / 2..di.0 / 2 {
            let xf = x as f32;
            let yf = y as f32;

            // transform canvas coordinates to viewport coordinates
            // note that the viewport axis and scale is the same of the canvas, so the transform is just a scaling op
            let view_coord = Vector::from_3(
                (xf + start.0 as f32) * view_width / dims.0 as f32,
                (yf - start.1 as f32) * view_height / dims.1 as f32,
                view_dist,
            );

            let cv = cfg.world.cam_pos;
            // create ray coming off viewport
            let v_ray = Ray {
                o: cv,
                d: view_coord, // can adjust rotation by multiplying by rotation matrix here
            };

            if let Some((i, p)) = closest_hit(&v_ray, set, (view_dist, f32::INFINITY)) {
                let mut color_v = Vector::zero(3);

                for l in lights {
                    color_v = color_v + light(i, set, &p, l, cfg.render.max_reflections);
                }

                // clamp sum of light colors to correct output range and multiply by surface color
                let color_v = (set[i].material(&p).color * color_v).clamp(0.0, 1.0);
                draw_pixel(&mut buf, (x, y), dims, map_color(color_v));
            }
        }
    }

    buf
}
