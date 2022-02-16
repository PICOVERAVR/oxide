#[cfg(test)]
mod tests {
    use crate::vec::*;

    #[test]
    fn add_test() {
        let v1 = Vector::from_v([1.0, 2.0, 3.0, 4.0], 4);
        let v2 = Vector::from_v([6.0, 7.0, 8.0, 9.0], 4);

        assert_eq!(v1 + v2, Vector::from_v([7.0, 9.0, 11.0, 13.0], 4));
    }

    #[test]
    fn sub_test() {
        let v1 = Vector::from_v([1.0, 2.0, 3.0, 4.0], 4);
        let v2 = Vector::from_v([6.0, 7.0, 8.0, 9.0], 4);

        assert_eq!(v1 - v2, Vector::from_v([-5.0, -5.0, -5.0, -5.0], 4));
    }

    #[test]
    fn mul_test() {
        let v1 = Vector::from_v([1.0, 2.0, 3.0, 4.0], 4);
        let v2 = Vector::from_v([6.0, 7.0, 8.0, 9.0], 4);

        assert_eq!(v1 * v2, Vector::from_v([6.0, 14.0, 24.0, 36.0], 4));
    }

    #[test]
    fn dot_test() {
        let v1 = Vector::from_v([1.0, 2.0, 3.0, 4.0], 4);
        let v2 = Vector::from_v([5.0, 4.0, 3.0, 2.0], 4);

        assert_eq!(v1.dot(v2), 30.0);
    }

    #[test]
    fn norm_test() {
        let v = Vector::from_v([5.0, 0.0, 0.0, 0.0], 4);

        assert_eq!(v.norm(), Vector::from_v([1.0, 0.0, 0.0, 0.0], 4));
    }

    #[test]
    fn clamp_test() {
        let v = Vector::from_v([-5.0, -1.0, 1.0, 5.0], 4);

        assert_eq!(
            v.clamp(-1.0, 1.0),
            Vector::from_v([-1.0, -1.0, 1.0, 1.0], 4)
        );
    }
}
