//! Defines a faster Vector datatype for graphics use.

use std::ops::{Add, Div, Mul, Neg, Sub};

/// A vector with anywhere between 1 and 4 elements.
/// This struct is much faster than using the native Rust `Vec` type since `Vec` requires memory allocations for all vectors,
/// while Vector always lives on the stack.
#[derive(Clone, Copy, Debug, Default)]
pub struct Vector {
    v: [f32; 4],
    len: usize,
}

impl Vector {
    /// Returns the internal vector state. Exists so the internal `v` field can be replaced with SIMD intrinsics if possible.
    pub fn get(&self) -> [f32; 4] {
        self.v
    }

    pub fn x(&self) -> f32 {
        self.v[0]
    }

    pub fn y(&self) -> f32 {
        assert!(self.len >= 1);
        self.v[1]
    }

    pub fn z(&self) -> f32 {
        assert!(self.len >= 2);
        self.v[2]
    }

    pub fn w(&self) -> f32 {
        assert!(self.len >= 3);
        self.v[3]
    }

    /// Returns a zero vector of size `len`.
    pub fn zero(len: usize) -> Vector {
        assert!(len > 0);
        assert!(len < 5);

        Vector { v: [0.0; 4], len }
    }

    /// Returns a vector with all elements set to `s` and of size `len`.
    pub fn from_s(s: f32, len: usize) -> Vector {
        assert!(len > 0);
        assert!(len < 5);

        Vector { v: [s; 4], len }
    }

    /// Returns a vector created from array `arr` and of size `len`.
    pub fn from_v(arr: [f32; 4], len: usize) -> Vector {
        assert!(len > 0);
        assert!(len < 5);

        Vector { v: arr, len }
    }

    /// Returns a vector created from regular Rust vector `vec` with a size between 1 and 4.
    pub fn from_vec(vec: Vec<f32>) -> Vector {
        assert!(!vec.is_empty());
        assert!(vec.len() < 5);

        Vector {
            v: [vec[0], vec[1], vec[2], vec[3]],
            len: vec.len(),
        }
    }

    /// Returns a vector of length 3 and with values `x`, `y`, and `z`.
    pub fn from_3(x: f32, y: f32, z: f32) -> Vector {
        Vector {
            v: [x, y, z, 0.0],
            len: 3,
        }
    }

    /// Returns the length of the vector, which is always between 1 and 4 (inclusive).
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the vector is empty, which should never happen.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the dot product of `self` and `rhs`.  Both vectors must have the same length.
    pub fn dot(self, rhs: Vector) -> f32 {
        assert!(self.len == rhs.len);

        let mut sum = 0.0;
        for i in 0..self.len {
            sum += self.v[i] * rhs.v[i];
        }
        sum
    }

    /// Returns the normalized version of `self`.
    pub fn norm(self) -> Vector {
        let mag = self.dot(self).sqrt();

        Vector {
            v: [
                self.v[0] / mag,
                self.v[1] / mag,
                self.v[2] / mag,
                self.v[3] / mag,
            ],
            len: self.len,
        }
    }

    /// Returns a vector with all elements clamped to the range [min, max].
    pub fn clamp(self, min: f32, max: f32) -> Vector {
        let mut ret = Vector {
            v: [0.0, 0.0, 0.0, 0.0],
            len: self.len,
        };

        for i in 0..4 {
            let inp = self.v[i];

            let res = match inp {
                _low if inp < min => min,
                _in if min <= inp && inp <= max => inp,
                _high if inp > max => max,
                _ => panic!(),
            };

            ret.v[i] = res;
        }

        ret
    }

    /// Returns the smaller of two vectors, element-wise.
    pub fn min(self, rhs: Vector) -> Vector {
        let mut ret = Vector {
            v: [0.0, 0.0, 0.0, 0.0],
            len: self.len,
        };

        for i in 0..4 {
            let inp = self.v[i];

            let res = match inp {
                _lhs if inp < rhs.v[i] => inp,
                _high => rhs.v[i],
            };

            ret.v[i] = res;
        }

        ret
    }

    /// Returns the larger of two vectors, element-wise.
    pub fn max(self, rhs: Vector) -> Vector {
        let mut ret = Vector {
            v: [0.0, 0.0, 0.0, 0.0],
            len: self.len,
        };

        for i in 0..4 {
            let inp = self.v[i];

            let res = match inp {
                _lhs if inp > rhs.v[i] => inp,
                _high => rhs.v[i],
            };

            ret.v[i] = res;
        }

        ret
    }

    /// Returns the reflection of `i` off a surface with the normal `n`.
    /// I and N should be pointing in the same direction and have the same length.
    pub fn refl(i: Vector, n: Vector) -> Vector {
        assert!(i.len() == n.len());
        // compute R = 2 * N * dot(N, I) - I
        let s = n.dot(i) * 2.0;
        Vector::from_s(s, i.len()).mul(n).sub(i)
    }

    /// Returns the linear interpolation of `a` and `b` according to `f` according
    /// to the equation `a * f + b * (1 - f)`.
    /// `f` is clamped to the range [0, 1].
    pub fn lerp(a: Vector, b: Vector, f: f32) -> Vector {
        let f = f.clamp(0.0, 1.0);
        a * Vector::from_s(f, 3) + b * Vector::from_s(1.0 - f, 3)
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        assert!(self.len == rhs.len);

        Vector {
            v: [
                self.v[0] + rhs.v[0],
                self.v[1] + rhs.v[1],
                self.v[2] + rhs.v[2],
                self.v[3] + rhs.v[3],
            ],
            len: self.len,
        }
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        assert!(self.len == rhs.len);

        Vector {
            v: [
                self.v[0] - rhs.v[0],
                self.v[1] - rhs.v[1],
                self.v[2] - rhs.v[2],
                self.v[3] - rhs.v[3],
            ],
            len: self.len,
        }
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        assert!(self.len == rhs.len);

        Vector {
            v: [
                self.v[0] * rhs.v[0],
                self.v[1] * rhs.v[1],
                self.v[2] * rhs.v[2],
                self.v[3] * rhs.v[3],
            ],
            len: self.len,
        }
    }
}

impl Div<Vector> for Vector {
    type Output = Vector;

    fn div(self, rhs: Vector) -> Vector {
        assert!(self.len == rhs.len);

        Vector {
            v: [
                self.v[0] / rhs.v[0],
                self.v[1] / rhs.v[1],
                self.v[2] / rhs.v[2],
                self.v[3] / rhs.v[3],
            ],
            len: self.len,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            v: [-self.v[0], -self.v[1], -self.v[2], -self.v[3]],
            len: self.len,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        for i in 0..self.len() {
            if self.v[i] != other.v[i] {
                return false;
            }
        }

        true
    }
}
