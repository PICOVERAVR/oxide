#[cfg(test)]
mod tests {
    /*
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
                1.0, 2.0,
                3.0, 4.0,
                5.0, 6.0,
                7.0, 8.0,
            ],
            rlen: 2,
            clen: 4,
        };

        let m2 = Matrix {
            mat: vec! [
                7.0, 2.0, 5.0,
                3.0, 9.0, 4.0,
            ],
            rlen: 3,
            clen: 2,
        };

        let m_exp = Matrix {
            mat: vec! [
                13.0, 20.0, 13.0,
                33.0, 42.0, 31.0,
                53.0, 64.0, 49.0,
                73.0, 86.0, 67.0,
            ],
            rlen: 3,
            clen: 4,
        };

        let m3 = matmul(&m1, &m2);

        assert_eq!(m_exp.rlen, m3.rlen);
        assert_eq!(m_exp.clen, m3.clen);
        assert_eq!(m_exp.mat, m3.mat);
    }

    #[test]
    fn cut_test() {
        let m1 = Matrix {
            mat: vec![
                1, 2, 3, 4, 5,
                6, 7, 8, 9, 10,
                11, 12, 13, 14, 15,
                16, 17, 18, 19, 20,
                21, 22, 23, 24, 25,
            ],
            rlen: 5,
            clen: 5,
        };

        let v1 = cut(&m1, (0, 0));

        assert_eq!(v1, m1.mat);

        let v2 = cut(&m1, (1, 1));

        assert_eq!(v2, vec![
            6, 7, 8, 9,
            11, 12, 13, 14,
            16, 17, 18, 19,
            21, 22, 23, 24,
        ]);

        assert_eq!(v2.len(), m1.rlen * m1.clen - m1.rlen - m1.clen + 1)
    }
    */
}
