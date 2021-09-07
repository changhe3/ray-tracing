use nalgebra::Vector3;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Ray {
    orig: Vector3<f32>,
    dir: Vector3<f32>,
}

impl Ray {
    pub fn new(orig: Vector3<f32>, dir: Vector3<f32>) -> Self {
        Ray { orig, dir }
    }

    pub fn trace(&self, t: f32) -> Vector3<f32> {
        self.dir * t + self.orig
    }

    pub fn intersect_sphere
}
