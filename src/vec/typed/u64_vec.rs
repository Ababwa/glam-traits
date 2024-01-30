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
		BitAnd,
		BitOr,
		BitXor,
		Not,
		Shl,
		Shr,
	},
};
use crate::GBVec;

pub trait U64Vec
where
	for <'a> Self:
		Clone +
		Copy +
		PartialEq +
		Default +
		Div<Output = Self> +
		DivAssign +
		Div<u64, Output = Self> +
		DivAssign<u64> +
		Mul<Output = Self> +
		MulAssign +
		Mul<u64, Output = Self> +
		MulAssign<u64> +
		Add<Output = Self> +
		AddAssign +
		Add<u64, Output = Self> +
		AddAssign<u64> +
		Sub<Output = Self> +
		SubAssign +
		Sub<u64, Output = Self> +
		SubAssign<u64> +
		Rem<Output = Self> +
		RemAssign +
		Rem<u64, Output = Self> +
		RemAssign<u64> +
		Sum +
		Sum<&'a Self> +
		Product +
		Product<&'a Self> +
		Index<usize, Output = u64> +
		IndexMut<usize, Output = u64> +
		Display +
		Debug +
		Eq +
		Not +
		BitAnd +
		BitAnd<u64> +
		BitOr +
		BitOr<u64> +
		BitXor +
		BitXor<u64> +
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
	const MIN: Self;
	const MAX: Self;
	const DIM: usize;
	fn splat(v: u64) -> Self;
	fn from_slice(slice: &[u64]) -> Self;
	fn write_to_slice(self, slice: &mut [u64]);
	fn dot(self, rhs: Self) -> u64;
	fn dot_into_vec(self, rhs: Self) -> Self;
	fn min(self, rhs: Self) -> Self;
	fn max(self, rhs: Self) -> Self;
	fn clamp(self, min: Self, max: Self) -> Self;
	fn min_element(self) -> u64;
	fn max_element(self) -> u64;
	fn element_sum(self) -> u64;
	fn element_product(self) -> u64;
	fn select(mask: Self::BVecType, if_true: Self, if_false: Self) -> Self;
	fn cmpeq(self, rhs: Self) -> Self::BVecType;
	fn cmpne(self, rhs: Self) -> Self::BVecType;
	fn cmpge(self, rhs: Self) -> Self::BVecType;
	fn cmpgt(self, rhs: Self) -> Self::BVecType;
	fn cmple(self, rhs: Self) -> Self::BVecType;
	fn cmplt(self, rhs: Self) -> Self::BVecType;
	fn length_squared(self) -> u64;
	fn wrapping_add(self, rhs: Self) -> Self;
	fn wrapping_sub(self, rhs: Self) -> Self;
	fn wrapping_mul(self, rhs: Self) -> Self;
	fn wrapping_div(self, rhs: Self) -> Self;
	fn saturating_add(self, rhs: Self) -> Self;
	fn saturating_sub(self, rhs: Self) -> Self;
	fn saturating_mul(self, rhs: Self) -> Self;
	fn saturating_div(self, rhs: Self) -> Self;
}

impl U64Vec for glam::U64Vec2 {
	type BVecType = glam::BVec2;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const DIM: usize = 2;
	fn splat(v: u64) -> Self { Self::splat(v) }
	fn from_slice(slice: &[u64]) -> Self { Self::from_slice(slice) }
	fn write_to_slice(self, slice: &mut [u64]) { self.write_to_slice(slice) }
	fn dot(self, rhs: Self) -> u64 { self.dot(rhs) }
	fn dot_into_vec(self, rhs: Self) -> Self { self.dot_into_vec(rhs) }
	fn min(self, rhs: Self) -> Self { self.min(rhs) }
	fn max(self, rhs: Self) -> Self { self.max(rhs) }
	fn clamp(self, min: Self, max: Self) -> Self { self.clamp(min, max) }
	fn min_element(self) -> u64 { self.min_element() }
	fn max_element(self) -> u64 { self.max_element() }
	fn element_sum(self) -> u64 { self.element_sum() }
	fn element_product(self) -> u64 { self.element_product() }
	fn select(mask: Self::BVecType, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVecType { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVecType { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVecType { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVecType { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVecType { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVecType { self.cmplt(rhs) }
	fn length_squared(self) -> u64 { self.length_squared() }
	fn wrapping_add(self, rhs: Self) -> Self { self.wrapping_add(rhs) }
	fn wrapping_sub(self, rhs: Self) -> Self { self.wrapping_sub(rhs) }
	fn wrapping_mul(self, rhs: Self) -> Self { self.wrapping_mul(rhs) }
	fn wrapping_div(self, rhs: Self) -> Self { self.wrapping_div(rhs) }
	fn saturating_add(self, rhs: Self) -> Self { self.saturating_add(rhs) }
	fn saturating_sub(self, rhs: Self) -> Self { self.saturating_sub(rhs) }
	fn saturating_mul(self, rhs: Self) -> Self { self.saturating_mul(rhs) }
	fn saturating_div(self, rhs: Self) -> Self { self.saturating_div(rhs) }
}

impl U64Vec for glam::U64Vec3 {
	type BVecType = glam::BVec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const DIM: usize = 3;
	fn splat(v: u64) -> Self { Self::splat(v) }
	fn from_slice(slice: &[u64]) -> Self { Self::from_slice(slice) }
	fn write_to_slice(self, slice: &mut [u64]) { self.write_to_slice(slice) }
	fn dot(self, rhs: Self) -> u64 { self.dot(rhs) }
	fn dot_into_vec(self, rhs: Self) -> Self { self.dot_into_vec(rhs) }
	fn min(self, rhs: Self) -> Self { self.min(rhs) }
	fn max(self, rhs: Self) -> Self { self.max(rhs) }
	fn clamp(self, min: Self, max: Self) -> Self { self.clamp(min, max) }
	fn min_element(self) -> u64 { self.min_element() }
	fn max_element(self) -> u64 { self.max_element() }
	fn element_sum(self) -> u64 { self.element_sum() }
	fn element_product(self) -> u64 { self.element_product() }
	fn select(mask: Self::BVecType, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVecType { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVecType { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVecType { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVecType { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVecType { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVecType { self.cmplt(rhs) }
	fn length_squared(self) -> u64 { self.length_squared() }
	fn wrapping_add(self, rhs: Self) -> Self { self.wrapping_add(rhs) }
	fn wrapping_sub(self, rhs: Self) -> Self { self.wrapping_sub(rhs) }
	fn wrapping_mul(self, rhs: Self) -> Self { self.wrapping_mul(rhs) }
	fn wrapping_div(self, rhs: Self) -> Self { self.wrapping_div(rhs) }
	fn saturating_add(self, rhs: Self) -> Self { self.saturating_add(rhs) }
	fn saturating_sub(self, rhs: Self) -> Self { self.saturating_sub(rhs) }
	fn saturating_mul(self, rhs: Self) -> Self { self.saturating_mul(rhs) }
	fn saturating_div(self, rhs: Self) -> Self { self.saturating_div(rhs) }
}

impl U64Vec for glam::U64Vec4 {
	type BVecType = glam::BVec4;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const DIM: usize = 4;
	fn splat(v: u64) -> Self { Self::splat(v) }
	fn from_slice(slice: &[u64]) -> Self { Self::from_slice(slice) }
	fn write_to_slice(self, slice: &mut [u64]) { self.write_to_slice(slice) }
	fn dot(self, rhs: Self) -> u64 { self.dot(rhs) }
	fn dot_into_vec(self, rhs: Self) -> Self { self.dot_into_vec(rhs) }
	fn min(self, rhs: Self) -> Self { self.min(rhs) }
	fn max(self, rhs: Self) -> Self { self.max(rhs) }
	fn clamp(self, min: Self, max: Self) -> Self { self.clamp(min, max) }
	fn min_element(self) -> u64 { self.min_element() }
	fn max_element(self) -> u64 { self.max_element() }
	fn element_sum(self) -> u64 { self.element_sum() }
	fn element_product(self) -> u64 { self.element_product() }
	fn select(mask: Self::BVecType, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVecType { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVecType { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVecType { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVecType { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVecType { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVecType { self.cmplt(rhs) }
	fn length_squared(self) -> u64 { self.length_squared() }
	fn wrapping_add(self, rhs: Self) -> Self { self.wrapping_add(rhs) }
	fn wrapping_sub(self, rhs: Self) -> Self { self.wrapping_sub(rhs) }
	fn wrapping_mul(self, rhs: Self) -> Self { self.wrapping_mul(rhs) }
	fn wrapping_div(self, rhs: Self) -> Self { self.wrapping_div(rhs) }
	fn saturating_add(self, rhs: Self) -> Self { self.saturating_add(rhs) }
	fn saturating_sub(self, rhs: Self) -> Self { self.saturating_sub(rhs) }
	fn saturating_mul(self, rhs: Self) -> Self { self.saturating_mul(rhs) }
	fn saturating_div(self, rhs: Self) -> Self { self.saturating_div(rhs) }
}
