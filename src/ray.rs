use crate::draw::Color;
use crate::mat::Matrix;
use crate::vec::*;

// draw a pixel on ppm
// x and y are coordinates going from -width/2 to width/2 and -height/2 to height/2 respectively
pub fn draw_pixel(ppm: &mut Matrix<Color>, x: i32, y: i32, color: Color) {
    let ax = x + ppm.rlen as i32 / 2;
    let ay = ppm.clen as i32 / 2 - y; // y direction needs to be flipped because the canvas y direction goes top to bottom

    ppm.mat[ay as usize * ppm.rlen + ax as usize] = color;
}

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
    pub spec: f32,
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
            _small if t1 < t.0 => f32::NEG_INFINITY,
            _fit if t.0 < t1 && t1 < t.1 => t1,
            _large if t1 > t.1 => f32::INFINITY,
            _ => f32::INFINITY, // handles case where t is NaN
        };

        let t2 = match t2 {
            _small if t2 < t.0 => f32::NEG_INFINITY,
            _fit if t.0 < t2 && t2 < t.1 => t2,
            _large if t2 > t.1 => f32::INFINITY,
            _ => f32::INFINITY,
        };

        // man, enum variants are cool
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

// runs lighting calculations at point p
pub fn light(obj: &impl RayInteraction, p: &[f32], light: &Light) -> Vec<f32> {

    let lc = &light.color;

    // calculate vector going _to_ the light source
    let l = match &light.kind {
        LightType::Point(lp) => sub(lp, p),
        LightType::Directional(ldir) => ldir.to_vec(),
        LightType::Ambient => return lc.to_vec(),
    };

    let i = norm(&l);
    let n = obj.normal(p);

    let m = obj.material(p);

    // multiply by both the material color and light color, as diffuse light is affected by both
    let diff = dot(&n, &i).max(0.0);
    let mut color = mul(&mul(&[diff, diff, diff], &m.color), &light.color);

    // find reflected ray r off object with equation 2 * N * dot(N, L) - L
    // N = normal, L = vector going _to_ light source
    let spec_scalar = 2.0 * dot(&n, &l);
    let r = sub(&mul(&[spec_scalar, spec_scalar, spec_scalar], &n), &l);
    
    let np = [-p[0], -p[1], -p[2]]; // negative p, or a vector to the camera
    let spec_dot = dot(&norm(&r), &norm(&np));
    if m.spec > 0.0 && spec_dot > 0.0 {
        // diff * color + spec
        let spec = spec_dot.powf(m.spec);
        let specc = mul(&[spec, spec, spec], lc); // specular color depends on light source
        color = add(&color, &specc);
    }
    
    clamp(&color, 0.0, 1.0) // clamp color to proper range
}
