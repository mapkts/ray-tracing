//! Convenient re-exports of common items.
//!
//! The contents of this module must be imported manually:
//!
//! ```
//! use ray_tracing::prelude::*;
//! ```

pub use crate::camera::Camera;
pub use crate::color::Rgb;
pub use crate::consts::*;
pub use crate::error::{ErrorKind, Result};
pub use crate::ray::Ray;
pub use crate::vec::raw::Scalar;
pub use crate::vec::{Point3, Vec3};
