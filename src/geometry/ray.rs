use crate::prelude::*;
use nalgebra::Unit;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub orig: Point,
    pub dir: Unit<Vector>,
}

impl Ray {
    pub fn new(origin: impl Into<Point>, direction: impl Into<Vector>) -> Self {
        Self {
            orig: origin.into(),
            dir: Unit::new_normalize(direction.into()),
        }
    }

    pub fn from_points(origin: impl Into<Point>, to: impl Into<Point>) -> Self {
        let orig = origin.into();
        Self {
            orig,
            dir: Unit::new_normalize(to.into() - orig),
        }
    }

    pub fn trace(self, t: f32) -> Point {
        self.orig + self.dir.into_inner() * t
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::ray::Ray;
    use crate::geometry::sphere::Sphere;
    use crate::geometry::Shape;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_intersect_sphere_tangent() {
        let r = Ray::new([0.0, 1.0, -5.0], [0.0, 0.0, 1.0]);
        let s = Sphere::default();
        let res = s.intersect(r);
        assert_eq!(res.len(), 1);
        assert_abs_diff_eq!(res[0], 5f32);
    }

    #[test]
    fn test_intersect_sphere_null() {
        let r = Ray::new([0.0, 2.0, -5.0], [0.0, 0.0, 1.0]);
        let s = Sphere::default();
        let res = s.intersect(r);
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_intersect_sphere_center() {
        let r = Ray::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let s = Sphere::default();
        let res = s.intersect(r);
        res.into_inner()
            .unwrap()
            .iter()
            .copied()
            .zip([-1.0, 1.0].iter().copied())
            .for_each(|(lhs, rhs)| {
                assert_abs_diff_eq!(lhs, rhs);
            });
    }
}
