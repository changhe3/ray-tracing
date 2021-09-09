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
