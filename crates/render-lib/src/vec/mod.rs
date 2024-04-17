use crate::matrix::{Matrix3x3, MatrixMul};

pub type Vec3 = [f32; 3];

pub trait Vector {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;

    fn length(&self) -> f32;
    fn unit(&self) -> Self;
    fn add_vec(&self, rhs: &Self) -> Self;
    fn add(&self, rhs: f32) -> Self;
    fn sub_vec(&self, rhs: &Self) -> Self;
    fn sub(&self, rhs: f32) -> Self;
    fn mul(&self, rhs: f32) -> Self;
    fn div(&self, rhs: f32) -> Self;
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

    fn length(&self) -> f32 {
        f32::sqrt(self[0] * self[0] + self[1] * self[1] + self[2] * self[2])
    }

    fn unit(&self) -> Self {
        let length = self.length();
        self.map(|element| element / length)
    }

    fn add_vec(&self, rhs: &Self) -> Self {
        [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]]
    }

    fn add(&self, rhs: f32) -> Self {
        self.map(|element| element + rhs)
    }

    fn sub_vec(&self, rhs: &Self) -> Self {
        [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]]
    }

    fn sub(&self, rhs: f32) -> Self {
        self.map(|element| element - rhs)
    }

    fn mul(&self, rhs: f32) -> Self {
        self.map(|element| element * rhs)
    }

    fn div(&self, rhs: f32) -> Self {
        self.map(|element| element - rhs)
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

    #[test]
    fn length() {
        use crate::vec::Vector;

        assert_eq!(14.0f32.sqrt(), [1.0, 2.0, 3.0].length())
    }

    #[test]
    fn unit() {
        use crate::vec::Vector;
        assert!(1.0 - [1283.0, 63231.1, 84351.9].unit().length() < 0.00001)
    }
}
