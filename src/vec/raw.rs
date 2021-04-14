//! Generic raw vector types.
use core::ops::*;

/// Types that can be served as scalar values in vector space.
pub trait Scalar:
    Copy
    + AddAssign<Self>
    + SubAssign<Self>
    + DivAssign<Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
    fn sqrt(self) -> Self;
    fn zero() -> Self;
}

macro_rules! impl_scalar {
    ($($scalar: ty),+) => {
        $(
            impl Scalar for $scalar {
                #[inline]
                fn sqrt(self) -> Self {
                    self.sqrt()
                }

                #[inline]
                fn zero() -> Self {
                    Default::default()
                }
            }
        )+
    }
}

// From https://danielkeep.github.io/tlborm/book/blk-counting.html
#[doc(hidden)]
#[macro_export]
macro_rules! replace_tt {
    ($_t:tt $sub:tt) => {
        $sub
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_unop {
    ($Vec:ident($($field:tt),+), $Op:ident, $op:tt) => {
        impl<T> $Op for $Vec<T>
            where T: $Op<Output=T>
        {
            type Output = Self;

            #[inline]
            fn $op(self) -> Self::Output {
                Self::new($( self.$field.$op() ),+)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_binop {
    (op_vector, $Vec:ident($($field:tt),+), $Op:ident, $op:tt) => {
        impl<T, V> $Op<V> for $Vec<T>
            where T: $Op<T, Output=T>, V: Into<$Vec<T>>
        {
            type Output = Self;

            #[inline]
            fn $op(self, rhs: V) -> Self::Output {
                let rhs: Self = rhs.into();
                Self::new($( self.$field.$op(rhs.$field) ),+)
            }
        }
    };

    (op_assgin_vector, $Vec:ident($($field:tt),+), $Op:ident, $op:tt) => {
        impl<T, V> $Op<V> for $Vec<T>
            where T: $Op<T>, V: Into<$Vec<T>>
        {
            #[inline]
            fn $op(&mut self, rhs: V) {
                let rhs: Self = rhs.into();
                $( self.$field.$op(rhs.$field); )+
            }
        }
    };
}

/// Implements common traits and methods for a given vector.
///
/// - 1st arg is the representation (`named struct` or `tuple struct`) and the dimension of the 
/// given vector.
/// - 2nd arg is the identity and parameters used to generate a `new` method; each paramters can be
/// a single alphabet letter.
/// - 3rd arg is either the fields or the the indexes of the given vector, depending on its
/// internal representation.
///
/// In order to implement dimension-specifc methods for your vector, see [`impl_vec_specific`].
///
/// # Example
///
/// ```ignore
/// // A tuple struct vector.
/// #[derive(Copy, Clone, Debug, Default, PartialEq)]
/// pub struct Vec3<T>(T, T, T);
///
/// impl_vec_common!(tuple_struct@3, Vec3(x, y, z), (0, 1, 2));
/// impl_vec_specific!(@3, Vec3);
///
/// // A named struct vector.
/// #[derive(Copy, Clone, Debug, Default, PartialEq)]
/// pub struct Vec4<T> {
///     x: T,
///     y: T,
///     z: T,
///     w: T,
/// }
///
/// impl_vec_common!(named_struct@4, Vec4(x, y, z, w), (x, y, z, w));
/// impl_vec_specific!(@4, Vec4);
/// ```
#[macro_export]
macro_rules! impl_vec_common {
    (named_struct@$fieldcount:literal, $Vec:ident($($param:tt),+), $($field:tt),+) => {
        // impl new
        impl<T> $Vec<T> {
            pub const fn new($($param:T),+) -> Self {
                Self { $($param),+ }
            }
        }

        impl_vec_common!(common $fieldcount, $Vec($($param),+), ($($param),+));
    };

    (tuple_struct@$fieldcount:literal, $Vec:ident($($param:tt),+), ($($field:tt),+)) => {
        // impl new
        impl<T> $Vec<T> {
            pub const fn new($($param:T),+) -> Self {
                $Vec($($param),+)
            }
        }

        impl_vec_common!(common $fieldcount, $Vec($($param),+), ($($field),+));
    };

    (common $fieldcount:literal, $Vec:ident($($param:tt),+), ($($field:tt),+)) => {
        // Binary operators
        impl_binop!(op_vector, $Vec($($field),+), Add, add);
        impl_binop!(op_vector, $Vec($($field),+), Sub, sub);
        impl_binop!(op_vector, $Vec($($field),+), Mul, mul);
        impl_binop!(op_vector, $Vec($($field),+), Div, div);
        impl_binop!(op_assgin_vector, $Vec($($field),+), AddAssign, add_assign);
        impl_binop!(op_assgin_vector, $Vec($($field),+), SubAssign, sub_assign);
        impl_binop!(op_assgin_vector, $Vec($($field),+), MulAssign, mul_assign);
        impl_binop!(op_assgin_vector, $Vec($($field),+), DivAssign, div_assign);

        // Unary operators
        impl_unop!($Vec($($field),+), Neg, neg);

        // Conversions
        impl<T> From<T> for $Vec<T> where T: Copy {
            #[inline]
            fn from(t: T) -> $Vec<T> {
                $Vec::new($(replace_tt!($field t)),+)
            }
        }

        impl<T> Into<($(replace_tt!($field T)),+)> for $Vec<T> {
            #[inline]
            fn into(self) -> ($(replace_tt!($field T)),+) {
                ($(self.$field),+)
            }
        }

        impl<T> From<($(replace_tt!($param T)),+)> for $Vec<T> {
            #[inline]
            fn from(tuple: ($(replace_tt!($param T)),+)) -> Self {
                let ($($param),+) = tuple;
                $Vec::new($($param),+)
            }
        }

        impl<T> Into<[T; $fieldcount]> for $Vec<T> {
            #[inline]
            fn into(self) -> [T; $fieldcount] {
                [$(self.$field),+]
            }
        }

        impl<T> From<[T; $fieldcount]> for $Vec<T> {
            #[inline]
            fn from(array: [T; $fieldcount]) -> Self {
                let [$($param),+] = array;
                $Vec::new($($param),+)
            }
        }

        // common methods
        impl<T: Scalar> $Vec<T> {
            #[inline]
            pub fn component_count(&self) -> usize {
                $fieldcount
            }

            #[inline]
            pub fn sum(self) -> T {
                let ($($param),+) = self.into();
                T::zero() $(+ $param)*
            }

            #[inline]
            pub fn dot(self, rhs: Self) -> T {
                return (self * rhs).sum();
            }

            #[inline]
            pub fn len_squared(self) -> T {
                self.dot(self)
            }

            #[inline]
            pub fn len(self) -> T {
                self.len_squared().sqrt()
            }

            #[inline]
            pub fn normal(self) -> Self {
                self / self.len()
            }

            #[inline]
            pub fn normalize(&mut self) {
                *self /= self.len();
            }
        }

    };
}

/// Implements dimension-specific methods for a given vector.
///
/// - 1st arg is the dimension of the given vector.
/// - 2nd arg is the identity of the given vector.
///
/// Noting that you must implement `impl_vec_common` for your type first in order to successfully 
/// call this macro.
///
/// # Example
///
/// ```ignore
/// #[derive(Copy, Clone, Debug, Default, PartialEq)]
/// pub struct Vec3<T>(T, T, T);
///
/// impl_vec_common!(tuple_struct@3, Vec3(x, y, z), (0, 1, 2));
/// impl_vec_specific!(@3, Vec3);
/// ```
#[macro_export]
macro_rules! impl_vec_specific {
    (@3, $($Vec:ident)+) => {
        $(impl<T: Scalar> $Vec<T> {
            #[inline]
            pub fn cross(self, rhs: Self) -> Self {
                let ((x, y, z), (u, v, w)) = (self.into(), rhs.into());
                (y * w - z * v, z * u - x * w, x * v - y * u).into()
            }
        })+
    };

    (@1, $($Vec:ident)+) => {};
    (@2, $($Vec:ident)+) => {};
    (@4, $($Vec:ident)+) => {};
    (@fieldcount:tt, $($Vec:ident)+) => {};
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
/// A 2D spatial vector type.
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
/// A 3D spatial vector type.
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
/// A 4D spatial vector type.
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl_scalar!(f32, f64);

impl_vec_common!(named_struct@2, Vec2(x, y), (x, y));
impl_vec_common!(named_struct@3, Vec3(x, y, z), (x, y, z));
impl_vec_common!(named_struct@4, Vec4(x, y, z, w), (x, y, z, w));

impl_vec_specific!(@2, Vec2);
impl_vec_specific!(@3, Vec3);
impl_vec_specific!(@4, Vec4);

#[cfg(test)]
mod tests {
}
