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

        // Divide the color by the number of samples and grama-correct for `gamma=2.0`.
        let scale = 1.0 / samples_per_pixel as f64;
        r = (r * scale).sqrt();
        g = (g * scale).sqrt();
        b = (b * scale).sqrt();

        write!(
            stream,
            "{} {} {}\n",
            (r.clamp(0.0, 0.999) * 256.0) as u8,
            (g.clamp(0.0, 0.999) * 256.0) as u8,
            (b.clamp(0.0, 0.999) * 256.0) as u8,
        )
        .map_err(|_| ErrorKind::WriteColor(format!("{:?}", stream)))
    }
}
