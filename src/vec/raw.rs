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
        impl<T> core::ops::$Op for $Vec<T>
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
    (op, $Vec:ident($($field:tt),+), $Op:ident, $op:tt) => {
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

    (op_assgin, $Vec:ident($($field:tt),+), $Op:ident, $op:tt) => {
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

    (op_commutative, $Vec:ident, $Op:ident, $op:ident, $($lhs:ident),*) => {
        $(impl core::ops::$Op<$Vec<$lhs>> for $lhs
        {
            type Output = $Vec<$lhs>;

            #[inline]
            fn $op(self, rhs: $Vec<$lhs>) -> Self::Output {
                rhs.$op(self)
            }
        })*
    }
}

/// Implements common traits and methods for a given vector.
///
/// - 1st arg is the representation (`named struct` or `tuple struct`) and the dimension of the
/// given vector.
/// - 2nd arg is the identifier and parameters used to generate a `new` method; each parameter can
/// be a single alphabet letter.
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
        impl_binop!(op, $Vec($($field),+), Add, add);
        impl_binop!(op, $Vec($($field),+), Sub, sub);
        impl_binop!(op, $Vec($($field),+), Mul, mul);
        impl_binop!(op, $Vec($($field),+), Div, div);
        impl_binop!(op_assgin, $Vec($($field),+), AddAssign, add_assign);
        impl_binop!(op_assgin, $Vec($($field),+), SubAssign, sub_assign);
        impl_binop!(op_assgin, $Vec($($field),+), MulAssign, mul_assign);
        impl_binop!(op_assgin, $Vec($($field),+), DivAssign, div_assign);
        impl_binop!(op_commutative, $Vec, Mul, mul, f32, f64, i8, u8, i16, u16, i32, u32, i64, u64);

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
        impl<T> $Vec<T> {
            #[inline]
            pub fn count(&self) -> usize {
                $fieldcount
            }

            #[inline]
            pub fn map<D,F>(self, mut f: F) -> $Vec<D> where F: FnMut(T) -> D {
                $Vec::new($(f(self.$field)),+)
            }
        }

        impl<T: Scalar> $Vec<T> {
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
/// - 2nd arg is the identifier of the given vector.
///
/// Noting that your must use [`impl_vec_common`] to implement common traits and methods for your
/// type first in order to successfully call this macro.
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
    (@3, $($Vec:ident),+) => {
        $(impl<T: Scalar> $Vec<T> {
            #[inline]
            pub fn cross(self, rhs: Self) -> Self {
                let ((x, y, z), (u, v, w)) = (self.into(), rhs.into());
                (y * w - z * v, z * u - x * w, x * v - y * u).into()
            }
        })+
    };

    (@1, $($Vec:ident),+) => {};
    (@2, $($Vec:ident),+) => {};
    (@4, $($Vec:ident),+) => {};
    (@fieldcount:tt, $($Vec:ident),+) => {};
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
/// A 2D spatial vector type.
pub struct Vec2d<T> {
    pub x: T,
    pub y: T,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
/// A 3D spatial vector type.
pub struct Vec3d<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
/// A 4D spatial vector type.
pub struct Vec4d<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl_scalar!(f32, f64);

impl_vec_common!(named_struct@2, Vec2d(x, y), (x, y));
impl_vec_common!(named_struct@3, Vec3d(x, y, z), (x, y, z));
impl_vec_common!(named_struct@4, Vec4d(x, y, z, w), (x, y, z, w));

impl_vec_specific!(@2, Vec2d);
impl_vec_specific!(@3, Vec3d);
impl_vec_specific!(@4, Vec4d);

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Copy, Clone, Debug, Default, PartialEq)]
    struct Tv2<T>(T, T);

    #[derive(Copy, Clone, Debug, Default, PartialEq)]
    struct Tv3<T>(T, T, T);

    #[derive(Copy, Clone, Debug, Default, PartialEq)]
    struct Tv4<T>(T, T, T, T);

    #[derive(Copy, Clone, Debug, Default, PartialEq)]
    struct Sv2<T> {
        x: T,
        y: T,
    }

    #[derive(Copy, Clone, Debug, Default, PartialEq)]
    struct Sv3<T> {
        x: T,
        y: T,
        z: T,
    }

    #[derive(Copy, Clone, Debug, Default, PartialEq)]
    struct Sv4<T> {
        x: T,
        y: T,
        z: T,
        w: T,
    }

    // Tests if macros work as expected.
    impl_vec_common!(tuple_struct@2, Tv2(x, y), (0, 1));
    impl_vec_common!(tuple_struct@3, Tv3(x, y, z), (0, 1, 2));
    impl_vec_common!(tuple_struct@4, Tv4(x, y, z, w), (0, 1, 2, 3));
    impl_vec_common!(named_struct@2, Sv2(x, y), (x, y));
    impl_vec_common!(named_struct@3, Sv3(x, y, z), (x, y, z));
    impl_vec_common!(named_struct@4, Sv4(x, y, z, w), (x, y, z, w));
    impl_vec_specific!(@2, Tv2, Sv2);
    impl_vec_specific!(@3, Tv3, Sv3);
    impl_vec_specific!(@4, Tv4, Sv4);

    // Convenient macros
    macro_rules! tv {
        ($x:literal $(,)? $y:literal) => {
            Tv2::new($x as f64, $y as f64)
        };
        ($x:literal $(,)? $y:literal $(,)? $z:literal $(,)?) => {
            Tv3::new($x as f64, $y as f64, $z as f64)
        };
        ($x:literal $(,)? $y:literal $(,)? $z:literal $(,)? $w:literal $(,)?) => {
            Tv4::new($x as f64, $y as f64, $z as f64, $w as f64)
        };
    }

    macro_rules! sv {
        ($x:literal $(,)? $y:literal) => {
            Sv2::new($x as f64, $y as f64)
        };
        ($x:literal $(,)? $y:literal $(,)? $z:literal $(,)?) => {
            Sv3::new($x as f64, $y as f64, $z as f64)
        };
        ($x:literal $(,)? $y:literal $(,)? $z:literal $(,)? $w:literal $(,)?) => {
            Sv4::new($x as f64, $y as f64, $z as f64, $w as f64)
        };
    }

    #[test]
    fn vec_count() {
        assert_eq!(tv!(3, 4).count(), 2);
        assert_eq!(tv!(3, 4, 5).count(), 3);
        assert_eq!(tv!(3, 4, 5, 6).count(), 4);

        assert_eq!(sv!(3, 4).count(), 2);
        assert_eq!(sv!(3, 4, 5).count(), 3);
        assert_eq!(sv!(3, 4, 5, 6).count(), 4);
    }

    #[test]
    fn vec_dot() {
        assert_eq!(tv!(3, 4).dot(tv!(1, 2)), 11.);
        assert_eq!(tv!(3, 4, 5).dot(tv!(1, 2, 3)), 26.);
        assert_eq!(tv!(3, 4, 5, 6).dot(tv!(1, 2, 3, 4)), 50.);

        assert_eq!(sv!(3, 4).dot(sv!(1, 2)), 11.);
        assert_eq!(sv!(3, 4, 5).dot(sv!(1, 2, 3)), 26.);
        assert_eq!(sv!(3, 4, 5, 6).dot(sv!(1, 2, 3, 4)), 50.);
    }

    #[test]
    fn vec_sum() {
        assert_eq!(tv!(3, 4).sum(), 7.);
        assert_eq!(tv!(3, 4, 5).sum(), 12.);
        assert_eq!(tv!(3, 4, 5, 6).sum(), 18.);

        assert_eq!(sv!(3, 4).sum(), 7.);
        assert_eq!(sv!(3, 4, 5).sum(), 12.);
        assert_eq!(sv!(3, 4, 5, 6).sum(), 18.);
    }

    #[test]
    fn vec_len_squared() {
        assert_eq!(tv!(3, 4).len_squared(), 25.);
        assert_eq!(tv!(3, 4, 5).len_squared(), 50.);
        assert_eq!(tv!(3, 4, 5, 6).len_squared(), 86.);

        assert_eq!(sv!(3, 4).len_squared(), 25.);
        assert_eq!(sv!(3, 4, 5).len_squared(), 50.);
        assert_eq!(sv!(3, 4, 5, 6).len_squared(), 86.);
    }

    #[test]
    fn vec_len() {
        assert_eq!(tv!(3, 4).len(), 5.);
        assert_eq!(tv!(2, 4, 4).len(), 6.);
        assert_eq!(tv!(2, 4, 4, 0).len(), 6.);

        assert_eq!(sv!(3, 4).len(), 5.);
        assert_eq!(sv!(2, 4, 4).len(), 6.);
        assert_eq!(sv!(2, 4, 4, 0).len(), 6.);
    }

    #[test]
    fn vec_normal() {
        let tv1 = tv!(3, 4);
        let tv2 = tv!(3, 4, 5);
        let tv3 = tv!(3, 4, 5, 6);

        let sv1 = sv!(3, 4);
        let sv2 = sv!(3, 4, 5);
        let sv3 = sv!(3, 4, 5, 6);

        assert_eq!(sv1.normal(), sv1 / sv1.len());
        assert_eq!(sv2.normal(), sv2 / sv2.len());
        assert_eq!(sv3.normal(), sv3 / sv3.len());
        assert_eq!(tv1.normal(), tv1 / tv1.len());
        assert_eq!(tv2.normal(), tv2 / tv2.len());
        assert_eq!(tv3.normal(), tv3 / tv3.len());
    }

    #[test]
    fn vec_normalize() {
        let mut tv1 = tv!(3, 4);
        let mut tv2 = tv!(3, 4, 5);
        let mut tv3 = tv!(3, 4, 5, 6);

        let mut sv1 = sv!(3, 4);
        let mut sv2 = sv!(3, 4, 5);
        let mut sv3 = sv!(3, 4, 5, 6);

        tv1.normalize();
        tv2.normalize();
        tv3.normalize();
        sv1.normalize();
        sv2.normalize();
        sv3.normalize();

        assert!(tv1.sub(tv1 / tv1.len()).sum() < 0.0000000001);
        assert!(tv2.sub(tv2 / tv2.len()).sum() < 0.0000000001);
        assert!(tv3.sub(tv3 / tv3.len()).sum() < 0.0000000001);
        assert!(sv1.sub(sv1 / sv1.len()).sum() < 0.0000000001);
        assert!(sv2.sub(sv2 / sv2.len()).sum() < 0.0000000001);
        assert!(sv3.sub(sv3 / sv3.len()).sum() < 0.0000000001);
    }

    #[test]
    fn vec_map() {
        let tv1 = tv!(3, 4);
        let tv2 = tv!(3, 4, 5);
        let tv3 = tv!(3, 4, 5, 6);

        let sv1 = sv!(3, 4);
        let sv2 = sv!(3, 4, 5);
        let sv3 = sv!(3, 4, 5, 6);

        assert_eq!(tv1.map(|x| x / x), tv!(1., 1.));
        assert_eq!(tv2.map(|x| x / x), tv!(1., 1., 1.));
        assert_eq!(tv3.map(|x| x / x), tv!(1., 1., 1., 1.));

        assert_eq!(sv1.map(|x| x / x), sv!(1., 1.));
        assert_eq!(sv2.map(|x| x / x), sv!(1., 1., 1.));
        assert_eq!(sv3.map(|x| x / x), sv!(1., 1., 1., 1.));
    }

    #[test]
    fn vec_cross() {
        let tv1 = tv!(2., 4., 4.);
        let tv2 = tv!(1., 2., 3.);

        let sv1 = sv!(2., 4., 4.);
        let sv2 = sv!(1., 2., 3.);

        let cross1: Tv3<f64> = (4. * 3. - 4. * 2., 4. * 1. - 2. * 3., 2. * 2. - 4. * 1.).into();
        let cross2: Sv3<f64> = (4. * 3. - 4. * 2., 4. * 1. - 2. * 3., 2. * 2. - 4. * 1.).into();

        assert_eq!(tv1.cross(tv2), cross1);
        assert_eq!(sv1.cross(sv2), cross2);
    }
}
