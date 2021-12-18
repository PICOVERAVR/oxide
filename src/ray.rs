use crate::draw::Color;
use crate::vec::*;
use crate::render::{closest_hit, any_hit};

pub enum LightType {
    Point(Vec<f32>), // holds the position of the light
    Directional(Vec<f32>), // holds the direction going _to_ the light
    Ambient,
}

pub struct Light {
    pub color: Vec<f32>,
    pub kind: LightType,
}

pub struct Ray {
    pub o: Vec<f32>,
    pub d: Vec<f32>,
}

#[derive(PartialEq)]
pub enum HitType {
    Hit(f32), // returns closest time t of intersection
    Miss(),
}

#[derive(Clone)]
pub struct Material {
    pub color: Vec<f32>,
    pub spec: f32, // specular exponent, set to -1 for no specular highlights
    pub refl: f32, // reflectivity from 0 to 1
}

// behavior needed to interact with traced rays
pub trait RayInteraction {
    // check for hit from ray r over time range t
    fn hit(&self, r: &Ray, t: (f32, f32)) -> HitType;

    // calculate normal at point p on surface
    fn normal(&self, p: &[f32]) -> Vec<f32>;

    // calculate color at point p on surface
    fn material(&self, p: &[f32]) -> &Material;
}

// an infinite plane with a given normal
pub struct Plane {
    pub n: Vec<f32>,
    pub mat: Material,
}

impl RayInteraction for Plane {
    fn hit(&self, r: &Ray, _t: (f32, f32)) -> HitType {
        if dot(&norm(&neg(&self.n)), &norm(&r.d)) > 0.0 {
            return HitType::Hit(0.0) // TODO
        }

        HitType::Miss()
    }

    fn normal(&self, _p: &[f32]) -> Vec<f32> {
        self.n.to_vec()
    }

    fn material(&self, _p: &[f32]) -> &Material {
        &self.mat
    }
}

pub struct Sphere {
    pub c: Vec<f32>,
    pub r: f32,
    pub mat: Material,
}

impl RayInteraction for Sphere {
    // check if ray r hits sphere s at time t
    fn hit(&self, r: &Ray, t: (f32, f32)) -> HitType {
        let a = dot(&r.d, &r.d);
        
        let oc = sub(&r.o, &self.c);
        let b = 2.0 * dot(&oc, &r.d);

        let c = dot(&oc, &oc) - self.r * self.r;

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

    fn normal(&self, p: &[f32]) -> Vec<f32> {
        norm(&sub(p, &self.c))
    }

    fn material(&self, _p: &[f32]) -> &Material {
        &self.mat
    }
}

// runs lighting calculations at point p for object obj
// num_refl determines maximum recursion depth reflections
pub fn light(obj: &impl RayInteraction, objs: &[impl RayInteraction], p: &[f32], l: &Light, num_refl: u32) -> Vec<f32> {

    let lc = &l.color;

    // calculate vector going _to_ the light source
    let lv = match &l.kind {
        LightType::Point(lp) => sub(lp, p),
        LightType::Directional(ldir) => ldir.to_vec(),
        LightType::Ambient => return lc.to_vec(),
    };

    let max = match l.kind {
        LightType::Directional(_) => f32::INFINITY, // no max t for directional lights
        _ => 0.99, // don't test for shadows beyond the light origin for point lights
    };

    let m = obj.material(p);
    let mut color = m.color.to_vec();

    // if the ray going to the light hits another object, point p is in shadow
    // avoid edge case where object hits itself by using a small offset from 0 for t
    if let Some((_, _)) = any_hit(&Ray {o: p.to_vec(), d: lv.to_vec()}, objs, (0.01, max)) {
        return vec![0.0, 0.0, 0.0] // no light contribution if in shadow
    }

    let i = norm(&lv);
    let n = obj.normal(p);

    let diff = dot(&n, &i).max(0.0);
    color = mul(&mul(&[diff, diff, diff], &color), &l.color);

    let r = refl(&lv, &n); // calculate reflected vector off normal
    
    let np = neg(p); // negative p, or a vector to the camera
    let spec_dot = dot(&norm(&r), &norm(&np));
    if m.spec > 0.0 && spec_dot > 0.0 {
        // diff * color + spec
        let spec = spec_dot.powf(m.spec);
        let specc = mul(&[spec, spec, spec], lc); // specular color depends on light source
        color = add(&color, &specc);
    }

    clamp(&color, 0.0, 1.0); // clamp color to proper range

    if num_refl > 0 && m.refl > 0.0 {
        let r = refl(&lv, &obj.normal(p));
        let ref_ray = Ray {
            o: p.to_vec(),
            d: r.to_vec(),
        };

        // if object is reflective and we can recurse more, calculate lighting on reflection
        if let Some((i2, p2)) = closest_hit(&ref_ray, objs, (0.01, f32::INFINITY)) {
            let ref_l = Light {
                color: color.to_vec(),
                kind: LightType::Point(p.to_vec()),
            };

            let ref_color = light(&objs[i2], objs, &p2, &ref_l, num_refl - 1);

            // if we reflect off the object and hit something, compute the light for that:
            // color = color * (1 - refl) + ref_color * refl
            color = add(
                &mul(&ref_color, &[m.refl, m.refl, m.refl]),
                &mul(&color, &[1.0 - m.refl, 1.0 - m.refl, 1.0 - m.refl])
            );
        }
    }

    color
}
