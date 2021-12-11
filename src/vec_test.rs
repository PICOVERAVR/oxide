#[cfg(test)]
mod tests {
    use crate::vec::*;

    #[test]
    fn add_test() {
        let v1 = vec![1, 2, 3, 4, 5];
        let v2 = vec![6, 7, 8, 9, 10];

        assert_eq!(add(&v1, &v2), vec![7, 9, 11, 13, 15]);
    }

    #[test]
    fn sub_test() {
        let v1 = vec![1, 2, 3, 4, 5];
        let v2 = vec![6, 7, 8, 9, 10];

        assert_eq!(sub(&v1, &v2), vec![-5, -5, -5, -5, -5]);
    }

    #[test]
    fn mul_test() {
        let v1 = vec![1, 2, 3, 4, 5];
        let v2 = vec![6, 7, 8, 9, 10];

        assert_eq!(mul(&v1, &v2), vec![6, 14, 24, 36, 50]);
    }

    #[test]
    fn dot_test() {
        let v1 = vec![1, 2, 3, 4, 5];
        let v2 = vec![5, 4, 3, 2, 1];

        assert_eq!(dot(&v1, &v2), 35);
    }

    #[test]
    fn num_test() {
        let v = vec![5.0, 0.0, 0.0, 0.0];
        let vn = vec![1.0, 0.0, 0.0, 0.0];

        assert_eq!(norm(&v), vn);
    }
}