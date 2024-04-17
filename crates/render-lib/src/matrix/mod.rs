pub type Matrix3x3 = [[f32; 3]; 3];

pub trait Unit {
    fn unit() -> Self;
}

impl Unit for Matrix3x3 {
    fn unit() -> Self {
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
    }
}

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

pub trait Transpose {
    type Output;
    fn transpose(&self) -> Self::Output;
}

impl Transpose for Matrix3x3 {
    type Output = Matrix3x3;

    fn transpose(&self) -> Self::Output {
        [
            [self[0][0], self[1][0], self[2][0]],
            [self[0][1], self[1][1], self[2][1]],
            [self[0][2], self[1][2], self[2][2]],
        ]
    }
}

mod tests {

    #[test]
    fn unit() {
        use crate::matrix::{Matrix3x3, Unit};

        assert_eq!(
            Matrix3x3::unit(),
            [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
        )
    }

    #[test]
    fn add() {
        use crate::matrix::{Matrix3x3, MatrixAdd};

        let a: Matrix3x3 = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        let b: Matrix3x3 = [[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]];

        assert_eq!(
            a.add(&b),
            [[2.0, 3.0, 4.0], [5.0, 6.0, 7.0], [8.0, 9.0, 10.0]]
        )
    }

    #[test]
    fn sub() {
        use crate::matrix::{Matrix3x3, MatrixSub};

        let a: Matrix3x3 = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        let b: Matrix3x3 = [[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]];

        assert_eq!(
            a.sub(&b),
            [[0.0, 1.0, 2.0], [3.0, 4.0, 5.0], [6.0, 7.0, 8.0]]
        )
    }

    #[test]
    fn mul() {
        use crate::matrix::{Matrix3x3, MatrixMul, Unit};

        let a: Matrix3x3 = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        let b: Matrix3x3 = Matrix3x3::unit();

        assert_eq!(a.mul(&b), a)
    }
    
    #[test]
    fn transpose() {
        use crate::matrix::{Matrix3x3, Transpose};
        
        let a: Matrix3x3 = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        assert_eq!(a.transpose(), [[1.0,4.0,7.0],[2.0,5.0,8.0],[3.0,6.0,9.0]])
    }
}
