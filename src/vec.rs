use std::ops::{Add, Sub, Mul, Div};
use num_traits::float::Float;

use num::cast::*;

pub fn add<T>(lhs: &[T], rhs: &[T]) -> Vec<T>
    where T: Add<Output = T> + Copy
    {
    assert!(!lhs.is_empty());

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
    assert!(!lhs.is_empty());
    assert!(!rhs.is_empty());

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
    assert!(!lhs.is_empty());
    assert!(!rhs.is_empty());

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
    where T: Float
    {

    let mag = dot(v, v).sqrt();

    let mut ret: Vec<T> = Vec::with_capacity(v.len());

    for x in v {
        ret.push(*x / mag);
    }

    ret
}

// negate a vector element-wise
pub fn neg<T>(v: &[T]) -> Vec<T>
    where T: Float
    {

    let mut ret: Vec<T> = Vec::with_capacity(v.len());

    for x in v {
        ret.push(-*x);
    }

    ret
}

// clamp all values in a vector to [min, max]
pub fn clamp<T>(v: &[T], min: T, max: T) -> Vec<T>
    where T: Float
    {

    let mut ret: Vec<T> = Vec::with_capacity(v.len());

    for x in v {
        let x = match *x {
            _low if *x < min => min,
            _in if min <= *x && *x <= max => *x,
            _high if *x > max => max,
            _ => panic!(),
        };

        ret.push(x);
    }

    ret
}

// return minimum of two vectors, element-wise
pub fn min<T>(lhs: &[T], rhs: &[T]) -> Vec<T>
    where T: Float
    {
    assert_eq!(lhs.len(), rhs.len());

    let mut ret: Vec<T> = Vec::with_capacity(lhs.len());

    for (i, &x) in lhs.iter().enumerate() {
        let cmp = match x {
            _lhs if x < rhs[i] => x,
            _rhs if x >= rhs[i] => rhs[i],
            _ => panic!(),
        };

        ret.push(cmp)
    }

    ret
}

// return maximum of two vectors, element wise
pub fn max<T>(lhs: &[T], rhs: &[T]) -> Vec<T>
    where T: Float
    {

    let mut ret: Vec<T> = Vec::with_capacity(lhs.len());

    for (i, &x) in lhs.iter().enumerate() {
        let cmp = match x {
            _lhs if x > rhs[i] => x,
            _rhs if x <= rhs[i] => rhs[i],
            _ => panic!(),
        };

        ret.push(cmp)
    }

    ret
}

// returns the reflection of i off a surface with the normal N
// NOTE: I and N should be pointing in the same direction
pub fn refl<T>(i: &[T], n: &[T]) -> Vec<T> 
    where T: Float
    {
    assert!(i.len() == n.len());
    
    // compute R = 2 * N * dot(N, I) - I
    let s = dot(n, i) * num::cast(2.0).unwrap();
    sub(&mul(&vec![s; i.len()], n), i)
}
