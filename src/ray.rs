//! Controls how rays interact with shapes.

use crate::vec::*;

/// Defines how a light can behave.
pub enum LightType {
    /// A point light that emits light from a single point.
    Point(Vector),
    /// A directional light that emits light from a single direction.
    /// The `Vector` held holds the direction going _to_ the light source.
    Directional(Vector),
    /// A ambient light illuminates everything equally.
    Ambient,
}

/// A light source in the scene.
pub struct Light {
    pub color: Vector,
    pub kind: LightType,
}

/// A mathematical ray structure.
pub struct Ray {
    /// The origin.
    pub o: Vector,
    /// The direction.
    pub d: Vector,
}

/// Defines how a ray can intersect with things.
#[derive(PartialEq)]
pub enum HitType {
    /// Indicates an intersection at time parameter `t`.
    Hit(f32),
    /// Indicates no intersection.
    Miss(),
}

/// Controls how colors appear in the scene.
#[derive(Copy, Clone)]
pub struct Material {
    pub color: Vector,
    pub spec: f32, // specular exponent, set to -1 for no specular highlights
    pub refl: f32, // reflectivity from 0 to 1
}

/// Defines behavior needed to interact with traced rays.
pub trait RayInteraction {
    /// Checks for a hit from ray `r` over time range `t`.
    fn hit(&self, r: &Ray, t: (f32, f32)) -> HitType;

    /// Calculates a normal at point `p` on the object surface.
    fn normal(&self, p: &Vector) -> Vector;

    /// Calculates the color at point `p` on surface.
    /// Constant unless the material is defined procedurally.
    fn material(&self, p: &Vector) -> Material;
}

/// Defines an infinite plane with a given normal.
pub struct Plane {
    /// A point on the plane.
    pub p: Vector,
    /// A normal perpendicular to the plane.
    pub n: Vector,
    /// The material of the plane.
    pub mat: Material,
}

impl RayInteraction for Plane {
    fn hit(&self, r: &Ray, _t: (f32, f32)) -> HitType {
        if self.n.dot(-r.d) >= 0.0 {
            let t = self.n.dot(r.o - self.p) / self.n.dot(-r.d);

            if _t.0 < t && t < _t.1 {
                return HitType::Hit(t);
            } else {
                return HitType::Miss();
            }
        }
        HitType::Miss()
    }

    fn normal(&self, _p: &Vector) -> Vector {
        self.n
    }

    fn material(&self, _p: &Vector) -> Material {
        self.mat
    }
}

/// Defines a sphere.
pub struct Sphere {
    /// The center of the sphere.
    pub c: Vector,
    /// The radius of the sphere.
    pub r: f32,
    /// The material of the sphere.
    pub mat: Material,
}

impl RayInteraction for Sphere {
    /// Checks if ray `r` hits sphere `s` at time `t`.
    fn hit(&self, r: &Ray, t: (f32, f32)) -> HitType {
        let a = r.d.dot(r.d);
        let oc = r.o - self.c;
        let b = 2.0 * oc.dot(r.d);

        let c = oc.dot(oc) - self.r * self.r;

        let t1 = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
        let t2 = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);

        // constrain t1 and t2 to the range [t.0, t.1] or assign em to be inf

        let mut inf = (false, false);

        if t.0 < t1 && t1 < t.1 {
            // statement here to avoid keeping NaNs
        } else {
            inf.0 = true;
        }

        if t.0 < t2 && t2 < t.1 {
            // statement here to avoid keeping NaNs
        } else {
            inf.1 = true;
        }

        match (t1, t2) {
            _miss if inf.0 && inf.1 => HitType::Miss(),
            _edge if !inf.0 && inf.1 => HitType::Hit(t1),
            _edge if inf.0 && !inf.1 => HitType::Hit(t2),
            _int if !inf.0 && !inf.1 => match (t1, t2) {
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
