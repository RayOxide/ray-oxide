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
