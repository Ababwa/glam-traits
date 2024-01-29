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
use num_traits::{
	NumAssign,
	FromPrimitive,
	ToPrimitive,
	PrimInt,
	Signed,
};
use crate::SIntVec3;

pub trait SIntVec2
where
	for <'a> Self:
		Clone +
		Copy +
		PartialEq +
		Default +
		Div<Output = Self> +
		DivAssign +
		Div<Self::Scalar, Output = Self> +
		DivAssign<Self::Scalar> +
		Mul<Output = Self> +
		MulAssign +
		Mul<Self::Scalar, Output = Self> +
		MulAssign<Self::Scalar> +
		Add<Output = Self> +
		AddAssign +
		Add<Self::Scalar, Output = Self> +
		AddAssign<Self::Scalar> +
		Sub<Output = Self> +
		SubAssign +
		Sub<Self::Scalar, Output = Self> +
		SubAssign<Self::Scalar> +
		Rem<Output = Self> +
		RemAssign +
		Rem<Self::Scalar, Output = Self> +
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
		Eq +
		Not +
		BitAnd +
		BitAnd<Self::Scalar> +
		BitOr +
		BitOr<Self::Scalar> +
		BitXor +
		BitXor<Self::Scalar> +
		Shl<i8> + Shr<i8> + Shl<u8> + Shr<u8> +
		Shl<i16> + Shr<i16> + Shl<u16> + Shr<u16> +
		Shl<i32> + Shr<i32> + Shl<u32> + Shr<u32> +
		Shl<i64> + Shr<i64> + Shl<u64> + Shr<u64> +
		AsRef<[Self::Scalar; 2]> +
		AsMut<[Self::Scalar; 2]> +
		From<[Self::Scalar; 2]> +
		Into<[Self::Scalar; 2]> +
		From<(Self::Scalar, Self::Scalar)> +
		Into<(Self::Scalar, Self::Scalar)> +
	,
	Self::Scalar:
		Div<Self, Output = Self> +
		Mul<Self, Output = Self> +
		Add<Self, Output = Self> +
		Sub<Self, Output = Self> +
		Rem<Self, Output = Self> +
		Copy +
		PartialOrd +
		NumAssign +
		FromPrimitive +
		ToPrimitive +
		PrimInt +
		Signed +
	,
	Self::SIntVec3Type: SIntVec3<Scalar = Self::Scalar>,
{
	type Scalar;
	type SIntVec3Type;
	const ZERO: Self;
	const ONE: Self;
	const NEG_ONE: Self;
	const MIN: Self;
	const MAX: Self;
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
	fn wrapping_add(self, rhs: Self) -> Self;
	fn wrapping_sub(self, rhs: Self) -> Self;
	fn wrapping_mul(self, rhs: Self) -> Self;
	fn wrapping_div(self, rhs: Self) -> Self;
	fn saturating_add(self, rhs: Self) -> Self;
	fn saturating_sub(self, rhs: Self) -> Self;
	fn saturating_mul(self, rhs: Self) -> Self;
	fn saturating_div(self, rhs: Self) -> Self;
	fn new(x: Self::Scalar, y: Self::Scalar) -> Self;
	fn from_array(a: [Self::Scalar; 2]) -> Self;
	fn to_array(&self) -> [Self::Scalar; 2];
	fn extend(self, v: Self::Scalar) -> Self::SIntVec3Type;
	fn perp(self) -> Self;
	fn perp_dot(self, rhs: Self) -> Self::Scalar;
	fn rotate(self, rhs: Self) -> Self;
}

impl SIntVec2 for glam::I16Vec2 {
	type Scalar = i16;
	type SIntVec3Type = glam::I16Vec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const NEG_ONE: Self = Self::NEG_ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
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
	fn wrapping_add(self, rhs: Self) -> Self { self.wrapping_add(rhs) }
	fn wrapping_sub(self, rhs: Self) -> Self { self.wrapping_sub(rhs) }
	fn wrapping_mul(self, rhs: Self) -> Self { self.wrapping_mul(rhs) }
	fn wrapping_div(self, rhs: Self) -> Self { self.wrapping_div(rhs) }
	fn saturating_add(self, rhs: Self) -> Self { self.saturating_add(rhs) }
	fn saturating_sub(self, rhs: Self) -> Self { self.saturating_sub(rhs) }
	fn saturating_mul(self, rhs: Self) -> Self { self.saturating_mul(rhs) }
	fn saturating_div(self, rhs: Self) -> Self { self.saturating_div(rhs) }
	fn new(x: Self::Scalar, y: Self::Scalar) -> Self { Self::new(x, y) }
	fn from_array(a: [Self::Scalar; 2]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 2] { self.to_array() }
	fn extend(self, v: Self::Scalar) -> Self::SIntVec3Type { self.extend(v) }
	fn perp(self) -> Self { self.perp() }
	fn perp_dot(self, rhs: Self) -> Self::Scalar { self.perp_dot(rhs) }
	fn rotate(self, rhs: Self) -> Self { self.rotate(rhs) }
}

impl SIntVec2 for glam::IVec2 {
	type Scalar = i32;
	type SIntVec3Type = glam::IVec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const NEG_ONE: Self = Self::NEG_ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
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
	fn wrapping_add(self, rhs: Self) -> Self { self.wrapping_add(rhs) }
	fn wrapping_sub(self, rhs: Self) -> Self { self.wrapping_sub(rhs) }
	fn wrapping_mul(self, rhs: Self) -> Self { self.wrapping_mul(rhs) }
	fn wrapping_div(self, rhs: Self) -> Self { self.wrapping_div(rhs) }
	fn saturating_add(self, rhs: Self) -> Self { self.saturating_add(rhs) }
	fn saturating_sub(self, rhs: Self) -> Self { self.saturating_sub(rhs) }
	fn saturating_mul(self, rhs: Self) -> Self { self.saturating_mul(rhs) }
	fn saturating_div(self, rhs: Self) -> Self { self.saturating_div(rhs) }
	fn new(x: Self::Scalar, y: Self::Scalar) -> Self { Self::new(x, y) }
	fn from_array(a: [Self::Scalar; 2]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 2] { self.to_array() }
	fn extend(self, v: Self::Scalar) -> Self::SIntVec3Type { self.extend(v) }
	fn perp(self) -> Self { self.perp() }
	fn perp_dot(self, rhs: Self) -> Self::Scalar { self.perp_dot(rhs) }
	fn rotate(self, rhs: Self) -> Self { self.rotate(rhs) }
}

impl SIntVec2 for glam::I64Vec2 {
	type Scalar = i64;
	type SIntVec3Type = glam::I64Vec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const NEG_ONE: Self = Self::NEG_ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
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
	fn wrapping_add(self, rhs: Self) -> Self { self.wrapping_add(rhs) }
	fn wrapping_sub(self, rhs: Self) -> Self { self.wrapping_sub(rhs) }
	fn wrapping_mul(self, rhs: Self) -> Self { self.wrapping_mul(rhs) }
	fn wrapping_div(self, rhs: Self) -> Self { self.wrapping_div(rhs) }
	fn saturating_add(self, rhs: Self) -> Self { self.saturating_add(rhs) }
	fn saturating_sub(self, rhs: Self) -> Self { self.saturating_sub(rhs) }
	fn saturating_mul(self, rhs: Self) -> Self { self.saturating_mul(rhs) }
	fn saturating_div(self, rhs: Self) -> Self { self.saturating_div(rhs) }
	fn new(x: Self::Scalar, y: Self::Scalar) -> Self { Self::new(x, y) }
	fn from_array(a: [Self::Scalar; 2]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 2] { self.to_array() }
	fn extend(self, v: Self::Scalar) -> Self::SIntVec3Type { self.extend(v) }
	fn perp(self) -> Self { self.perp() }
	fn perp_dot(self, rhs: Self) -> Self::Scalar { self.perp_dot(rhs) }
	fn rotate(self, rhs: Self) -> Self { self.rotate(rhs) }
}
