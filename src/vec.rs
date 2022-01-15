use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Clone, Copy, Debug, Default)]
pub struct Vector {
    pub v: [f32; 4],
    len: usize,
}

impl Vector {
    pub fn zero(len: usize) -> Vector {
        assert!(len > 0);
        assert!(len < 5);

        Vector {
            v: [0.0; 4],
            len
        }
    }

    pub fn from_s(s: f32, len: usize) -> Vector {
        assert!(len > 0);
        assert!(len < 5);

        Vector {
            v: [s; 4],
            len
        }
    }

    pub fn from_v(arr: [f32; 4], len: usize) -> Vector {
        assert!(len > 0);
        assert!(len < 5);

        Vector {
            v: arr,
            len
        }
    }

    pub fn from_vec(vec: Vec<f32>) -> Vector {
        Vector {
            v: [vec[0], vec[1], vec[2], vec[3]],
            len: vec.len()
        }
    }

    pub fn from_3(x: f32, y: f32, z: f32) -> Vector {
        Vector {
            v: [x, y, z, 0.0],
            len: 3
        }
    }

    // length of the vector - must be in range [1, 4]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn dot(self, rhs: Vector) -> f32 {
        assert!(self.len == rhs.len);

        let mut sum = 0.0;
        for i in 0..self.len {
            sum += self.v[i] * rhs.v[i];
        }
        
        sum
    }

    pub fn norm(self) -> Vector {
        let mag = self.dot(self).sqrt();

        Vector {
            v: [self.v[0] / mag, self.v[1] / mag, self.v[2] / mag, self.v[3] / mag],
            len: self.len,
        }
    }

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

    /*
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
    */
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        assert!(self.len == rhs.len);

        Vector {
            v: [self.v[0] + rhs.v[0], self.v[1] + rhs.v[1], self.v[2] + rhs.v[2], self.v[3] + rhs.v[3]],
            len: self.len,
        }
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        assert!(self.len == rhs.len);

        Vector {
            v: [self.v[0] - rhs.v[0], self.v[1] - rhs.v[1], self.v[2] - rhs.v[2], self.v[3] - rhs.v[3]],
            len: self.len,
        }
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        assert!(self.len == rhs.len);

        Vector {
            v: [self.v[0] * rhs.v[0], self.v[1] * rhs.v[1], self.v[2] * rhs.v[2], self.v[3] * rhs.v[3]],
            len: self.len,
        }
    }
}

impl Div<Vector> for Vector {
    type Output = Vector;

    fn div(self, rhs: Vector) -> Vector {
        assert!(self.len == rhs.len);

        Vector {
            v: [self.v[0] / rhs.v[0], self.v[1] / rhs.v[1], self.v[2] / rhs.v[2], self.v[3] / rhs.v[3]],
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
                return false
            }
        }

        true
    }
}

// returns the reflection of i off a surface with the normal N
// NOTE: I and N should be pointing in the same direction
pub fn refl(i: Vector, n: Vector) -> Vector {
    assert!(i.len() == n.len());
    
    // compute R = 2 * N * dot(N, I) - I
    let s = n.dot(i) * 2.0;
    Vector::from_s(s, i.len()).mul(n).sub(i)
}
