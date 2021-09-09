use crate::geometry::ray::Ray;
use crate::geometry::Shape;
use crate::prelude::*;
use arrayvec::ArrayVec;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Sphere {
    pub c: Point,
    pub r: f32,
}

impl Sphere {
    pub fn new(center: [f32; 3], radius: f32) -> Self {
        Self {
            c: center.into(),
            r: radius,
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            c: Point::origin(),
            r: 1.0,
        }
    }
}

impl Shape for Sphere {
    type Hits = ArrayVec<f32, 2>;

    //noinspection RsBorrowChecker
    fn intersect(&self, Ray { orig, dir }: Ray) -> Self::Hits {
        let c_to_orig = orig - self.c;
        let k_dot_c_to_org = dir.dot(&c_to_orig);
        let const_term = c_to_orig.magnitude_squared() - self.r.powi(2);
        let delta = k_dot_c_to_org.powi(2) - const_term;

        let mut solution = ArrayVec::new();
        if delta >= 0.0 {
            let delta_sqrt = delta.sqrt();
            solution.push(-k_dot_c_to_org - delta_sqrt);
            if delta > 0.0 {
                solution.push(-k_dot_c_to_org + delta_sqrt);
            }
        }
        solution
    }
}
