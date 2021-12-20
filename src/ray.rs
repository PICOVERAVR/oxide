use crate::draw::Color;
use crate::vec::*;
use crate::render::{closest_hit, any_hit};

pub enum LightType {
    Point(Vector), // holds the position of the light
    Directional(Vector), // holds the direction going _to_ the light
    Ambient,
}

pub struct Light {
    pub color: Vector,
    pub kind: LightType,
}

pub struct Ray {
    pub o: Vector,
    pub d: Vector,
}

#[derive(PartialEq)]
pub enum HitType {
    Hit(f32), // returns closest time t of intersection
    Miss(),
}

#[derive(Copy, Clone)]
pub struct Material {
    pub color: Vector,
    pub spec: f32, // specular exponent, set to -1 for no specular highlights
    pub refl: f32, // reflectivity from 0 to 1
}

// behavior needed to interact with traced rays
pub trait RayInteraction {
    // check for hit from ray r over time range t
    fn hit(&self, r: &Ray, t: (f32, f32)) -> HitType;

    // calculate normal at point p on surface
    fn normal(&self, p: &Vector) -> Vector;

    // calculate color at point p on surface
    fn material(&self, p: &Vector) -> Material;
}

// an infinite plane with a given normal
pub struct Plane {
    pub p: Vector,
    pub n: Vector,
    pub mat: Material,
}

impl RayInteraction for Plane {
    fn hit(&self, r: &Ray, _t: (f32, f32)) -> HitType {
        let t = self.n.dot(self.p - r.o) / self.n.dot(r.d);
        HitType::Hit(t)
    }

    fn normal(&self, _p: &Vector) -> Vector {
        self.n
    }

    fn material(&self, _p: &Vector) -> Material {
        self.mat
    }
}

pub struct Sphere {
    pub c: Vector,
    pub r: f32,
    pub mat: Material,
}

impl RayInteraction for Sphere {
    // check if ray r hits sphere s at time t
    fn hit(&self, r: &Ray, t: (f32, f32)) -> HitType {
        let a = r.d.dot(r.d);
        
        let oc = r.o - self.c;
        let b = 2.0 * oc.dot(r.d);

        let c = oc.dot(oc) - self.r * self.r;

        let t1 = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
        let t2 = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);

        // constrain t1 and t2 to the range [t.0, t.1] or assign em to be +/- inf

        let t1 = match t1 {
            _small if t1 <= t.0 => f32::NEG_INFINITY,
            _fit if t.0 < t1 && t1 < t.1 => t1,
            _large if t1 >= t.1 => f32::INFINITY,
            _ => f32::INFINITY, // handles case where t is NaN
        };

        let t2 = match t2 {
            _small if t2 <= t.0 => f32::NEG_INFINITY,
            _fit if t.0 < t2 && t2 < t.1 => t2,
            _large if t2 >= t.1 => f32::INFINITY,
            _ => f32::INFINITY,
        };

        match (t1, t2) {
            _miss if t1.is_infinite() && t2.is_infinite() => HitType::Miss(),
            _edge if !t1.is_infinite() && t2.is_infinite() => HitType::Hit(t1),
            _edge if t1.is_infinite() && !t2.is_infinite() => HitType::Hit(t2),
            _int if !t1.is_infinite() && !t2.is_infinite() => match (t1, t2) {
                _t1 if t1 < t2 => HitType::Hit(t1),
                _t2 => HitType::Hit(t2), // t1 >= t2
            },
            (_, _) => panic!(),
        }
    }

    fn normal(&self, p: &Vector) -> Vector {
        (*p - self.c).norm()
    }

    fn material(&self, _p: &Vector) -> Material {
        self.mat
    }
}

// runs lighting calculations at point p for object obj
// num_refl determines maximum recursion depth reflections
pub fn light(obj: &impl RayInteraction, objs: &[impl RayInteraction], p: &Vector, l: &Light, num_refl: u32) -> Vector {

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

    let m = obj.material(p);
    let mut color = m.color;

    // if the ray going to the light hits another object, point p is in shadow
    // avoid edge case where object hits itself by using a small offset from 0 for t
    if let Some((_, _)) = any_hit(&Ray {o: *p, d: lv}, objs, (0.01, max)) {
        return Vector::zero(3) // no light contribution if in shadow
    }

    let i = lv.norm();
    let n = obj.normal(p);

    let diff = n.dot(i).max(0.0);
    let diff_v = Vector::new_s(diff, 3);
    color = diff_v * color * l.color;

    let r = refl(lv, n); // calculate reflected vector off normal
    
    let np = -*p; // negative p, or a vector to the camera
    let spec_dot = r.norm().dot(np.norm());
    if m.spec > 0.0 && spec_dot > 0.0 {
        // diff * color + spec
        let spec_v = Vector::new_s(spec_dot.powf(m.spec), 3);
        let specc = spec_v * lc; // specular color depends on light source
        color = color + specc;
    }

    color.clamp(0.0, 1.0); // clamp color to proper range

    if num_refl > 0 && m.refl > 0.0 {
        let r = refl(lv, obj.normal(p));
        let ref_ray = Ray {
            o: *p,
            d: r,
        };

        // if object is reflective and we can recurse more, calculate lighting on reflection
        if let Some((i2, p2)) = closest_hit(&ref_ray, objs, (0.01, f32::INFINITY)) {
            let ref_l = Light {
                color,
                kind: LightType::Point(*p),
            };

            let ref_color = light(&objs[i2], objs, &p2, &ref_l, num_refl - 1);

            // if we reflect off the object and hit something, compute the light for that:
            // color = color * (1 - refl) + ref_color * refl
            color = ref_color * Vector::new_s(m.refl, 3) + color * Vector::new_s(1.0 - m.refl, 3);
        }
    }

    color
}
