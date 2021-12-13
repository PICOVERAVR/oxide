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
    Point(Vec<f32>),
    Directional(Vec<f32>),
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

// behavior needed to interact with traced rays
pub trait RayInteraction {
    // check for hit from ray r over time range t
    fn hit(&self, r: &Ray, t: (f32, f32)) -> HitType;

    // calculate normal at point p on surface
    fn normal(&self, p: &[f32]) -> Vec<f32>;

    // calculate color at point p on surface
    fn color(&self, p: &[f32]) -> Vec<f32>;
}

// an infinite plane with a given normal
pub struct Plane {
    pub n: Vec<f32>,
    pub color: Vec<f32>,
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

    fn color(&self, _p: &[f32]) -> Vec<f32> {
        self.color.to_vec()
    }
}

pub struct Sphere {
    pub c: Vec<f32>,
    pub r: f32,
    pub color: Vec<f32>,
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

    fn color(&self, _p: &[f32]) -> Vec<f32> {
        self.color.to_vec()
    }
}

// runs lighting calculations at point p
pub fn light(obj: &impl RayInteraction, p: &[f32], l: &Light) -> Vec<f32> {
    let lv = match &l.kind {
        LightType::Point(lp) => sub(lp, p),
        LightType::Directional(ldir) => ldir.to_vec(),
        LightType::Ambient => vec![1.0, 1.0, 1.0],
    };

    let i = norm(&lv);
    
    let diff = dot(&obj.normal(p), &i).max(0.0); // clamp to 0
    
    mul(&[diff, diff, diff], &l.color) // diff * color
}
