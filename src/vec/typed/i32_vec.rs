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

pub trait I32Vec
where
	for <'a> Self:
		Clone +
		Copy +
		PartialEq +
		Default +
		Div +
		DivAssign +
		Div<i32> +
		DivAssign<i32> +
		Mul +
		MulAssign +
		Mul<i32> +
		MulAssign<i32> +
		Add +
		AddAssign +
		Add<i32> +
		AddAssign<i32> +
		Sub +
		SubAssign +
		Sub<i32> +
		SubAssign<i32> +
		Rem +
		RemAssign +
		Rem<i32> +
		RemAssign<i32> +
		Sum +
		Sum<&'a Self> +
		Product +
		Product<&'a Self> +
		Index<usize, Output = i32> +
		IndexMut<usize, Output = i32> +
		Display +
		Debug +
		Neg +
		Eq +
		Not +
		BitAnd +
		BitAnd<i32> +
		BitOr +
		BitOr<i32> +
		BitXor +
		BitXor<i32> +
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
	fn splat(v: i32) -> Self;
	fn from_slice(slice: &[i32]) -> Self;
	fn write_to_slice(self, slice: &mut [i32]);
	fn dot(self, rhs: Self) -> i32;
	fn dot_into_vec(self, rhs: Self) -> Self;
	fn min(self, rhs: Self) -> Self;
	fn max(self, rhs: Self) -> Self;
	fn clamp(self, min: Self, max: Self) -> Self;
	fn min_element(self) -> i32;
	fn max_element(self) -> i32;
	fn element_sum(self) -> i32;
	fn element_product(self) -> i32;
	fn select(mask: Self::BVecType, if_true: Self, if_false: Self) -> Self;
	fn cmpeq(self, rhs: Self) -> Self::BVecType;
	fn cmpne(self, rhs: Self) -> Self::BVecType;
	fn cmpge(self, rhs: Self) -> Self::BVecType;
	fn cmpgt(self, rhs: Self) -> Self::BVecType;
	fn cmple(self, rhs: Self) -> Self::BVecType;
	fn cmplt(self, rhs: Self) -> Self::BVecType;
	fn length_squared(self) -> i32;
	fn abs(self) -> Self;
	fn signum(self) -> Self;
	fn is_negative_bitmask(self) -> u32;
	fn distance_squared(self, rhs: Self) -> i32;
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

impl I32Vec for glam::IVec2 {
	type BVecType = glam::BVec2;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const NEG_ONE: Self = Self::NEG_ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const DIM: usize = 2;
	fn splat(v: i32) -> Self { Self::splat(v) }
	fn from_slice(slice: &[i32]) -> Self { Self::from_slice(slice) }
	fn write_to_slice(self, slice: &mut [i32]) { self.write_to_slice(slice) }
	fn dot(self, rhs: Self) -> i32 { self.dot(rhs) }
	fn dot_into_vec(self, rhs: Self) -> Self { self.dot_into_vec(rhs) }
	fn min(self, rhs: Self) -> Self { self.min(rhs) }
	fn max(self, rhs: Self) -> Self { self.max(rhs) }
	fn clamp(self, min: Self, max: Self) -> Self { self.clamp(min, max) }
	fn min_element(self) -> i32 { self.min_element() }
	fn max_element(self) -> i32 { self.max_element() }
	fn element_sum(self) -> i32 { self.element_sum() }
	fn element_product(self) -> i32 { self.element_product() }
	fn select(mask: Self::BVecType, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVecType { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVecType { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVecType { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVecType { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVecType { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVecType { self.cmplt(rhs) }
	fn length_squared(self) -> i32 { self.length_squared() }
	fn abs(self) -> Self { self.abs() }
	fn signum(self) -> Self { self.signum() }
	fn is_negative_bitmask(self) -> u32 { self.is_negative_bitmask() }
	fn distance_squared(self, rhs: Self) -> i32 { self.distance_squared(rhs) }
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

impl I32Vec for glam::IVec3 {
	type BVecType = glam::BVec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const NEG_ONE: Self = Self::NEG_ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const DIM: usize = 3;
	fn splat(v: i32) -> Self { Self::splat(v) }
	fn from_slice(slice: &[i32]) -> Self { Self::from_slice(slice) }
	fn write_to_slice(self, slice: &mut [i32]) { self.write_to_slice(slice) }
	fn dot(self, rhs: Self) -> i32 { self.dot(rhs) }
	fn dot_into_vec(self, rhs: Self) -> Self { self.dot_into_vec(rhs) }
	fn min(self, rhs: Self) -> Self { self.min(rhs) }
	fn max(self, rhs: Self) -> Self { self.max(rhs) }
	fn clamp(self, min: Self, max: Self) -> Self { self.clamp(min, max) }
	fn min_element(self) -> i32 { self.min_element() }
	fn max_element(self) -> i32 { self.max_element() }
	fn element_sum(self) -> i32 { self.element_sum() }
	fn element_product(self) -> i32 { self.element_product() }
	fn select(mask: Self::BVecType, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVecType { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVecType { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVecType { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVecType { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVecType { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVecType { self.cmplt(rhs) }
	fn length_squared(self) -> i32 { self.length_squared() }
	fn abs(self) -> Self { self.abs() }
	fn signum(self) -> Self { self.signum() }
	fn is_negative_bitmask(self) -> u32 { self.is_negative_bitmask() }
	fn distance_squared(self, rhs: Self) -> i32 { self.distance_squared(rhs) }
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

impl I32Vec for glam::IVec4 {
	type BVecType = glam::BVec4;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const NEG_ONE: Self = Self::NEG_ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const DIM: usize = 4;
	fn splat(v: i32) -> Self { Self::splat(v) }
	fn from_slice(slice: &[i32]) -> Self { Self::from_slice(slice) }
	fn write_to_slice(self, slice: &mut [i32]) { self.write_to_slice(slice) }
	fn dot(self, rhs: Self) -> i32 { self.dot(rhs) }
	fn dot_into_vec(self, rhs: Self) -> Self { self.dot_into_vec(rhs) }
	fn min(self, rhs: Self) -> Self { self.min(rhs) }
	fn max(self, rhs: Self) -> Self { self.max(rhs) }
	fn clamp(self, min: Self, max: Self) -> Self { self.clamp(min, max) }
	fn min_element(self) -> i32 { self.min_element() }
	fn max_element(self) -> i32 { self.max_element() }
	fn element_sum(self) -> i32 { self.element_sum() }
	fn element_product(self) -> i32 { self.element_product() }
	fn select(mask: Self::BVecType, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVecType { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVecType { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVecType { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVecType { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVecType { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVecType { self.cmplt(rhs) }
	fn length_squared(self) -> i32 { self.length_squared() }
	fn abs(self) -> Self { self.abs() }
	fn signum(self) -> Self { self.signum() }
	fn is_negative_bitmask(self) -> u32 { self.is_negative_bitmask() }
	fn distance_squared(self, rhs: Self) -> i32 { self.distance_squared(rhs) }
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
