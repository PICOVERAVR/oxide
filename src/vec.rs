use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

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
