//! RGB color type.
use crate::prelude::*;
use crate::vec::{Color, Vec3};
use std::fmt;
use std::io;

/// RGB color data.
pub type Rgb = Color<f64>;

impl From<Vec3> for Rgb {
    fn from(v: Vec3) -> Self {
        let (x, y, z) = v.into();
        (x, y, z).into()
    }
}

impl Into<Vec3> for Rgb {
    fn into(self) -> Vec3 {
        let (x, y, z) = self.into();
        (x, y, z).into()
    }
}

impl Rgb {
    pub fn write<W: io::Write + fmt::Debug>(
        self,
        stream: &mut W,
        samples_per_pixel: i32,
    ) -> Result<()> {
        let (mut r, mut g, mut b) = self.into();

        let scale = 1.0 / samples_per_pixel as f64;
        r = (r * scale).sqrt();
        g = (g * scale).sqrt();
        b = (b * scale).sqrt();

        write!(
            stream,
            "{} {} {} ",
            ((r * 256.0) as u8).clamp(0, u8::MAX),
            ((g * 256.0) as u8).clamp(0, u8::MAX),
            ((b * 256.0) as u8).clamp(0, u8::MAX),
        )
        .map_err(|_| ErrorKind::WriteColor(format!("{:?}", stream)))
    }
}
