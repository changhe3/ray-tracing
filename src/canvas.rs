use crate::color::Color;
use nalgebra::{Matrix3xX, Vector2};

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Canvas {
    inner: Matrix3xX<f32>,
    dim: Vector2<usize>,
}

impl Default for Canvas {
    fn default() -> Self {
        Canvas::new([0, 0])
    }
}

impl Canvas {
    pub fn new([width, height]: [usize; 2]) -> Self {
        Canvas {
            inner: Matrix3xX::repeat(width * height, 0.0),
            dim: [width, height].into(),
        }
    }

    pub fn into_inner(self) -> Matrix3xX<f32> {
        self.inner
    }

    fn index_at(&self, [x, y]: [usize; 2]) -> usize {
        let width = self.dim[0];
        x + y * width
    }

    pub fn set_pixel(&mut self, idx: [usize; 2], color: Color) {
        self.inner.set_column(self.index_at(idx), &color);
    }

    pub fn pixel_at(&self, idx: [usize; 2]) -> Color {
        self.inner.column(self.index_at(idx)).into_owned().into()
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::color::Color;
    use itertools::Itertools;

    #[test]
    fn test_new_canvas() {
        let canvas = Canvas::new([10, 20]);
        (0..10).cartesian_product(0..20).for_each(|(x, y)| {
            assert_eq!(canvas.pixel_at([x, y]), Color::new([0.0, 0.0, 0.0]));
        });
    }

    #[test]
    fn test_write_pixel() {
        let mut canvas = Canvas::new([10, 20]);
        let red = Color::RED.into();
        canvas.set_pixel([2, 3], red);
        assert_eq!(canvas.pixel_at([2, 3]), red);
    }
}
