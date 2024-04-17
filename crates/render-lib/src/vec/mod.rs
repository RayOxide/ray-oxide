use crate::matrix::{Matrix3x3, MatrixMul};

pub type Vec3 = [f32; 3];

pub trait Vector {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
}

impl Vector for Vec3 {
    fn x(&self) -> f32 {
        self[0]
    }

    fn y(&self) -> f32 {
        self[1]
    }

    fn z(&self) -> f32 {
        self[2]
    }
}

impl MatrixMul<Vec3> for Matrix3x3 {
    type Output = Vec3;

    fn mul(&self, rhs: &Vec3) -> Self::Output {
        [
            self[0][0] * rhs[0] + self[0][1] * rhs[1] + self[0][2] * rhs[2],
            self[1][0] * rhs[0] + self[1][1] * rhs[1] + self[1][2] * rhs[2],
            self[2][0] * rhs[0] + self[2][1] * rhs[1] + self[2][2] * rhs[2],
        ]
    }
}

mod tests {
    #[test]
    fn matrix_vec_mul() {
        use crate::matrix::{Matrix3x3, MatrixMul, Unit};
        use crate::vec::Vec3;

        let a = Matrix3x3::unit();
        let b: Vec3 = [1.0, 2.0, 3.0];

        assert_eq!(b, a.mul(&b))
    }
}
