use std::ops::{Add, Sub, Mul, Div};
use num_traits::float::Float;

pub fn add<T>(lhs: &[T], rhs: &[T]) -> Vec<T> 
    where T: Add<Output = T> + Copy
    {
    assert_eq!(lhs.len(), rhs.len()); // make sure vectors are of the same size
    assert!(lhs.len() != 0);
    assert!(rhs.len() != 0);

    let mut ret: Vec<T> = Vec::with_capacity(lhs.len());

    for (i, &x) in lhs.iter().enumerate() {
        ret.push(x + rhs[i]);
    }

    ret
}

pub fn sub<T>(lhs: &[T], rhs: &[T]) -> Vec<T> 
    where T: Sub<Output = T> + Copy
    {
    assert_eq!(lhs.len(), rhs.len());
    assert!(lhs.len() != 0);
    assert!(rhs.len() != 0);

    let mut ret: Vec<T> = Vec::with_capacity(lhs.len());

    for (i, &x) in lhs.iter().enumerate() {
        ret.push(x - rhs[i]);
    }

    ret
}

pub fn mul<T>(lhs: &[T], rhs: &[T]) -> Vec<T> 
    where T: Mul<Output = T> + Copy
    {
    assert_eq!(lhs.len(), rhs.len());
    assert!(lhs.len() != 0);
    assert!(rhs.len() != 0);

    let mut ret: Vec<T> = Vec::with_capacity(lhs.len());

    for (i, &x) in lhs.iter().enumerate() {
        ret.push(x * rhs[i]);
    }

    ret
}

pub fn dot<T>(lhs: &[T], rhs: &[T]) -> T
    where T: Mul<Output = T> + Add<Output = T> + Copy
    {
    assert_eq!(lhs.len(), rhs.len());
    assert!(lhs.len() >= 2);
    assert!(rhs.len() >= 2);

    let mut ret = lhs[0] * rhs[0]; // init with first element

    for (i, &x) in lhs[1..].iter().enumerate() { // add rest of elements
        ret = ret + x * rhs[i + 1];
    }

    ret
}

// normalize a vector
pub fn norm<T>(v: &[T]) -> Vec<T>
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Copy + Float
    {

    let mag = dot(&v, &v).sqrt();

    let mut ret: Vec<T> = Vec::with_capacity(v.len());

    for x in v {
        ret.push(*x / mag);
    }

    ret
}

// negate a vector element-wise
pub fn neg<T>(v: &[T]) -> Vec<T>
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Copy + Float
    {

    let mut ret: Vec<T> = Vec::with_capacity(v.len());

    for x in v {
        ret.push(-*x);
    }

    ret
}
