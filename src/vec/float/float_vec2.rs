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
use num_traits::{
	NumAssign,
	FromPrimitive,
	ToPrimitive,
	Float,
	Signed,
};
use crate::FloatVec3;

pub trait FloatVec2
where
	for <'a> Self:
		Clone +
		Copy +
		PartialEq +
		Default +
		Div +
		DivAssign +
		Div<Self::Scalar> +
		DivAssign<Self::Scalar> +
		Mul +
		MulAssign +
		Mul<Self::Scalar> +
		MulAssign<Self::Scalar> +
		Add +
		AddAssign +
		Add<Self::Scalar> +
		AddAssign<Self::Scalar> +
		Sub +
		SubAssign +
		Sub<Self::Scalar> +
		SubAssign<Self::Scalar> +
		Rem +
		RemAssign +
		Rem<Self::Scalar> +
		RemAssign<Self::Scalar> +
		Sum +
		Sum<&'a Self> +
		Product +
		Product<&'a Self> +
		Index<usize, Output = Self::Scalar> +
		IndexMut<usize, Output = Self::Scalar> +
		Display +
		Debug +
		Neg +
		AsRef<[Self::Scalar; 2]> +
		AsMut<[Self::Scalar; 2]> +
		From<[Self::Scalar; 2]> +
		Into<[Self::Scalar; 2]> +
		From<(Self::Scalar, Self::Scalar)> +
		Into<(Self::Scalar, Self::Scalar)> +
	,
	Self::Scalar:
		Div<Self> +
		Mul<Self> +
		Add<Self> +
		Sub<Self> +
		Rem<Self> +
		Copy +
		PartialOrd +
		NumAssign +
		FromPrimitive +
		ToPrimitive +
		Float +
		Signed +
	,
	Self::FloatVec3Type: FloatVec3<Scalar = Self::Scalar>,
{
	type Scalar;
	type FloatVec3Type;
	const ZERO: Self;
	const ONE: Self;
	const NEG_ONE: Self;
	const MIN: Self;
	const MAX: Self;
	const NAN: Self;
	const INFINITY: Self;
	const NEG_INFINITY: Self;
	const X: Self;
	const Y: Self;
	const NEG_X: Self;
	const NEG_Y: Self;
	const AXES: [Self; 2];
	fn splat(v: Self::Scalar) -> Self;
	fn from_slice(slice: &[Self::Scalar]) -> Self;
	fn write_to_slice(self, slice: &mut [Self::Scalar]);
	fn dot(self, rhs: Self) -> Self::Scalar;
	fn dot_into_vec(self, rhs: Self) -> Self;
	fn min(self, rhs: Self) -> Self;
	fn max(self, rhs: Self) -> Self;
	fn clamp(self, min: Self, max: Self) -> Self;
	fn min_element(self) -> Self::Scalar;
	fn max_element(self) -> Self::Scalar;
	fn element_sum(self) -> Self::Scalar;
	fn element_product(self) -> Self::Scalar;
	fn select(mask: glam::BVec2, if_true: Self, if_false: Self) -> Self;
	fn cmpeq(self, rhs: Self) -> glam::BVec2;
	fn cmpne(self, rhs: Self) -> glam::BVec2;
	fn cmpge(self, rhs: Self) -> glam::BVec2;
	fn cmpgt(self, rhs: Self) -> glam::BVec2;
	fn cmple(self, rhs: Self) -> glam::BVec2;
	fn cmplt(self, rhs: Self) -> glam::BVec2;
	fn length_squared(self) -> Self::Scalar;
	fn abs(self) -> Self;
	fn signum(self) -> Self;
	fn is_negative_bitmask(self) -> u32;
	fn distance_squared(self, rhs: Self) -> Self::Scalar;
	fn div_euclid(self, rhs: Self) -> Self;
	fn rem_euclid(self, rhs: Self) -> Self;
	fn copysign(self, rhs: Self) -> Self;
	fn is_finite(self) -> bool;
	fn is_nan(self) -> bool;
	fn length(self) -> Self::Scalar;
	fn length_recip(self) -> Self::Scalar;
	fn distance(self, rhs: Self) -> Self::Scalar;
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
	fn powf(self, n: Self::Scalar) -> Self;
	fn recip(self) -> Self;
	fn lerp(self, rhs: Self, s: Self::Scalar) -> Self;
	fn midpoint(self, rhs: Self) -> Self;
	fn abs_diff_eq(self, rhs: Self, max_abs_diff: Self::Scalar) -> bool;
	fn clamp_length(self, min: Self::Scalar, max: Self::Scalar) -> Self;
	fn clamp_length_max(self, max: Self::Scalar) -> Self;
	fn clamp_length_min(self, min: Self::Scalar) -> Self;
	fn mul_add(self, a: Self, b: Self) -> Self;
	fn new(x: Self::Scalar, y: Self::Scalar) -> Self;
	fn from_array(a: [Self::Scalar; 2]) -> Self;
	fn to_array(&self) -> [Self::Scalar; 2];
	fn extend(self, v: Self::Scalar) -> Self::FloatVec3Type;
	fn perp(self) -> Self;
	fn perp_dot(self, rhs: Self) -> Self::Scalar;
	fn rotate(self, rhs: Self) -> Self;
	fn is_nan_mask(self) -> glam::BVec2;
	fn angle_between(self, rhs: Self) -> Self::Scalar;
	fn from_angle(angle: Self::Scalar) -> Self;
	fn to_angle(self) -> Self::Scalar;
}

impl FloatVec2 for glam::Vec2 {
	type Scalar = f32;
	type FloatVec3Type = glam::Vec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const NEG_ONE: Self = Self::NEG_ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const NAN: Self = Self::NAN;
	const INFINITY: Self = Self::INFINITY;
	const NEG_INFINITY: Self = Self::NEG_INFINITY;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
	const NEG_X: Self = Self::NEG_X;
	const NEG_Y: Self = Self::NEG_Y;
	const AXES: [Self; 2] = Self::AXES;
	fn splat(v: Self::Scalar) -> Self { Self::splat(v) }
	fn from_slice(slice: &[Self::Scalar]) -> Self { Self::from_slice(slice) }
	fn write_to_slice(self, slice: &mut [Self::Scalar]) { self.write_to_slice(slice) }
	fn dot(self, rhs: Self) -> Self::Scalar { self.dot(rhs) }
	fn dot_into_vec(self, rhs: Self) -> Self { self.dot_into_vec(rhs) }
	fn min(self, rhs: Self) -> Self { self.min(rhs) }
	fn max(self, rhs: Self) -> Self { self.max(rhs) }
	fn clamp(self, min: Self, max: Self) -> Self { self.clamp(min, max) }
	fn min_element(self) -> Self::Scalar { self.min_element() }
	fn max_element(self) -> Self::Scalar { self.max_element() }
	fn element_sum(self) -> Self::Scalar { self.element_sum() }
	fn element_product(self) -> Self::Scalar { self.element_product() }
	fn select(mask: glam::BVec2, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> glam::BVec2 { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> glam::BVec2 { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> glam::BVec2 { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> glam::BVec2 { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> glam::BVec2 { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> glam::BVec2 { self.cmplt(rhs) }
	fn length_squared(self) -> Self::Scalar { self.length_squared() }
	fn abs(self) -> Self { self.abs() }
	fn signum(self) -> Self { self.signum() }
	fn is_negative_bitmask(self) -> u32 { self.is_negative_bitmask() }
	fn distance_squared(self, rhs: Self) -> Self::Scalar { self.distance_squared(rhs) }
	fn div_euclid(self, rhs: Self) -> Self { self.div_euclid(rhs) }
	fn rem_euclid(self, rhs: Self) -> Self { self.rem_euclid(rhs) }
	fn copysign(self, rhs: Self) -> Self { self.copysign(rhs) }
	fn is_finite(self) -> bool { self.is_finite() }
	fn is_nan(self) -> bool { self.is_nan() }
	fn length(self) -> Self::Scalar { self.length() }
	fn length_recip(self) -> Self::Scalar { self.length_recip() }
	fn distance(self, rhs: Self) -> Self::Scalar { self.distance(rhs) }
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
	fn powf(self, n: Self::Scalar) -> Self { self.powf(n) }
	fn recip(self) -> Self { self.recip() }
	fn lerp(self, rhs: Self, s: Self::Scalar) -> Self { self.lerp(rhs, s) }
	fn midpoint(self, rhs: Self) -> Self { self.midpoint(rhs) }
	fn abs_diff_eq(self, rhs: Self, max_abs_diff: Self::Scalar) -> bool { self.abs_diff_eq(rhs, max_abs_diff) }
	fn clamp_length(self, min: Self::Scalar, max: Self::Scalar) -> Self { self.clamp_length(min, max) }
	fn clamp_length_max(self, max: Self::Scalar) -> Self { self.clamp_length_max(max) }
	fn clamp_length_min(self, min: Self::Scalar) -> Self { self.clamp_length_min(min) }
	fn mul_add(self, a: Self, b: Self) -> Self { self.mul_add(a, b) }
	fn new(x: Self::Scalar, y: Self::Scalar) -> Self { Self::new(x, y) }
	fn from_array(a: [Self::Scalar; 2]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 2] { self.to_array() }
	fn extend(self, v: Self::Scalar) -> Self::FloatVec3Type { self.extend(v) }
	fn perp(self) -> Self { self.perp() }
	fn perp_dot(self, rhs: Self) -> Self::Scalar { self.perp_dot(rhs) }
	fn rotate(self, rhs: Self) -> Self { self.rotate(rhs) }
	fn is_nan_mask(self) -> glam::BVec2 { self.is_nan_mask() }
	fn angle_between(self, rhs: Self) -> Self::Scalar { self.angle_between(rhs) }
	fn from_angle(angle: Self::Scalar) -> Self { Self::from_angle(angle) }
	fn to_angle(self) -> Self::Scalar { self.to_angle() }
}

impl FloatVec2 for glam::DVec2 {
	type Scalar = f64;
	type FloatVec3Type = glam::DVec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const NEG_ONE: Self = Self::NEG_ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const NAN: Self = Self::NAN;
	const INFINITY: Self = Self::INFINITY;
	const NEG_INFINITY: Self = Self::NEG_INFINITY;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
	const NEG_X: Self = Self::NEG_X;
	const NEG_Y: Self = Self::NEG_Y;
	const AXES: [Self; 2] = Self::AXES;
	fn splat(v: Self::Scalar) -> Self { Self::splat(v) }
	fn from_slice(slice: &[Self::Scalar]) -> Self { Self::from_slice(slice) }
	fn write_to_slice(self, slice: &mut [Self::Scalar]) { self.write_to_slice(slice) }
	fn dot(self, rhs: Self) -> Self::Scalar { self.dot(rhs) }
	fn dot_into_vec(self, rhs: Self) -> Self { self.dot_into_vec(rhs) }
	fn min(self, rhs: Self) -> Self { self.min(rhs) }
	fn max(self, rhs: Self) -> Self { self.max(rhs) }
	fn clamp(self, min: Self, max: Self) -> Self { self.clamp(min, max) }
	fn min_element(self) -> Self::Scalar { self.min_element() }
	fn max_element(self) -> Self::Scalar { self.max_element() }
	fn element_sum(self) -> Self::Scalar { self.element_sum() }
	fn element_product(self) -> Self::Scalar { self.element_product() }
	fn select(mask: glam::BVec2, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> glam::BVec2 { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> glam::BVec2 { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> glam::BVec2 { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> glam::BVec2 { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> glam::BVec2 { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> glam::BVec2 { self.cmplt(rhs) }
	fn length_squared(self) -> Self::Scalar { self.length_squared() }
	fn abs(self) -> Self { self.abs() }
	fn signum(self) -> Self { self.signum() }
	fn is_negative_bitmask(self) -> u32 { self.is_negative_bitmask() }
	fn distance_squared(self, rhs: Self) -> Self::Scalar { self.distance_squared(rhs) }
	fn div_euclid(self, rhs: Self) -> Self { self.div_euclid(rhs) }
	fn rem_euclid(self, rhs: Self) -> Self { self.rem_euclid(rhs) }
	fn copysign(self, rhs: Self) -> Self { self.copysign(rhs) }
	fn is_finite(self) -> bool { self.is_finite() }
	fn is_nan(self) -> bool { self.is_nan() }
	fn length(self) -> Self::Scalar { self.length() }
	fn length_recip(self) -> Self::Scalar { self.length_recip() }
	fn distance(self, rhs: Self) -> Self::Scalar { self.distance(rhs) }
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
	fn powf(self, n: Self::Scalar) -> Self { self.powf(n) }
	fn recip(self) -> Self { self.recip() }
	fn lerp(self, rhs: Self, s: Self::Scalar) -> Self { self.lerp(rhs, s) }
	fn midpoint(self, rhs: Self) -> Self { self.midpoint(rhs) }
	fn abs_diff_eq(self, rhs: Self, max_abs_diff: Self::Scalar) -> bool { self.abs_diff_eq(rhs, max_abs_diff) }
	fn clamp_length(self, min: Self::Scalar, max: Self::Scalar) -> Self { self.clamp_length(min, max) }
	fn clamp_length_max(self, max: Self::Scalar) -> Self { self.clamp_length_max(max) }
	fn clamp_length_min(self, min: Self::Scalar) -> Self { self.clamp_length_min(min) }
	fn mul_add(self, a: Self, b: Self) -> Self { self.mul_add(a, b) }
	fn new(x: Self::Scalar, y: Self::Scalar) -> Self { Self::new(x, y) }
	fn from_array(a: [Self::Scalar; 2]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 2] { self.to_array() }
	fn extend(self, v: Self::Scalar) -> Self::FloatVec3Type { self.extend(v) }
	fn perp(self) -> Self { self.perp() }
	fn perp_dot(self, rhs: Self) -> Self::Scalar { self.perp_dot(rhs) }
	fn rotate(self, rhs: Self) -> Self { self.rotate(rhs) }
	fn is_nan_mask(self) -> glam::BVec2 { self.is_nan_mask() }
	fn angle_between(self, rhs: Self) -> Self::Scalar { self.angle_between(rhs) }
	fn from_angle(angle: Self::Scalar) -> Self { Self::from_angle(angle) }
	fn to_angle(self) -> Self::Scalar { self.to_angle() }
}