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

#[cfg(test)]
mod tests {
    use crate::geometry::ray::Ray;
    use crate::geometry::sphere::Sphere;
    use crate::geometry::{HitContext, Hittable, Shape};
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_intersect_sphere_tangent() {
        let r = Ray::new([0.0, 1.0, -5.0], [0.0, 0.0, 1.0]);
        let s = Sphere::default().into_object();
        let inter = s.intersect(r);
        let (t, &HitContext { obj_hit, .. }) = inter.hit().unwrap();
        assert_eq!(inter.size(), 1);
        assert_abs_diff_eq!(t, 5.0);
        assert_eq!(obj_hit, &s as &dyn Hittable);
    }

    #[test]
    fn test_intersect_sphere_null() {
        let r = Ray::new([0.0, 2.0, -5.0], [0.0, 0.0, 1.0]);
        let s = Sphere::default().into_object();
        let inter = s.intersect(r);
        assert_eq!(inter.size(), 0);
        assert_eq!(inter.hit(), None);
    }

    #[test]
    fn test_intersect_sphere_center() {
        let r = Ray::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let s = Sphere::default().into_object();
        let inter = s.intersect(r);
        let (t, &HitContext { obj_hit, .. }) = inter.hit().unwrap();
        assert_abs_diff_eq!(t, 1.0);
        assert_eq!(obj_hit, &s as &dyn Hittable);
        let (t, &HitContext { obj_hit, .. }) = inter.between(-f32::INFINITY..0.0).next().unwrap();
        assert_abs_diff_eq!(t, -1.0);
        assert_eq!(obj_hit, &s as &dyn Hittable);
    }
}
