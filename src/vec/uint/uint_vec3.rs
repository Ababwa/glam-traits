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
use num_traits::{
	NumAssign,
	FromPrimitive,
	ToPrimitive,
	PrimInt,
	Unsigned,
};
use crate::GBVec3;
use crate::UIntVec2;
use crate::UIntVec4;

pub trait UIntVec3
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
		AsRef<[Self::Scalar; 3]> +
		AsMut<[Self::Scalar; 3]> +
		From<[Self::Scalar; 3]> +
		Into<[Self::Scalar; 3]> +
		From<(Self::Scalar, Self::Scalar, Self::Scalar)> +
		Into<(Self::Scalar, Self::Scalar, Self::Scalar)> +
		From<(Self::UIntVec2Type, Self::Scalar)> +
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
		PrimInt +
		Unsigned +
	,
	Self::BVec3Type: GBVec3,
	Self::UIntVec2Type: UIntVec2<Scalar = Self::Scalar>,
	Self::UIntVec4Type: UIntVec4<Scalar = Self::Scalar>,
{
	type Scalar;
	type BVec3Type;
	type UIntVec2Type;
	type UIntVec4Type;
	const ZERO: Self;
	const ONE: Self;
	const MIN: Self;
	const MAX: Self;
	const X: Self;
	const Y: Self;
	const Z: Self;
	const AXES: [Self; 3];
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
	fn select(mask: Self::BVec3Type, if_true: Self, if_false: Self) -> Self;
	fn cmpeq(self, rhs: Self) -> Self::BVec3Type;
	fn cmpne(self, rhs: Self) -> Self::BVec3Type;
	fn cmpge(self, rhs: Self) -> Self::BVec3Type;
	fn cmpgt(self, rhs: Self) -> Self::BVec3Type;
	fn cmple(self, rhs: Self) -> Self::BVec3Type;
	fn cmplt(self, rhs: Self) -> Self::BVec3Type;
	fn length_squared(self) -> Self::Scalar;
	fn wrapping_add(self, rhs: Self) -> Self;
	fn wrapping_sub(self, rhs: Self) -> Self;
	fn wrapping_mul(self, rhs: Self) -> Self;
	fn wrapping_div(self, rhs: Self) -> Self;
	fn saturating_add(self, rhs: Self) -> Self;
	fn saturating_sub(self, rhs: Self) -> Self;
	fn saturating_mul(self, rhs: Self) -> Self;
	fn saturating_div(self, rhs: Self) -> Self;
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar) -> Self;
	fn from_array(a: [Self::Scalar; 3]) -> Self;
	fn to_array(&self) -> [Self::Scalar; 3];
	fn extend(self, v: Self::Scalar) -> Self::UIntVec4Type;
	fn truncate(self) -> Self::UIntVec2Type;
	fn cross(self, rhs: Self) -> Self;
}

impl UIntVec3 for glam::U16Vec3 {
	type Scalar = u16;
	type BVec3Type = glam::BVec3;
	type UIntVec2Type = glam::U16Vec2;
	type UIntVec4Type = glam::U16Vec4;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
	const Z: Self = Self::Z;
	const AXES: [Self; 3] = Self::AXES;
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
	fn select(mask: Self::BVec3Type, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVec3Type { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVec3Type { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVec3Type { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVec3Type { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVec3Type { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVec3Type { self.cmplt(rhs) }
	fn length_squared(self) -> Self::Scalar { self.length_squared() }
	fn wrapping_add(self, rhs: Self) -> Self { self.wrapping_add(rhs) }
	fn wrapping_sub(self, rhs: Self) -> Self { self.wrapping_sub(rhs) }
	fn wrapping_mul(self, rhs: Self) -> Self { self.wrapping_mul(rhs) }
	fn wrapping_div(self, rhs: Self) -> Self { self.wrapping_div(rhs) }
	fn saturating_add(self, rhs: Self) -> Self { self.saturating_add(rhs) }
	fn saturating_sub(self, rhs: Self) -> Self { self.saturating_sub(rhs) }
	fn saturating_mul(self, rhs: Self) -> Self { self.saturating_mul(rhs) }
	fn saturating_div(self, rhs: Self) -> Self { self.saturating_div(rhs) }
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar) -> Self { Self::new(x, y, z) }
	fn from_array(a: [Self::Scalar; 3]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 3] { self.to_array() }
	fn extend(self, v: Self::Scalar) -> Self::UIntVec4Type { self.extend(v) }
	fn truncate(self) -> Self::UIntVec2Type { self.truncate() }
	fn cross(self, rhs: Self) -> Self { self.cross(rhs) }
}

impl UIntVec3 for glam::UVec3 {
	type Scalar = u32;
	type BVec3Type = glam::BVec3;
	type UIntVec2Type = glam::UVec2;
	type UIntVec4Type = glam::UVec4;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
	const Z: Self = Self::Z;
	const AXES: [Self; 3] = Self::AXES;
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
	fn select(mask: Self::BVec3Type, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVec3Type { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVec3Type { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVec3Type { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVec3Type { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVec3Type { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVec3Type { self.cmplt(rhs) }
	fn length_squared(self) -> Self::Scalar { self.length_squared() }
	fn wrapping_add(self, rhs: Self) -> Self { self.wrapping_add(rhs) }
	fn wrapping_sub(self, rhs: Self) -> Self { self.wrapping_sub(rhs) }
	fn wrapping_mul(self, rhs: Self) -> Self { self.wrapping_mul(rhs) }
	fn wrapping_div(self, rhs: Self) -> Self { self.wrapping_div(rhs) }
	fn saturating_add(self, rhs: Self) -> Self { self.saturating_add(rhs) }
	fn saturating_sub(self, rhs: Self) -> Self { self.saturating_sub(rhs) }
	fn saturating_mul(self, rhs: Self) -> Self { self.saturating_mul(rhs) }
	fn saturating_div(self, rhs: Self) -> Self { self.saturating_div(rhs) }
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar) -> Self { Self::new(x, y, z) }
	fn from_array(a: [Self::Scalar; 3]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 3] { self.to_array() }
	fn extend(self, v: Self::Scalar) -> Self::UIntVec4Type { self.extend(v) }
	fn truncate(self) -> Self::UIntVec2Type { self.truncate() }
	fn cross(self, rhs: Self) -> Self { self.cross(rhs) }
}

impl UIntVec3 for glam::U64Vec3 {
	type Scalar = u64;
	type BVec3Type = glam::BVec3;
	type UIntVec2Type = glam::U64Vec2;
	type UIntVec4Type = glam::U64Vec4;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
	const Z: Self = Self::Z;
	const AXES: [Self; 3] = Self::AXES;
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
	fn select(mask: Self::BVec3Type, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVec3Type { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVec3Type { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVec3Type { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVec3Type { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVec3Type { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVec3Type { self.cmplt(rhs) }
	fn length_squared(self) -> Self::Scalar { self.length_squared() }
	fn wrapping_add(self, rhs: Self) -> Self { self.wrapping_add(rhs) }
	fn wrapping_sub(self, rhs: Self) -> Self { self.wrapping_sub(rhs) }
	fn wrapping_mul(self, rhs: Self) -> Self { self.wrapping_mul(rhs) }
	fn wrapping_div(self, rhs: Self) -> Self { self.wrapping_div(rhs) }
	fn saturating_add(self, rhs: Self) -> Self { self.saturating_add(rhs) }
	fn saturating_sub(self, rhs: Self) -> Self { self.saturating_sub(rhs) }
	fn saturating_mul(self, rhs: Self) -> Self { self.saturating_mul(rhs) }
	fn saturating_div(self, rhs: Self) -> Self { self.saturating_div(rhs) }
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar) -> Self { Self::new(x, y, z) }
	fn from_array(a: [Self::Scalar; 3]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 3] { self.to_array() }
	fn extend(self, v: Self::Scalar) -> Self::UIntVec4Type { self.extend(v) }
	fn truncate(self) -> Self::UIntVec2Type { self.truncate() }
	fn cross(self, rhs: Self) -> Self { self.cross(rhs) }
}
