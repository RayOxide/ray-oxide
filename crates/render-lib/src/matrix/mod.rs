pub type Matrix3x3 = [[f32; 3]; 3];

pub trait MatrixAdd {
    fn add(&self, rhs: &Self) -> Self;
}

impl MatrixAdd for Matrix3x3 {
    fn add(&self, rhs: &Self) -> Self {
        [
            [
                self[0][0] + rhs[0][0],
                self[0][1] + rhs[0][1],
                self[0][2] + rhs[0][2],
            ],
            [
                self[1][0] + rhs[1][0],
                self[1][1] + rhs[1][1],
                self[1][2] + rhs[1][2],
            ],
            [
                self[2][0] + rhs[2][0],
                self[2][1] + rhs[2][1],
                self[2][2] + rhs[2][2],
            ],
        ]
    }
}

pub trait MatrixSub {
    fn sub(&self, rhs: &Self) -> Self;
}

impl MatrixSub for Matrix3x3 {
    fn sub(&self, rhs: &Self) -> Self {
        [
            [
                self[0][0] - rhs[0][0],
                self[0][1] - rhs[0][1],
                self[0][2] - rhs[0][2],
            ],
            [
                self[1][0] - rhs[1][0],
                self[1][1] - rhs[1][1],
                self[1][2] - rhs[1][2],
            ],
            [
                self[2][0] - rhs[2][0],
                self[2][1] - rhs[2][1],
                self[2][2] - rhs[2][2],
            ],
        ]
    }
}

pub trait MatrixMul<Rhs> {
    type Output;

    fn mul(&self, rhs: &Rhs) -> Self::Output;
}

impl MatrixMul<Matrix3x3> for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(&self, rhs: &Matrix3x3) -> Self::Output {
        [
            [
                self[0][0] * rhs[0][0] + self[0][1] * rhs[1][0] + self[0][2] * rhs[2][0],
                self[0][0] * rhs[0][1] + self[0][1] * rhs[1][1] + self[0][2] * rhs[2][1],
                self[0][0] * rhs[0][2] + self[0][1] * rhs[1][2] + self[0][2] * rhs[2][2],
            ],
            [
                self[1][0] * rhs[0][0] + self[1][1] * rhs[1][0] + self[1][2] * rhs[2][0],
                self[1][0] * rhs[0][1] + self[1][1] * rhs[1][1] + self[1][2] * rhs[2][1],
                self[1][0] * rhs[0][2] + self[1][1] * rhs[1][2] + self[1][2] * rhs[2][2],
            ],
            [
                self[2][0] * rhs[0][0] + self[2][1] * rhs[1][0] + self[2][2] * rhs[2][0],
                self[2][0] * rhs[0][1] + self[2][1] * rhs[1][1] + self[2][2] * rhs[2][1],
                self[2][0] * rhs[0][2] + self[2][1] * rhs[1][2] + self[2][2] * rhs[2][2],
            ],
        ]
    }
}

mod tests {

    #[test]
    fn mul() {
        use crate::matrix::{Matrix3x3, MatrixMul};

        let a: Matrix3x3 = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        let b: Matrix3x3 = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

        assert_eq!(a.mul(&b), a)
    }
}
