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
	},
};
use crate::GBVec;

pub trait F64Vec
where
	for <'a> Self:
		Clone +
		Copy +
		PartialEq +
		Default +
		Div<Output = Self> +
		DivAssign +
		Div<f64, Output = Self> +
		DivAssign<f64> +
		Mul<Output = Self> +
		MulAssign +
		Mul<f64, Output = Self> +
		MulAssign<f64> +
		Add<Output = Self> +
		AddAssign +
		Add<f64, Output = Self> +
		AddAssign<f64> +
		Sub<Output = Self> +
		SubAssign +
		Sub<f64, Output = Self> +
		SubAssign<f64> +
		Rem<Output = Self> +
		RemAssign +
		Rem<f64, Output = Self> +
		RemAssign<f64> +
		Sum +
		Sum<&'a Self> +
		Product +
		Product<&'a Self> +
		Index<usize, Output = f64> +
		IndexMut<usize, Output = f64> +
		Display +
		Debug +
		Neg +
	,
	Self::BVecType: GBVec,
{
	type BVecType;
	const ZERO: Self;
	const ONE: Self;
	const NEG_ONE: Self;
	const MIN: Self;
	const MAX: Self;
	const NAN: Self;
	const INFINITY: Self;
	const NEG_INFINITY: Self;
	const DIM: usize;
	fn splat(v: f64) -> Self;
	fn from_slice(slice: &[f64]) -> Self;
	fn write_to_slice(self, slice: &mut [f64]);
	fn dot(self, rhs: Self) -> f64;
	fn dot_into_vec(self, rhs: Self) -> Self;
	fn min(self, rhs: Self) -> Self;
	fn max(self, rhs: Self) -> Self;
	fn clamp(self, min: Self, max: Self) -> Self;
	fn min_element(self) -> f64;
	fn max_element(self) -> f64;
	fn element_sum(self) -> f64;
	fn element_product(self) -> f64;
	fn select(mask: Self::BVecType, if_true: Self, if_false: Self) -> Self;
	fn cmpeq(self, rhs: Self) -> Self::BVecType;
	fn cmpne(self, rhs: Self) -> Self::BVecType;
	fn cmpge(self, rhs: Self) -> Self::BVecType;
	fn cmpgt(self, rhs: Self) -> Self::BVecType;
	fn cmple(self, rhs: Self) -> Self::BVecType;
	fn cmplt(self, rhs: Self) -> Self::BVecType;
	fn length_squared(self) -> f64;
	fn abs(self) -> Self;
	fn signum(self) -> Self;
	fn is_negative_bitmask(self) -> u32;
	fn distance_squared(self, rhs: Self) -> f64;
	fn div_euclid(self, rhs: Self) -> Self;
	fn rem_euclid(self, rhs: Self) -> Self;
	fn copysign(self, rhs: Self) -> Self;
	fn is_finite(self) -> bool;
	fn is_nan(self) -> bool;
	fn length(self) -> f64;
	fn length_recip(self) -> f64;
	fn distance(self, rhs: Self) -> f64;
	fn normalize(self) -> Self;
	fn try_normalize(self) -> Option<Self>;
	fn normalize_or_zero(self) -> Self;
	fn is_normalized(self) -> bool;
	fn project_onto(self, rhs: Self) -> Self;
	fn reject_from(self, rhs: Self) -> Self;
	fn project_onto_normalized(self, rhs: Self) -> Self;
	fn reject_from_normalized(self, rhs: Self) -> Self;
	fn round(self) -> Self;
	fn floor(self) -> Self;
	fn ceil(self) -> Self;
	fn trunc(self) -> Self;
	fn fract(self) -> Self;
	fn exp(self) -> Self;
	fn powf(self, n: f64) -> Self;
	fn recip(self) -> Self;
	fn lerp(self, rhs: Self, s: f64) -> Self;
	fn midpoint(self, rhs: Self) -> Self;
	fn abs_diff_eq(self, rhs: Self, max_abs_diff: f64) -> bool;
	fn clamp_length(self, min: f64, max: f64) -> Self;
	fn clamp_length_max(self, max: f64) -> Self;
	fn clamp_length_min(self, min: f64) -> Self;
	fn mul_add(self, a: Self, b: Self) -> Self;
}

impl F64Vec for glam::DVec2 {
	type BVecType = glam::BVec2;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const NEG_ONE: Self = Self::NEG_ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const NAN: Self = Self::NAN;
	const INFINITY: Self = Self::INFINITY;
	const NEG_INFINITY: Self = Self::NEG_INFINITY;
	const DIM: usize = 2;
	fn splat(v: f64) -> Self { Self::splat(v) }
	fn from_slice(slice: &[f64]) -> Self { Self::from_slice(slice) }
	fn write_to_slice(self, slice: &mut [f64]) { self.write_to_slice(slice) }
	fn dot(self, rhs: Self) -> f64 { self.dot(rhs) }
	fn dot_into_vec(self, rhs: Self) -> Self { self.dot_into_vec(rhs) }
	fn min(self, rhs: Self) -> Self { self.min(rhs) }
	fn max(self, rhs: Self) -> Self { self.max(rhs) }
	fn clamp(self, min: Self, max: Self) -> Self { self.clamp(min, max) }
	fn min_element(self) -> f64 { self.min_element() }
	fn max_element(self) -> f64 { self.max_element() }
	fn element_sum(self) -> f64 { self.element_sum() }
	fn element_product(self) -> f64 { self.element_product() }
	fn select(mask: Self::BVecType, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVecType { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVecType { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVecType { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVecType { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVecType { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVecType { self.cmplt(rhs) }
	fn length_squared(self) -> f64 { self.length_squared() }
	fn abs(self) -> Self { self.abs() }
	fn signum(self) -> Self { self.signum() }
	fn is_negative_bitmask(self) -> u32 { self.is_negative_bitmask() }
	fn distance_squared(self, rhs: Self) -> f64 { self.distance_squared(rhs) }
	fn div_euclid(self, rhs: Self) -> Self { self.div_euclid(rhs) }
	fn rem_euclid(self, rhs: Self) -> Self { self.rem_euclid(rhs) }
	fn copysign(self, rhs: Self) -> Self { self.copysign(rhs) }
	fn is_finite(self) -> bool { self.is_finite() }
	fn is_nan(self) -> bool { self.is_nan() }
	fn length(self) -> f64 { self.length() }
	fn length_recip(self) -> f64 { self.length_recip() }
	fn distance(self, rhs: Self) -> f64 { self.distance(rhs) }
	fn normalize(self) -> Self { self.normalize() }
	fn try_normalize(self) -> Option<Self> { self.try_normalize() }
	fn normalize_or_zero(self) -> Self { self.normalize_or_zero() }
	fn is_normalized(self) -> bool { self.is_normalized() }
	fn project_onto(self, rhs: Self) -> Self { self.project_onto(rhs) }
	fn reject_from(self, rhs: Self) -> Self { self.reject_from(rhs) }
	fn project_onto_normalized(self, rhs: Self) -> Self { self.project_onto_normalized(rhs) }
	fn reject_from_normalized(self, rhs: Self) -> Self { self.reject_from_normalized(rhs) }
	fn round(self) -> Self { self.round() }
	fn floor(self) -> Self { self.floor() }
	fn ceil(self) -> Self { self.ceil() }
	fn trunc(self) -> Self { self.trunc() }
	fn fract(self) -> Self { self.fract() }
	fn exp(self) -> Self { self.exp() }
	fn powf(self, n: f64) -> Self { self.powf(n) }
	fn recip(self) -> Self { self.recip() }
	fn lerp(self, rhs: Self, s: f64) -> Self { self.lerp(rhs, s) }
	fn midpoint(self, rhs: Self) -> Self { self.midpoint(rhs) }
	fn abs_diff_eq(self, rhs: Self, max_abs_diff: f64) -> bool { self.abs_diff_eq(rhs, max_abs_diff) }
	fn clamp_length(self, min: f64, max: f64) -> Self { self.clamp_length(min, max) }
	fn clamp_length_max(self, max: f64) -> Self { self.clamp_length_max(max) }
	fn clamp_length_min(self, min: f64) -> Self { self.clamp_length_min(min) }
	fn mul_add(self, a: Self, b: Self) -> Self { self.mul_add(a, b) }
}

impl F64Vec for glam::DVec3 {
	type BVecType = glam::BVec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const NEG_ONE: Self = Self::NEG_ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const NAN: Self = Self::NAN;
	const INFINITY: Self = Self::INFINITY;
	const NEG_INFINITY: Self = Self::NEG_INFINITY;
	const DIM: usize = 3;
	fn splat(v: f64) -> Self { Self::splat(v) }
	fn from_slice(slice: &[f64]) -> Self { Self::from_slice(slice) }
	fn write_to_slice(self, slice: &mut [f64]) { self.write_to_slice(slice) }
	fn dot(self, rhs: Self) -> f64 { self.dot(rhs) }
	fn dot_into_vec(self, rhs: Self) -> Self { self.dot_into_vec(rhs) }
	fn min(self, rhs: Self) -> Self { self.min(rhs) }
	fn max(self, rhs: Self) -> Self { self.max(rhs) }
	fn clamp(self, min: Self, max: Self) -> Self { self.clamp(min, max) }
	fn min_element(self) -> f64 { self.min_element() }
	fn max_element(self) -> f64 { self.max_element() }
	fn element_sum(self) -> f64 { self.element_sum() }
	fn element_product(self) -> f64 { self.element_product() }
	fn select(mask: Self::BVecType, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVecType { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVecType { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVecType { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVecType { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVecType { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVecType { self.cmplt(rhs) }
	fn length_squared(self) -> f64 { self.length_squared() }
	fn abs(self) -> Self { self.abs() }
	fn signum(self) -> Self { self.signum() }
	fn is_negative_bitmask(self) -> u32 { self.is_negative_bitmask() }
	fn distance_squared(self, rhs: Self) -> f64 { self.distance_squared(rhs) }
	fn div_euclid(self, rhs: Self) -> Self { self.div_euclid(rhs) }
	fn rem_euclid(self, rhs: Self) -> Self { self.rem_euclid(rhs) }
	fn copysign(self, rhs: Self) -> Self { self.copysign(rhs) }
	fn is_finite(self) -> bool { self.is_finite() }
	fn is_nan(self) -> bool { self.is_nan() }
	fn length(self) -> f64 { self.length() }
	fn length_recip(self) -> f64 { self.length_recip() }
	fn distance(self, rhs: Self) -> f64 { self.distance(rhs) }
	fn normalize(self) -> Self { self.normalize() }
	fn try_normalize(self) -> Option<Self> { self.try_normalize() }
	fn normalize_or_zero(self) -> Self { self.normalize_or_zero() }
	fn is_normalized(self) -> bool { self.is_normalized() }
	fn project_onto(self, rhs: Self) -> Self { self.project_onto(rhs) }
	fn reject_from(self, rhs: Self) -> Self { self.reject_from(rhs) }
	fn project_onto_normalized(self, rhs: Self) -> Self { self.project_onto_normalized(rhs) }
	fn reject_from_normalized(self, rhs: Self) -> Self { self.reject_from_normalized(rhs) }
	fn round(self) -> Self { self.round() }
	fn floor(self) -> Self { self.floor() }
	fn ceil(self) -> Self { self.ceil() }
	fn trunc(self) -> Self { self.trunc() }
	fn fract(self) -> Self { self.fract() }
	fn exp(self) -> Self { self.exp() }
	fn powf(self, n: f64) -> Self { self.powf(n) }
	fn recip(self) -> Self { self.recip() }
	fn lerp(self, rhs: Self, s: f64) -> Self { self.lerp(rhs, s) }
	fn midpoint(self, rhs: Self) -> Self { self.midpoint(rhs) }
	fn abs_diff_eq(self, rhs: Self, max_abs_diff: f64) -> bool { self.abs_diff_eq(rhs, max_abs_diff) }
	fn clamp_length(self, min: f64, max: f64) -> Self { self.clamp_length(min, max) }
	fn clamp_length_max(self, max: f64) -> Self { self.clamp_length_max(max) }
	fn clamp_length_min(self, min: f64) -> Self { self.clamp_length_min(min) }
	fn mul_add(self, a: Self, b: Self) -> Self { self.mul_add(a, b) }
}

impl F64Vec for glam::DVec4 {
	type BVecType = glam::BVec4;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const NEG_ONE: Self = Self::NEG_ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const NAN: Self = Self::NAN;
	const INFINITY: Self = Self::INFINITY;
	const NEG_INFINITY: Self = Self::NEG_INFINITY;
	const DIM: usize = 4;
	fn splat(v: f64) -> Self { Self::splat(v) }
	fn from_slice(slice: &[f64]) -> Self { Self::from_slice(slice) }
	fn write_to_slice(self, slice: &mut [f64]) { self.write_to_slice(slice) }
	fn dot(self, rhs: Self) -> f64 { self.dot(rhs) }
	fn dot_into_vec(self, rhs: Self) -> Self { self.dot_into_vec(rhs) }
	fn min(self, rhs: Self) -> Self { self.min(rhs) }
	fn max(self, rhs: Self) -> Self { self.max(rhs) }
	fn clamp(self, min: Self, max: Self) -> Self { self.clamp(min, max) }
	fn min_element(self) -> f64 { self.min_element() }
	fn max_element(self) -> f64 { self.max_element() }
	fn element_sum(self) -> f64 { self.element_sum() }
	fn element_product(self) -> f64 { self.element_product() }
	fn select(mask: Self::BVecType, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVecType { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVecType { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVecType { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVecType { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVecType { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVecType { self.cmplt(rhs) }
	fn length_squared(self) -> f64 { self.length_squared() }
	fn abs(self) -> Self { self.abs() }
	fn signum(self) -> Self { self.signum() }
	fn is_negative_bitmask(self) -> u32 { self.is_negative_bitmask() }
	fn distance_squared(self, rhs: Self) -> f64 { self.distance_squared(rhs) }
	fn div_euclid(self, rhs: Self) -> Self { self.div_euclid(rhs) }
	fn rem_euclid(self, rhs: Self) -> Self { self.rem_euclid(rhs) }
	fn copysign(self, rhs: Self) -> Self { self.copysign(rhs) }
	fn is_finite(self) -> bool { self.is_finite() }
	fn is_nan(self) -> bool { self.is_nan() }
	fn length(self) -> f64 { self.length() }
	fn length_recip(self) -> f64 { self.length_recip() }
	fn distance(self, rhs: Self) -> f64 { self.distance(rhs) }
	fn normalize(self) -> Self { self.normalize() }
	fn try_normalize(self) -> Option<Self> { self.try_normalize() }
	fn normalize_or_zero(self) -> Self { self.normalize_or_zero() }
	fn is_normalized(self) -> bool { self.is_normalized() }
	fn project_onto(self, rhs: Self) -> Self { self.project_onto(rhs) }
	fn reject_from(self, rhs: Self) -> Self { self.reject_from(rhs) }
	fn project_onto_normalized(self, rhs: Self) -> Self { self.project_onto_normalized(rhs) }
	fn reject_from_normalized(self, rhs: Self) -> Self { self.reject_from_normalized(rhs) }
	fn round(self) -> Self { self.round() }
	fn floor(self) -> Self { self.floor() }
	fn ceil(self) -> Self { self.ceil() }
	fn trunc(self) -> Self { self.trunc() }
	fn fract(self) -> Self { self.fract() }
	fn exp(self) -> Self { self.exp() }
	fn powf(self, n: f64) -> Self { self.powf(n) }
	fn recip(self) -> Self { self.recip() }
	fn lerp(self, rhs: Self, s: f64) -> Self { self.lerp(rhs, s) }
	fn midpoint(self, rhs: Self) -> Self { self.midpoint(rhs) }
	fn abs_diff_eq(self, rhs: Self, max_abs_diff: f64) -> bool { self.abs_diff_eq(rhs, max_abs_diff) }
	fn clamp_length(self, min: f64, max: f64) -> Self { self.clamp_length(min, max) }
	fn clamp_length_max(self, max: f64) -> Self { self.clamp_length_max(max) }
	fn clamp_length_min(self, min: f64) -> Self { self.clamp_length_min(min) }
	fn mul_add(self, a: Self, b: Self) -> Self { self.mul_add(a, b) }
}
