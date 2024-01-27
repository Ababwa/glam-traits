use std::{
	fmt::{Debug, Display},
	iter::{Product, Sum},
	ops::{
		Add, AddAssign, BitAnd, BitOr, BitXor, Div, DivAssign, Index, IndexMut, Mul, MulAssign,
		Neg, Not, Rem, RemAssign, Shl, Shr, Sub, SubAssign,
	},
};
use num_traits::{Float, Num, PrimInt, Signed};
use glam::{BVec2, BVec3};
use crate::bvec::GBVec4;

pub trait GVec
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
		Index<usize> +
		IndexMut<usize> +
		Display +
		Debug +
	,
	Self::Scalar:
		Div<Self> +
		Mul<Self> +
		Add<Self> +
		Sub<Self> +
		Rem<Self> +
		Num +
	,
{
	type Scalar;
	const ZERO: Self;
	const ONE: Self;
	const MIN: Self;
	const MAX: Self;
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
	fn length_squared(self) -> Self::Scalar;
}

pub trait SignedVec
where
	Self: GVec + Neg,
	Self::Scalar: Signed,
{
	const NEG_ONE: Self;
	fn abs(self) -> Self;
	fn signum(self) -> Self;
	fn is_negative_bitmask(self) -> u32;
	fn distance_squared(self, rhs: Self) -> Self::Scalar;
	fn div_euclid(self, rhs: Self) -> Self;
}

pub trait FloatVec
where
	Self: SignedVec,
	Self::Scalar: Signed + Float,
{
	const NAN: Self;
	const INFINITY: Self;
	const NEG_INFINITY: Self;
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
}

pub trait IntVec
where
	Self:
		GVec +
		Eq +
		Not +
		BitAnd +
		BitAnd<Self::Scalar> +
		BitOr +
		BitOr<Self::Scalar> +
		BitXor +
		BitXor<Self::Scalar> +
		Shl<i8> + Shr<i8> +
		Shl<i16> + Shr<i16> +
		Shl<i32> + Shr<i32> +
		Shl<i64> + Shr<i64> +
		Shl<u8> + Shr<u8> +
		Shl<u16> + Shr<u16> +
		Shl<u32> + Shr<u32> +
		Shl<u64> + Shr<u64> +
	,
	Self::Scalar: PrimInt,
{
	fn wrapping_add(self, rhs: Self) -> Self;
	fn wrapping_sub(self, rhs: Self) -> Self;
	fn wrapping_mul(self, rhs: Self) -> Self;
	fn wrapping_div(self, rhs: Self) -> Self;
	fn saturating_add(self, rhs: Self) -> Self;
	fn saturating_sub(self, rhs: Self) -> Self;
	fn saturating_mul(self, rhs: Self) -> Self;
	fn saturating_div(self, rhs: Self) -> Self;
}

pub trait GVec2
where
	Self:
		GVec +
		AsRef<[Self::Scalar; 2]> +
		AsMut<[Self::Scalar; 2]> +
		From<[Self::Scalar; 2]> +
		From<(Self::Scalar, Self::Scalar)> +
	,
{
	const X: Self;
	const Y: Self;
	const AXES: [Self; 2];
	fn new(x: Self::Scalar, y: Self::Scalar) -> Self;
	fn from_array(a: [Self::Scalar; 2]) -> Self;
	fn to_array(&self) -> [Self::Scalar; 2];
	fn select(mask: BVec2, if_true: Self, if_false: Self) -> Self;
	fn cmpeq(self, rhs: Self) -> BVec2;
	fn cmpne(self, rhs: Self) -> BVec2;
	fn cmpge(self, rhs: Self) -> BVec2;
	fn cmpgt(self, rhs: Self) -> BVec2;
	fn cmple(self, rhs: Self) -> BVec2;
	fn cmplt(self, rhs: Self) -> BVec2;
}

pub trait SignedVec2
where
	Self: GVec2 + SignedVec,
	Self::Scalar: Signed,
{
	const NEG_X: Self;
	const NEG_Y: Self;
	fn perp(self) -> Self;
	fn perp_dot(self, rhs: Self) -> Self::Scalar;
	fn rotate(self, rhs: Self) -> Self;
}

pub trait FloatVec2
where
	Self: SignedVec2 + FloatVec,
	Self::Scalar: Signed + Float,
{
	fn is_nan_mask(self) -> BVec2;
	fn angle_between(self, rhs: Self) -> Self::Scalar;
	fn from_angle(angle: Self::Scalar) -> Self;
	fn to_angle(self) -> Self::Scalar;
}

pub trait IntVec2
where
	Self: GVec2 + IntVec,
	Self::Scalar: PrimInt,
{}

pub trait GVec3
where
	Self:
		GVec +
		AsRef<[Self::Scalar; 3]> +
		AsMut<[Self::Scalar; 3]> +
		From<[Self::Scalar; 3]> +
		From<(Self::Scalar, Self::Scalar, Self::Scalar)> +
	,
{
	const X: Self;
	const Y: Self;
	const Z: Self;
	const AXES: [Self; 3];
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar) -> Self;
	fn from_array(a: [Self::Scalar; 3]) -> Self;
	fn to_array(&self) -> [Self::Scalar; 3];
	fn select(mask: BVec3, if_true: Self, if_false: Self) -> Self;
	fn cmpeq(self, rhs: Self) -> BVec3;
	fn cmpne(self, rhs: Self) -> BVec3;
	fn cmpge(self, rhs: Self) -> BVec3;
	fn cmpgt(self, rhs: Self) -> BVec3;
	fn cmple(self, rhs: Self) -> BVec3;
	fn cmplt(self, rhs: Self) -> BVec3;
	fn cross(self, rhs: Self) -> Self;
}

pub trait SignedVec3
where
	Self: GVec3 + SignedVec,
	Self::Scalar: Signed,
{
	const NEG_X: Self;
	const NEG_Y: Self;
	const NEG_Z: Self;
}

pub trait FloatVec3
where
	Self: SignedVec3 + FloatVec,
	Self::Scalar: Signed + Float,
{
	fn is_nan_mask(self) -> BVec3;
	fn angle_between(self, rhs: Self) -> Self::Scalar;
	fn any_orthogonal_vector(&self) -> Self;
	fn any_orthonormal_vector(&self) -> Self;
	fn any_orthonormal_pair(&self) -> (Self, Self);
}

pub trait IntVec3
where
	Self: GVec3 + IntVec,
	Self::Scalar: PrimInt,
{}

pub trait GVec4
where
	Self:
		GVec +
		AsRef<[Self::Scalar; 4]> +
		AsMut<[Self::Scalar; 4]> +
		From<[Self::Scalar; 4]> +
		From<(Self::Scalar, Self::Scalar, Self::Scalar, Self::Scalar)> +
	,
	Self::BVec4Type: GBVec4,
{
	type BVec4Type;
	const X: Self;
	const Y: Self;
	const Z: Self;
	const W: Self;
	const AXES: [Self; 4];
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar, w: Self::Scalar) -> Self;
	fn from_array(a: [Self::Scalar; 4]) -> Self;
	fn to_array(&self) -> [Self::Scalar; 4];
	fn select(mask: Self::BVec4Type, if_true: Self, if_false: Self) -> Self;
	fn cmpeq(self, rhs: Self) -> Self::BVec4Type;
	fn cmpne(self, rhs: Self) -> Self::BVec4Type;
	fn cmpge(self, rhs: Self) -> Self::BVec4Type;
	fn cmpgt(self, rhs: Self) -> Self::BVec4Type;
	fn cmple(self, rhs: Self) -> Self::BVec4Type;
	fn cmplt(self, rhs: Self) -> Self::BVec4Type;
}

pub trait SignedVec4
where
	Self: GVec4 + SignedVec,
	Self::Scalar: Signed,
{
	const NEG_X: Self;
	const NEG_Y: Self;
	const NEG_Z: Self;
	const NEG_W: Self;
}

pub trait FloatVec4
where
	Self: SignedVec4 + FloatVec,
	Self::Scalar: Signed + Float,
{
	fn is_nan_mask(self) -> Self::BVec4Type;
}

pub trait IntVec4
where
	Self: GVec4 + IntVec,
	Self::Scalar: PrimInt,
{}
