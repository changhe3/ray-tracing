use image::Rgb;
use nalgebra::{clamp, Vector3};

use std::ops::{Deref, DerefMut};
use tap::Conv;

macro_rules! def_color {
    ($name:ident, ($r:literal, $g:literal, $b:literal)) => {
        pub const $name: Rgb<u8> = Rgb([$r, $g, $b]);
    };
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Color {
    rgb: Vector3<f32>,
}

impl Color {
    def_color!(BLACK, (0, 0, 0));
    def_color!(WHITE, (255, 255, 255));
    def_color!(RED, (255, 0, 0));
    def_color!(LIME, (0, 255, 0));
    def_color!(BLUE, (0, 0, 255));
    def_color!(YELLOW, (255, 255, 0));
    def_color!(CYAN, (0, 255, 255));
    def_color!(MAGENTA, (255, 0, 255));
    def_color!(SILVER, (192, 192, 192));
    def_color!(GRAY, (128, 128, 128));
    def_color!(MAROON, (128, 0, 0));
    def_color!(OLIVE, (128, 128, 0));
    def_color!(GREEN, (0, 128, 0));
    def_color!(PURPLE, (128, 0, 128));
    def_color!(TEAL, (0, 128, 128));
    def_color!(NAVY, (0, 0, 128));

    pub fn into_inner(self) -> Vector3<f32> {
        self.rgb
    }
}

impl Color {
    pub const fn new([r, g, b]: [f32; 3]) -> Self {
        Self {
            rgb: Vector3::new(r, g, b),
        }
    }

    pub fn into_rgb(self) -> Rgb<u8> {
        let rgb: Vector3<f32> = self.rgb * 256.0;
        let rgb = rgb.map(|v| clamp(v.floor(), 0.0, 255.0) as u8);
        Rgb(rgb.data.0[0])
    }

    pub fn from_rgb(Rgb(rgb): Rgb<u8>) -> Self {
        let rgb = Vector3::from(rgb).map(|v| v.conv::<f32>()) / 255.0;
        Self { rgb }
    }
}

impl From<Vector3<f32>> for Color {
    fn from(rgb: Vector3<f32>) -> Self {
        Self { rgb }
    }
}

impl From<Rgb<u8>> for Color {
    fn from(rgb: Rgb<u8>) -> Self {
        Self::from_rgb(rgb)
    }
}

impl Deref for Color {
    type Target = Vector3<f32>;

    fn deref(&self) -> &Self::Target {
        &self.rgb
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rgb
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgb;
    use nalgebra::vector;

    #[test]
    fn test_to_rgb() {
        assert_eq!(
            Color::from(vector!(0.0, 0.0, 0.0)).into_rgb(),
            Rgb([0, 0, 0])
        );
        assert_eq!(
            Color::from(vector!(1.0, 1.0, 1.0)).into_rgb(),
            Rgb([255, 255, 255])
        );
        assert_eq!(
            Color::from(vector!(0.5, 0.5, 0.5)).into_rgb(),
            Rgb([128, 128, 128])
        );
    }
}
