use std::ops::Mul;
use std::ops::Add;

use crate::vec::dot;

// matrix organization:
// [ 0 1 2 ] < row
// [ 3 4 5 ]
// [ 6 7 8 ]
//   ^ col

pub struct Matrix<T> {
    pub mat: Vec<T>,
    pub rlen: usize,
    pub clen: usize,
}

pub fn mat<T>(rlen: usize, clen: usize) -> Matrix<T> {
    Matrix {
        mat: Vec::with_capacity(rlen * clen),
        rlen,
        clen,
    }
}

// returns the nth column vector (starting from 0)
pub fn col<T>(m: &Matrix<T>, n: usize) -> Vec<T>
    where T: Copy
    {
    let mut ret: Vec<T> = Vec::with_capacity(m.clen);

    for i in (n..m.mat.len()).step_by(m.rlen) {
        ret.push(m.mat[i]);
    }

    ret
}

// return the nth row vector (starting from 0)
pub fn row<T>(m: &Matrix<T>, n: usize) -> Vec<T>
    where T: Copy
    {
    let mut ret: Vec<T> = Vec::with_capacity(m.rlen);

    let ridx = n * m.rlen;
    for i in ridx..ridx + m.rlen {
        ret.push(m.mat[i]);
    }

    ret
}

// multiply two matrices inefficiently
pub fn matmul<T>(lhs: &Matrix<T>, rhs: &Matrix<T>) -> Matrix<T> 
    where T: Copy + Mul<Output = T> + Add<Output = T>
    {
    let mut ret = Matrix {
        mat: Vec::with_capacity(lhs.clen * rhs.rlen),
        rlen: rhs.rlen,
        clen: lhs.clen,
    };

    for y in 0..ret.clen {
        for x in 0..rhs.rlen {
            let row = row(lhs, y);
            let col = col(rhs, x);
            ret.mat.push(dot(&row, &col));
        }
    }

    ret
}