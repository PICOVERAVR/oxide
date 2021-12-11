#[cfg(test)]
mod tests {
    use crate::mat::*;

    #[test]
    fn row_test() {
        let m1 = Matrix {
            mat: vec![
                1, 2, 3, 4,
                5, 6, 7, 8,
                9, 10, 11, 12,
                13, 14, 15, 16
            ],
            rlen: 4,
            clen: 4,
        };

        assert_eq!(row(&m1, 2), vec![9, 10, 11, 12]);
    }

    #[test]
    fn col_test() {
        let m1 = Matrix {
            mat: vec![
                1, 2, 3, 4,
                5, 6, 7, 8,
                9, 10, 11, 12,
                13, 14, 15, 16
            ],
            rlen: 4,
            clen: 4,
        };

        assert_eq!(col(&m1, 0), vec![1, 5, 9, 13]);
        assert_eq!(col(&m1, 1), vec![2, 6, 10, 14]);
        assert_eq!(col(&m1, 2), vec![3, 7, 11, 15]);
        assert_eq!(col(&m1, 3), vec![4, 8, 12, 16]);
    }

    #[test]
    fn matmul_test() {
        let m1 = Matrix {
            mat: vec! [
                1, 2,
                3, 4,
                5, 6,
                7, 8,
            ],
            rlen: 2,
            clen: 4,
        };

        let m2 = Matrix {
            mat: vec! [
                7, 2, 5,
                3, 9, 4,
            ],
            rlen: 3,
            clen: 2,
        };

        let m_exp = Matrix { 
            mat: vec! [
                13, 20, 13,
                33, 42, 31,
                53, 64, 49,
                73, 86, 67,
            ],
            rlen: 3,
            clen: 4,
        };

        let m3 = matmul(&m1, &m2);

        assert_eq!(m_exp.rlen, m3.rlen);
        assert_eq!(m_exp.clen, m3.clen);
        assert_eq!(m_exp.mat, m3.mat);
    }
}