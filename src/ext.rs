/*!
This module provides extensions to [glam](https://docs.rs/glam).

Including:
* 8-bit integer vectors
* A generalized vector `select()` function.
*/

use std::{
	fmt::{Debug, Display},
	iter::{Product, Sum},
	ops::{
		Add, AddAssign,
		Sub, SubAssign,
		Mul, MulAssign,
		Div, DivAssign,
		Rem, RemAssign,
		BitAnd, BitOr, BitXor,
		Index, IndexMut,
		Neg, Not,
		Shl, Shr,
	},
};
use glam::{
	BVec2, BVec3, BVec4,
	I16Vec2, I16Vec3, I16Vec4,
	U16Vec2, U16Vec3, U16Vec4,
	IVec2, IVec3, IVec4,
	UVec2, UVec3, UVec4,
	I64Vec2, I64Vec3, I64Vec4,
	U64Vec2, U64Vec3, U64Vec4,
	Vec2, Vec3, Vec4,
	DVec2, DVec3, DVec4,
};
use num_traits::AsPrimitive;
use crate::*;

macro_rules! extended {
	((), $scalar:ty, $($symbol:ident),*) => {};
	(($extended:ty), $scalar:ty, $($symbol:ident),*) => {
		pub fn extend(self, a: $scalar) -> $extended { <$extended>::new($(self.$symbol,)* a) }
	};
}
macro_rules! truncated {
	((), $scalar:ty, $($symbol:ident),*) => {};
	($truncated:ty, $scalar:ty, $($keep:ident),*; $first:ident) => {
		pub fn truncate(self) -> $truncated { <$truncated>::new($(self.$keep),*) }
	};
	($truncated:ty, $scalar:ty, $($keep:ident),*; $first:ident $(, $symbol:ident)*) => {
		truncated!($truncated, $scalar, $($keep,)* $first; $($symbol),*);
	};
	(($truncated:ty), $scalar:ty, $first:ident $(, $symbol:ident)*) => {
		truncated!($truncated, $scalar, $first; $($symbol),*);
	};
}
macro_rules! sep {
	($first:expr $(, $last:expr)*; $sep:tt) => { $first $($sep $last)* };
}
macro_rules! nest {
	($symbol:ident, $one:expr) => { $one };
	($fn:ident, $first:expr $(, $last:expr)+) => { $first.$fn(nest!($fn $(, $last)+)) };
}
macro_rules! cmp_fn {
	($name:ident, $cmp:ident, $bvec:ty, $($symbol:ident),*) => {
		pub fn $name(self, rhs: Self) -> $bvec {
			<$bvec>::new($(self.$symbol.$cmp(&rhs.$symbol)),*)
		}
	};
}
macro_rules! as_fn {
	($name:ident, $type:ty, $scalar:ty, $($symbol:ident),*) => {
		pub fn $name(&self) -> $type { <$type>::new($(self.$symbol as $scalar),*) }
	};
}
macro_rules! bin_op {
	($name:ident, $($symbol:ident),*) => {
		pub const fn $name(self, rhs: Self) -> Self {
			Self { $($symbol: self.$symbol.$name(rhs.$symbol),)* }
		}
	};
}
macro_rules! impl_gen_bin_op {
	($trait:ident, $fn:ident, $gen:ty, $type:ident, $($symbol:ident),*) => {
		impl $trait<$gen> for $type {
			type Output = Self;
			fn $fn(self, rhs: $gen) -> Self {
				Self { $($symbol: self.$symbol.$fn(rhs.$symbol),)* }
			}
		}
	};
}
macro_rules! impl_bin_op_scalar {
	($trait:ident, $fn:ident, $scalar:ty, $type:ident, $($symbol:ident),*) => {
		impl $trait<$scalar> for $type {
			type Output = Self;
			fn $fn(self, rhs: $scalar) -> Self { Self { $($symbol: self.$symbol.$fn(rhs),)* } }
		}
	};
}
macro_rules! impl_assign {
	($trait:ident, $fn:ident, $type:ident, $($symbol:ident),*) => {
		impl $trait<$type> for $type {
			fn $fn(&mut self, rhs: Self) { $(self.$symbol.$fn(rhs.$symbol);)* }
		}
	};
}
macro_rules! impl_assign_scalar {
	($trait:ident, $fn:ident, $scalar:ty, $type:ident, $($symbol:ident),*) => {
		impl $trait<$scalar> for $type {
			fn $fn(&mut self, rhs: $scalar) { $(self.$symbol.$fn(rhs);)* }
		}
	};
}
macro_rules! impl_scalar_bin_op {
	($trait:ident, $fn:ident, $scalar:ty, $type:ident, $($symbol:ident),*) => {
		impl $trait<$type> for $scalar {
			type Output = $type;
			fn $fn(self, rhs: $type) -> $type { $type { $($symbol: self.$fn(rhs.$symbol),)* } }
		}
	};
}
macro_rules! bin_op_impls {
	(
		$bin:ident, $bin_fn:ident, $assign:ident, $assign_fn:ident, $type:ident, $scalar:ty,
		$($symbol:ident),*
	) => {
		impl_gen_bin_op!($bin, $bin_fn, $type, $type, $($symbol),*);
		impl_assign!($assign, $assign_fn, $type, $($symbol),*);
		impl_bin_op_scalar!($bin, $bin_fn, $scalar, $type, $($symbol),*);
		impl_assign_scalar!($assign, $assign_fn, $scalar, $type, $($symbol),*);
		impl_scalar_bin_op!($bin, $bin_fn, $scalar, $type, $($symbol),*);
	};
}
macro_rules! impl_acc {
	($trait:ident, $fn:ident, $init:ident, $acc_fn:ident, $type:ident) => {
		impl $trait for $type {
			fn $fn<I: Iterator<Item = Self>>(iter: I) -> Self {
				iter.fold(Self::$init, Self::$acc_fn)
			}
		}
		impl<'a> $trait<&'a Self> for $type {
			fn $fn<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
				iter.fold(Self::$init, |a, &b| Self::$acc_fn(a, b))
			}
		}
	};
}
macro_rules! impl_unary {
	(false, $trait:ident, $fn:ident, $type:ident, $($symbol:ident),*) => {};
	(true, $trait:ident, $fn:ident, $type:ident, $($symbol:ident),*) => {
		impl $trait for $type {
			type Output = Self;
			fn $fn(self) -> Self { Self { $($symbol: self.$symbol.$fn(),)* } }
		}
	};
}
macro_rules! impl_bin_op {
	($trait:ident, $fn:ident, $type:ident, $($symbol:ident),*) => {
		impl $trait for $type {
			type Output = Self;
			fn $fn(self, rhs: Self) -> Self {
				Self { $($symbol: self.$symbol.$fn(rhs.$symbol),)* }
			}
		}
	};
}
macro_rules! impl_bitwise {
	($trait:ident, $fn:ident, $scalar:ty, $type:ident, $($symbol:ident),*) => {
		impl_bin_op!($trait, $fn, $type, $($symbol),*);
		impl_bin_op_scalar!($trait, $fn, $scalar, $type, $($symbol),*);
	};
}
macro_rules! impl_shift {
	($shift:ty, $type:ident, $($symbol:ident),*) => {
		impl_bin_op_scalar!(Shl, shl, $shift, $type, $($symbol),*);
		impl_bin_op_scalar!(Shr, shr, $shift, $type, $($symbol),*);
	};
}
macro_rules! impl_display {
	($type:ident, $first:ident $(, $symbol:ident)*) => {
		impl Display for $type {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "[{}", self.$first)?;
				$(write!(f, ", {}", self.$symbol)?;)*
				write!(f, "]")
			}
		}
	};
}
macro_rules! omit {
	($omit:ident, $($keep:tt)*) => { $($keep)* };
}
macro_rules! impl_from_truncated {
	((), $type:ident, $scalar:ty, $($symbol:ident),*) => {};
	($truncated:ty, $type:ident, $scalar:ty, $($keep:ident),*; $first:ident) => {
		impl From<($truncated, $scalar)> for $type {
			fn from((v, a): ($truncated, $scalar)) -> Self { Self::new($(v.$keep,)* a) }
		}
	};
	($truncated:ty, $type:ident, $scalar:ty, $($keep:ident),*; $first:ident $(, $symbol:ident)*) => {
		impl_from_truncated!($truncated, $type, $scalar, $($keep,)* $first; $($symbol),*);
	};
	(($truncated:ty), $type:ident, $scalar:ty, $first:ident $(, $symbol:ident)*) => {
		impl_from_truncated!($truncated, $type, $scalar, $first; $($symbol),*);
	};
}
macro_rules! impl_try_from {
	($from:ty, $type:ident, $scalar:ty, $($symbol:ident),*) => {
		impl TryFrom<$from> for $type {
			type Error = core::num::TryFromIntError;
			fn try_from(v: $from) -> Result<Self, Self::Error> {
				Ok(Self::new($(<$scalar>::try_from(v.$symbol)?,)*))
			}
		}
	};
}
macro_rules! signed2 {
	(2, $scalar:ty) => {
		pub fn perp(self) -> Self { Self { x: -self.y, y: self.x } }
		pub fn perp_dot(self, rhs: Self) -> $scalar { (self.x * rhs.y) - (self.y * rhs.x) }
		pub fn rotate(self, rhs: Self) -> Self {
			Self { x: self.x * rhs.x - self.y * rhs.y, y: self.y * rhs.x + self.x * rhs.y }
		}
	};
	($dim:tt, $scalar:ty) => {};
}
macro_rules! signed {
	(false, $dim:tt, $scalar:ty, $($symbol:ident, $neg_cap:ident, $ord:expr, ($($axis:expr),*)),*) => {};
	(true, $dim:tt, $scalar:ty, $($symbol:ident, $neg_cap:ident, $ord:expr, ($($axis:expr),*)),*) => {
		pub const NEG_ONE: Self = Self::splat(-1);
		$(pub const $neg_cap: Self = Self::new($(-$axis),*);)*
		pub fn abs(self) -> Self { Self { $($symbol: self.$symbol.abs(),)* } }
		pub fn signum(self) -> Self { Self { $($symbol: self.$symbol.signum(),)* } }
		pub fn is_negative_bitmask(self) -> u32 {
			$(((self.$symbol.is_negative() as u32) << $ord))|*
		}
		pub fn distance_squared(self, rhs: Self) -> $scalar { (self - rhs).length_squared() }
		pub fn div_euclid(self, rhs: Self) -> Self {
			Self::new($(self.$symbol.div_euclid(rhs.$symbol),)*)
		}
		pub fn rem_euclid(self, rhs: Self) -> Self {
			Self::new($(self.$symbol.rem_euclid(rhs.$symbol),)*)
		}
		signed2!($dim, $scalar);
	};
}
macro_rules! cross {
	(3) => {
		pub fn cross(self, rhs: Self) -> Self {
			Self {
				x: self.y * rhs.z - rhs.y * self.z,
				y: self.z * rhs.x - rhs.z * self.x,
				z: self.x * rhs.y - rhs.x * self.y,
			}
		}
	};
	($dim:tt) => {};
}
macro_rules! decl_vec {
	(
		$fn:ident, $type:ident, $bvec:ty, $scalar:ty, $dim:tt, $signed:ident,
		($($extended:ty)?), ($($truncated:ty)?),
		$b8_fn:ident, $b8_type:ty, $b8_scalar:ty,
		$i16_fn:ident, $i16_type:ty, $u16_fn:ident, $u16_type:ty,
		$i32_fn:ident, $i32_type:ty, $u32_fn:ident, $u32_type:ty,
		$i64_fn:ident, $i64_type:ty, $u64_fn:ident, $u64_type:ty,
		$f32_fn:ident, $f32_type:ty, $f64_fn:ident, $f64_type:ty,
		$(($ord:expr, $symbol:ident, $capital:ident, $neg_cap:ident, ($($axis:expr),*)),)*
	) => {
		pub const fn $fn($($symbol: $scalar),*) -> $type { $type::new($($symbol),*) }
		#[derive(Clone, Copy, PartialEq, Eq)]
		pub struct $type { $(pub $symbol: $scalar,)* }
		impl $type {
			pub const ZERO: Self = Self::splat(0);
			pub const ONE: Self = Self::splat(1);
			pub const MIN: Self = Self::splat(<$scalar>::MIN);
			pub const MAX: Self = Self::splat(<$scalar>::MAX);
			$(pub const $capital: Self = Self::new($($axis),*);)*
			pub const AXES: [Self; $dim] = [$(Self::$capital),*];
			pub const fn new($($symbol: $scalar),*) -> Self { Self { $($symbol),* } }
			pub const fn splat(v: $scalar) -> Self { Self { $($symbol: v),* } }
			pub fn select(mask: $bvec, if_true: Self, if_false: Self) -> Self {
				Self {
					$($symbol: if mask.test($ord) { if_true.$symbol } else { if_false.$symbol },)*
				}
			}
			pub const fn from_array(a: [$scalar; $dim]) -> Self { Self::new($(a[$ord]),*) }
			pub const fn to_array(&self) -> [$scalar; $dim] { [$(self.$symbol),*] }
			pub const fn from_slice(slice: &[$scalar]) -> Self { Self::new($(slice[$ord]),*) }
			pub fn write_to_slice(self, slice: &mut [$scalar]) { $(slice[$ord] = self.$symbol;)* }
			extended!(($($extended)?), $scalar, $($symbol),*);
			truncated!(($($truncated)?), $scalar, $($symbol),*);
			pub fn dot(self, rhs: Self) -> $scalar { sep!($((self.$symbol * rhs.$symbol)),*; +) }
			pub fn dot_into_vec(self, rhs: Self) -> Self { Self::splat(self.dot(rhs)) }
			pub fn min(self, rhs: Self) -> Self {
				Self { $($symbol: self.$symbol.min(rhs.$symbol),)* }
			}
			pub fn max(self, rhs: Self) -> Self {
				Self { $($symbol: self.$symbol.max(rhs.$symbol),)* }
			}
			pub fn clamp(self, min: Self, max: Self) -> Self { self.max(min).min(max) }
			pub fn min_element(self) -> $scalar { nest!(min $(, self.$symbol)*) }
			pub fn max_element(self) -> $scalar { nest!(max $(, self.$symbol)*) }
			pub fn element_sum(self) -> $scalar { sep!($(self.$symbol),*; +) }
			pub fn element_product(self) -> $scalar { sep!($(self.$symbol),*; *) }
			cmp_fn!(cmpeq, eq, $bvec, $($symbol),*);
			cmp_fn!(cmpne, ne, $bvec, $($symbol),*);
			cmp_fn!(cmpge, ge, $bvec, $($symbol),*);
			cmp_fn!(cmpgt, gt, $bvec, $($symbol),*);
			cmp_fn!(cmple, le, $bvec, $($symbol),*);
			cmp_fn!(cmplt, lt, $bvec, $($symbol),*);
			pub fn length_squared(self) -> $scalar { self.dot(self) }
			as_fn!($b8_fn, $b8_type, $b8_scalar, $($symbol),*);
			as_fn!($i16_fn, $i16_type, i16, $($symbol),*);
			as_fn!($u16_fn, $u16_type, u16, $($symbol),*);
			as_fn!($i32_fn, $i32_type, i32, $($symbol),*);
			as_fn!($u32_fn, $u32_type, u32, $($symbol),*);
			as_fn!($i64_fn, $i64_type, i64, $($symbol),*);
			as_fn!($u64_fn, $u64_type, u64, $($symbol),*);
			as_fn!($f32_fn, $f32_type, f32, $($symbol),*);
			as_fn!($f64_fn, $f64_type, f64, $($symbol),*);
			bin_op!(wrapping_add, $($symbol),*);
			bin_op!(wrapping_sub, $($symbol),*);
			bin_op!(wrapping_mul, $($symbol),*);
			bin_op!(wrapping_div, $($symbol),*);
			bin_op!(saturating_add, $($symbol),*);
			bin_op!(saturating_sub, $($symbol),*);
			bin_op!(saturating_mul, $($symbol),*);
			bin_op!(saturating_div, $($symbol),*);
			signed!($signed, $dim, $scalar, $($symbol, $neg_cap, $ord, ($($axis),*)),*);
			cross!($dim);
		}
		impl Default for $type { fn default() -> Self { Self::ZERO } }
		bin_op_impls!(Div, div, DivAssign, div_assign, $type, $scalar, $($symbol),*);
		bin_op_impls!(Mul, mul, MulAssign, mul_assign, $type, $scalar, $($symbol),*);
		bin_op_impls!(Add, add, AddAssign, add_assign, $type, $scalar, $($symbol),*);
		bin_op_impls!(Sub, sub, SubAssign, sub_assign, $type, $scalar, $($symbol),*);
		bin_op_impls!(Rem, rem, RemAssign, rem_assign, $type, $scalar, $($symbol),*);
		impl AsRef<[$scalar; $dim]> for $type {
			fn as_ref(&self) -> &[$scalar; $dim] {
				unsafe { &*(self as *const $type as *const [$scalar; $dim]) }
			}
		}
		impl AsMut<[$scalar; $dim]> for $type {
			fn as_mut(&mut self) -> &mut [$scalar; $dim] {
				unsafe { &mut *(self as *mut $type as *mut [$scalar; $dim]) }
			}
		}
		impl_acc!(Sum, sum, ZERO, add, $type);
		impl_acc!(Product, product, ONE, mul, $type);
		impl_unary!($signed, Neg, neg, $type, $($symbol),*);
		impl_unary!(true, Not, not, $type, $($symbol),*);
		impl_bitwise!(BitAnd, bitand, $scalar, $type, $($symbol),*);
		impl_bitwise!(BitOr, bitor, $scalar, $type, $($symbol),*);
		impl_bitwise!(BitXor, bitxor, $scalar, $type, $($symbol),*);
		impl_shift!(i8, $type, $($symbol),*);
		impl_shift!(i16, $type, $($symbol),*);
		impl_shift!(i32, $type, $($symbol),*);
		impl_shift!(i64, $type, $($symbol),*);
		impl_shift!(u8, $type, $($symbol),*);
		impl_shift!(u16, $type, $($symbol),*);
		impl_shift!(u32, $type, $($symbol),*);
		impl_shift!(u64, $type, $($symbol),*);
		impl_gen_bin_op!(Shl, shl, $i32_type, $type, $($symbol),*);
		impl_gen_bin_op!(Shr, shr, $i32_type, $type, $($symbol),*);
		impl_gen_bin_op!(Shl, shl, $u32_type, $type, $($symbol),*);
		impl_gen_bin_op!(Shr, shr, $u32_type, $type, $($symbol),*);
		impl Index<usize> for $type {
			type Output = $scalar;
			fn index(&self, index: usize) -> &Self::Output {
				match index { $($ord => &self.$symbol,)* _ => panic!("index out of bounds"), }
			}
		}
		impl IndexMut<usize> for $type {
			fn index_mut(&mut self, index: usize) -> &mut Self::Output {
				match index { $($ord => &mut self.$symbol,)* _ => panic!("index out of bounds"), }
			}
		}
		impl_display!($type, $($symbol),*);
		impl Debug for $type {
			fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				fmt.debug_tuple(stringify!($type))$(.field(&self.$symbol))*.finish()
			}
		}
		impl From<[$scalar; $dim]> for $type {
			fn from(a: [$scalar; $dim]) -> Self { Self::new($(a[$ord]),*) }
		}
		impl From<$type> for [$scalar; $dim] {
			fn from(v: $type) -> Self { [$(v.$symbol),*] }
		}
		impl From<($(omit!($symbol, $scalar)),*)> for $type {
			fn from(($($symbol),*): ($(omit!($symbol, $scalar)),*)) -> Self {
				Self::new($($symbol),*)
			}
		}
		impl From<$type> for ($(omit!($symbol, $scalar)),*) {
			fn from(v: $type) -> Self { ($(v.$symbol),*) }
		}
		impl_from_truncated!(($($truncated)?), $type, $scalar, $($symbol),*);
		impl_try_from!($b8_type, $type, $scalar, $($symbol),*);
		impl_try_from!($i16_type, $type, $scalar, $($symbol),*);
		impl_try_from!($u16_type, $type, $scalar, $($symbol),*);
		impl_try_from!($i32_type, $type, $scalar, $($symbol),*);
		impl_try_from!($u32_type, $type, $scalar, $($symbol),*);
		impl_try_from!($i64_type, $type, $scalar, $($symbol),*);
		impl_try_from!($u64_type, $type, $scalar, $($symbol),*);
	};
}

decl_vec!(
	u8vec2, U8Vec2, BVec2, u8, 2, false, (U8Vec3), (), as_i8vec2, I8Vec2, i8,
	as_i16vec2, I16Vec2, as_u16vec2, U16Vec2,
	as_i32vec2, IVec2, as_u32vec2, UVec2,
	as_i64vec2, I64Vec2, as_u64vec2, U64Vec2,
	as_f32vec2, Vec2, as_f64vec2, DVec2,
	(0, x, X, NEG_X, (1, 0)),
	(1, y, Y, NEG_Y, (0, 1)),
);
impl_gvec!(U8Vec2, u8, BVec2, 2);
impl_gvec2!(U8Vec2);
impl_intvec!(U8Vec2);
impl IntVec2 for U8Vec2 {}
impl UIntVec for U8Vec2 {}
impl UIntVec2 for U8Vec2 {}
decl_vec!(
	u8vec3, U8Vec3, BVec3, u8, 3, false, (U8Vec4), (U8Vec2), as_i8vec3, I8Vec3, i8,
	as_i16vec3, I16Vec3, as_u16vec3, U16Vec3,
	as_i32vec3, IVec3, as_u32vec3, UVec3,
	as_i64vec3, I64Vec3, as_u64vec3, U64Vec3,
	as_f32vec3, Vec3, as_f64vec3, DVec3,
	(0, x, X, NEG_X, (1, 0, 0)),
	(1, y, Y, NEG_Y, (0, 1, 0)),
	(2, z, Z, NEG_Z, (0, 0, 1)),
);
impl_gvec!(U8Vec3, u8, BVec3, 3);
impl_gvec3!(U8Vec3);
impl_intvec!(U8Vec3);
impl IntVec3 for U8Vec3 {}
impl UIntVec for U8Vec3 {}
impl UIntVec3 for U8Vec3 {}
decl_vec!(
	u8vec4, U8Vec4, BVec4, u8, 4, false, (), (U8Vec3), as_i8vec4, I8Vec4, i8,
	as_i16vec4, I16Vec4, as_u16vec4, U16Vec4,
	as_i32vec4, IVec4, as_u32vec4, UVec4,
	as_i64vec4, I64Vec4, as_u64vec4, U64Vec4,
	as_f32vec4, Vec4, as_f64vec4, DVec4,
	(0, x, X, NEG_X, (1, 0, 0, 0)),
	(1, y, Y, NEG_Y, (0, 1, 0, 0)),
	(2, z, Z, NEG_Z, (0, 0, 1, 0)),
	(3, w, W, NEG_W, (0, 0, 0, 1)),
);
impl_gvec!(U8Vec4, u8, BVec4, 4);
impl_gvec4!(U8Vec4);
impl_intvec!(U8Vec4);
impl IntVec4 for U8Vec4 {}
impl UIntVec for U8Vec4 {}
impl UIntVec4 for U8Vec4 {}
decl_vec!(
	i8vec2, I8Vec2, BVec2, i8, 2, true, (I8Vec3), (), as_u8vec2, U8Vec2, u8,
	as_i16vec2, I16Vec2, as_u16vec2, U16Vec2,
	as_i32vec2, IVec2, as_u32vec2, UVec2,
	as_i64vec2, I64Vec2, as_u64vec2, U64Vec2,
	as_f32vec2, Vec2, as_f64vec2, DVec2,
	(0, x, X, NEG_X, (1, 0)),
	(1, y, Y, NEG_Y, (0, 1)),
);
impl_gvec!(I8Vec2, i8, BVec2, 2);
impl_gvec2!(I8Vec2);
impl_signedvec!(I8Vec2);
impl_signedvec2!(I8Vec2);
impl_intvec!(I8Vec2);
impl IntVec2 for I8Vec2 {}
impl SIntVec for I8Vec2 {}
impl SIntVec2 for I8Vec2 {}
decl_vec!(
	i8vec3, I8Vec3, BVec3, i8, 3, true, (I8Vec4), (I8Vec2), as_u8vec3, U8Vec3, u8,
	as_i16vec3, I16Vec3, as_u16vec3, U16Vec3,
	as_i32vec3, IVec3, as_u32vec3, UVec3,
	as_i64vec3, I64Vec3, as_u64vec3, U64Vec3,
	as_f32vec3, Vec3, as_f64vec3, DVec3,
	(0, x, X, NEG_X, (1, 0, 0)),
	(1, y, Y, NEG_Y, (0, 1, 0)),
	(2, z, Z, NEG_Z, (0, 0, 1)),
);
impl_gvec!(I8Vec3, i8, BVec3, 3);
impl_gvec3!(I8Vec3);
impl_signedvec!(I8Vec3);
impl_signedvec3!(I8Vec3);
impl_intvec!(I8Vec3);
impl IntVec3 for I8Vec3 {}
impl SIntVec for I8Vec3 {}
impl SIntVec3 for I8Vec3 {}
decl_vec!(
	i8vec4, I8Vec4, BVec4, i8, 4, true, (), (I8Vec3), as_u8vec4, U8Vec4, u8,
	as_i16vec4, I16Vec4, as_u16vec4, U16Vec4,
	as_i32vec4, IVec4, as_u32vec4, UVec4,
	as_i64vec4, I64Vec4, as_u64vec4, U64Vec4,
	as_f32vec4, Vec4, as_f64vec4, DVec4,
	(0, x, X, NEG_X, (1, 0, 0, 0)),
	(1, y, Y, NEG_Y, (0, 1, 0, 0)),
	(2, z, Z, NEG_Z, (0, 0, 1, 0)),
	(3, w, W, NEG_W, (0, 0, 0, 1)),
);
impl_gvec!(I8Vec4, i8, BVec4, 4);
impl_gvec4!(I8Vec4);
impl_signedvec!(I8Vec4);
impl_signedvec4!(I8Vec4);
impl_intvec!(I8Vec4);
impl IntVec4 for I8Vec4 {}
impl SIntVec for I8Vec4 {}
impl SIntVec4 for I8Vec4 {}

/**
Generalization of the glam `select()` function for vectors that selects each component from a
number of vectors, using the provided incides.

# Panics
Panics if the length of `indices` is less than `V::DIM`.

Panics if an index exceeds the bounds of `vecs`.
*/
pub fn select<V: GVec, I: AsPrimitive<usize>, D: Index<usize, Output = I>>(vecs: &[V], indices: &D) -> V {
	V::from_slice(&(0..V::DIM).map(|i| vecs[indices[i].as_()][i]).collect::<Vec<_>>())
}
