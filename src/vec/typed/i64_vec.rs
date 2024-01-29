use std::{
	fmt::{Debug, Display},
	iter::{Product, Sum},
	ops::{
		Add,
		AddAssign,
		Div,
		DivAssign,
		Index,
		IndexMut,
		Mul,
		MulAssign,
		Rem,
		RemAssign,
		Sub,
		SubAssign,
		Neg,
		BitAnd,
		BitOr,
		BitXor,
		Not,
		Shl,
		Shr,
	},
};
use crate::GBVec;

pub trait I64Vec
where
	for <'a> Self:
		Clone +
		Copy +
		PartialEq +
		Default +
		Div<Output = Self> +
		DivAssign +
		Div<i64, Output = Self> +
		DivAssign<i64> +
		Mul<Output = Self> +
		MulAssign +
		Mul<i64, Output = Self> +
		MulAssign<i64> +
		Add<Output = Self> +
		AddAssign +
		Add<i64, Output = Self> +
		AddAssign<i64> +
		Sub<Output = Self> +
		SubAssign +
		Sub<i64, Output = Self> +
		SubAssign<i64> +
		Rem<Output = Self> +
		RemAssign +
		Rem<i64, Output = Self> +
		RemAssign<i64> +
		Sum +
		Sum<&'a Self> +
		Product +
		Product<&'a Self> +
		Index<usize, Output = i64> +
		IndexMut<usize, Output = i64> +
		Display +
		Debug +
		Neg +
		Eq +
		Not +
		BitAnd +
		BitAnd<i64> +
		BitOr +
		BitOr<i64> +
		BitXor +
		BitXor<i64> +
		Shl<i8> + Shr<i8> + Shl<u8> + Shr<u8> +
		Shl<i16> + Shr<i16> + Shl<u16> + Shr<u16> +
		Shl<i32> + Shr<i32> + Shl<u32> + Shr<u32> +
		Shl<i64> + Shr<i64> + Shl<u64> + Shr<u64> +
	,
	Self::BVecType: GBVec,
{
	type BVecType;
	const ZERO: Self;
	const ONE: Self;
	const NEG_ONE: Self;
	const MIN: Self;
	const MAX: Self;
	const DIM: usize;
	fn splat(v: i64) -> Self;
	fn from_slice(slice: &[i64]) -> Self;
	fn write_to_slice(self, slice: &mut [i64]);
	fn dot(self, rhs: Self) -> i64;
	fn dot_into_vec(self, rhs: Self) -> Self;
	fn min(self, rhs: Self) -> Self;
	fn max(self, rhs: Self) -> Self;
	fn clamp(self, min: Self, max: Self) -> Self;
	fn min_element(self) -> i64;
	fn max_element(self) -> i64;
	fn select(mask: Self::BVecType, if_true: Self, if_false: Self) -> Self;
	fn cmpeq(self, rhs: Self) -> Self::BVecType;
	fn cmpne(self, rhs: Self) -> Self::BVecType;
	fn cmpge(self, rhs: Self) -> Self::BVecType;
	fn cmpgt(self, rhs: Self) -> Self::BVecType;
	fn cmple(self, rhs: Self) -> Self::BVecType;
	fn cmplt(self, rhs: Self) -> Self::BVecType;
	fn length_squared(self) -> i64;
	fn abs(self) -> Self;
	fn signum(self) -> Self;
	fn is_negative_bitmask(self) -> u32;
	fn distance_squared(self, rhs: Self) -> i64;
	fn div_euclid(self, rhs: Self) -> Self;
	fn rem_euclid(self, rhs: Self) -> Self;
	fn wrapping_add(self, rhs: Self) -> Self;
	fn wrapping_sub(self, rhs: Self) -> Self;
	fn wrapping_mul(self, rhs: Self) -> Self;
	fn wrapping_div(self, rhs: Self) -> Self;
	fn saturating_add(self, rhs: Self) -> Self;
	fn saturating_sub(self, rhs: Self) -> Self;
	fn saturating_mul(self, rhs: Self) -> Self;
	fn saturating_div(self, rhs: Self) -> Self;
}

impl I64Vec for glam::I64Vec2 {
	type BVecType = glam::BVec2;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const NEG_ONE: Self = Self::NEG_ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const DIM: usize = 2;
	fn splat(v: i64) -> Self { Self::splat(v) }
	fn from_slice(slice: &[i64]) -> Self { Self::from_slice(slice) }
	fn write_to_slice(self, slice: &mut [i64]) { self.write_to_slice(slice) }
	fn dot(self, rhs: Self) -> i64 { self.dot(rhs) }
	fn dot_into_vec(self, rhs: Self) -> Self { self.dot_into_vec(rhs) }
	fn min(self, rhs: Self) -> Self { self.min(rhs) }
	fn max(self, rhs: Self) -> Self { self.max(rhs) }
	fn clamp(self, min: Self, max: Self) -> Self { self.clamp(min, max) }
	fn min_element(self) -> i64 { self.min_element() }
	fn max_element(self) -> i64 { self.max_element() }
	fn select(mask: Self::BVecType, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVecType { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVecType { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVecType { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVecType { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVecType { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVecType { self.cmplt(rhs) }
	fn length_squared(self) -> i64 { self.length_squared() }
	fn abs(self) -> Self { self.abs() }
	fn signum(self) -> Self { self.signum() }
	fn is_negative_bitmask(self) -> u32 { self.is_negative_bitmask() }
	fn distance_squared(self, rhs: Self) -> i64 { self.distance_squared(rhs) }
	fn div_euclid(self, rhs: Self) -> Self { self.div_euclid(rhs) }
	fn rem_euclid(self, rhs: Self) -> Self { self.rem_euclid(rhs) }
	fn wrapping_add(self, rhs: Self) -> Self { self.wrapping_add(rhs) }
	fn wrapping_sub(self, rhs: Self) -> Self { self.wrapping_sub(rhs) }
	fn wrapping_mul(self, rhs: Self) -> Self { self.wrapping_mul(rhs) }
	fn wrapping_div(self, rhs: Self) -> Self { self.wrapping_div(rhs) }
	fn saturating_add(self, rhs: Self) -> Self { self.saturating_add(rhs) }
	fn saturating_sub(self, rhs: Self) -> Self { self.saturating_sub(rhs) }
	fn saturating_mul(self, rhs: Self) -> Self { self.saturating_mul(rhs) }
	fn saturating_div(self, rhs: Self) -> Self { self.saturating_div(rhs) }
}

impl I64Vec for glam::I64Vec3 {
	type BVecType = glam::BVec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const NEG_ONE: Self = Self::NEG_ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const DIM: usize = 3;
	fn splat(v: i64) -> Self { Self::splat(v) }
	fn from_slice(slice: &[i64]) -> Self { Self::from_slice(slice) }
	fn write_to_slice(self, slice: &mut [i64]) { self.write_to_slice(slice) }
	fn dot(self, rhs: Self) -> i64 { self.dot(rhs) }
	fn dot_into_vec(self, rhs: Self) -> Self { self.dot_into_vec(rhs) }
	fn min(self, rhs: Self) -> Self { self.min(rhs) }
	fn max(self, rhs: Self) -> Self { self.max(rhs) }
	fn clamp(self, min: Self, max: Self) -> Self { self.clamp(min, max) }
	fn min_element(self) -> i64 { self.min_element() }
	fn max_element(self) -> i64 { self.max_element() }
	fn select(mask: Self::BVecType, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVecType { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVecType { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVecType { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVecType { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVecType { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVecType { self.cmplt(rhs) }
	fn length_squared(self) -> i64 { self.length_squared() }
	fn abs(self) -> Self { self.abs() }
	fn signum(self) -> Self { self.signum() }
	fn is_negative_bitmask(self) -> u32 { self.is_negative_bitmask() }
	fn distance_squared(self, rhs: Self) -> i64 { self.distance_squared(rhs) }
	fn div_euclid(self, rhs: Self) -> Self { self.div_euclid(rhs) }
	fn rem_euclid(self, rhs: Self) -> Self { self.rem_euclid(rhs) }
	fn wrapping_add(self, rhs: Self) -> Self { self.wrapping_add(rhs) }
	fn wrapping_sub(self, rhs: Self) -> Self { self.wrapping_sub(rhs) }
	fn wrapping_mul(self, rhs: Self) -> Self { self.wrapping_mul(rhs) }
	fn wrapping_div(self, rhs: Self) -> Self { self.wrapping_div(rhs) }
	fn saturating_add(self, rhs: Self) -> Self { self.saturating_add(rhs) }
	fn saturating_sub(self, rhs: Self) -> Self { self.saturating_sub(rhs) }
	fn saturating_mul(self, rhs: Self) -> Self { self.saturating_mul(rhs) }
	fn saturating_div(self, rhs: Self) -> Self { self.saturating_div(rhs) }
}

impl I64Vec for glam::I64Vec4 {
	type BVecType = glam::BVec4;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const NEG_ONE: Self = Self::NEG_ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const DIM: usize = 4;
	fn splat(v: i64) -> Self { Self::splat(v) }
	fn from_slice(slice: &[i64]) -> Self { Self::from_slice(slice) }
	fn write_to_slice(self, slice: &mut [i64]) { self.write_to_slice(slice) }
	fn dot(self, rhs: Self) -> i64 { self.dot(rhs) }
	fn dot_into_vec(self, rhs: Self) -> Self { self.dot_into_vec(rhs) }
	fn min(self, rhs: Self) -> Self { self.min(rhs) }
	fn max(self, rhs: Self) -> Self { self.max(rhs) }
	fn clamp(self, min: Self, max: Self) -> Self { self.clamp(min, max) }
	fn min_element(self) -> i64 { self.min_element() }
	fn max_element(self) -> i64 { self.max_element() }
	fn select(mask: Self::BVecType, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVecType { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVecType { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVecType { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVecType { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVecType { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVecType { self.cmplt(rhs) }
	fn length_squared(self) -> i64 { self.length_squared() }
	fn abs(self) -> Self { self.abs() }
	fn signum(self) -> Self { self.signum() }
	fn is_negative_bitmask(self) -> u32 { self.is_negative_bitmask() }
	fn distance_squared(self, rhs: Self) -> i64 { self.distance_squared(rhs) }
	fn div_euclid(self, rhs: Self) -> Self { self.div_euclid(rhs) }
	fn rem_euclid(self, rhs: Self) -> Self { self.rem_euclid(rhs) }
	fn wrapping_add(self, rhs: Self) -> Self { self.wrapping_add(rhs) }
	fn wrapping_sub(self, rhs: Self) -> Self { self.wrapping_sub(rhs) }
	fn wrapping_mul(self, rhs: Self) -> Self { self.wrapping_mul(rhs) }
	fn wrapping_div(self, rhs: Self) -> Self { self.wrapping_div(rhs) }
	fn saturating_add(self, rhs: Self) -> Self { self.saturating_add(rhs) }
	fn saturating_sub(self, rhs: Self) -> Self { self.saturating_sub(rhs) }
	fn saturating_mul(self, rhs: Self) -> Self { self.saturating_mul(rhs) }
	fn saturating_div(self, rhs: Self) -> Self { self.saturating_div(rhs) }
}
