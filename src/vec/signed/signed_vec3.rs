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
	Signed,
};
use crate::GBVec3;
use crate::SignedVec2;
use crate::SignedVec4;

pub trait SignedVec3
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
		AsRef<[Self::Scalar; 3]> +
		AsMut<[Self::Scalar; 3]> +
		From<[Self::Scalar; 3]> +
		Into<[Self::Scalar; 3]> +
		From<(Self::Scalar, Self::Scalar, Self::Scalar)> +
		Into<(Self::Scalar, Self::Scalar, Self::Scalar)> +
		From<(Self::SignedVec2Type, Self::Scalar)> +
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
		Signed +
	,
	Self::BVec3Type: GBVec3,
	Self::SignedVec2Type: SignedVec2<Scalar = Self::Scalar>,
	Self::SignedVec4Type: SignedVec4<Scalar = Self::Scalar>,
{
	type Scalar;
	type BVec3Type;
	type SignedVec2Type;
	type SignedVec4Type;
	const ZERO: Self;
	const ONE: Self;
	const NEG_ONE: Self;
	const MIN: Self;
	const MAX: Self;
	const X: Self;
	const Y: Self;
	const Z: Self;
	const NEG_X: Self;
	const NEG_Y: Self;
	const NEG_Z: Self;
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
	fn abs(self) -> Self;
	fn signum(self) -> Self;
	fn is_negative_bitmask(self) -> u32;
	fn distance_squared(self, rhs: Self) -> Self::Scalar;
	fn div_euclid(self, rhs: Self) -> Self;
	fn rem_euclid(self, rhs: Self) -> Self;
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar) -> Self;
	fn from_array(a: [Self::Scalar; 3]) -> Self;
	fn to_array(&self) -> [Self::Scalar; 3];
	fn extend(self, v: Self::Scalar) -> Self::SignedVec4Type;
	fn truncate(self) -> Self::SignedVec2Type;
	fn cross(self, rhs: Self) -> Self;
}

impl SignedVec3 for glam::I16Vec3 {
	type Scalar = i16;
	type BVec3Type = glam::BVec3;
	type SignedVec2Type = glam::I16Vec2;
	type SignedVec4Type = glam::I16Vec4;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const NEG_ONE: Self = Self::NEG_ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
	const Z: Self = Self::Z;
	const NEG_X: Self = Self::NEG_X;
	const NEG_Y: Self = Self::NEG_Y;
	const NEG_Z: Self = Self::NEG_Z;
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
	fn abs(self) -> Self { self.abs() }
	fn signum(self) -> Self { self.signum() }
	fn is_negative_bitmask(self) -> u32 { self.is_negative_bitmask() }
	fn distance_squared(self, rhs: Self) -> Self::Scalar { self.distance_squared(rhs) }
	fn div_euclid(self, rhs: Self) -> Self { self.div_euclid(rhs) }
	fn rem_euclid(self, rhs: Self) -> Self { self.rem_euclid(rhs) }
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar) -> Self { Self::new(x, y, z) }
	fn from_array(a: [Self::Scalar; 3]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 3] { self.to_array() }
	fn extend(self, v: Self::Scalar) -> Self::SignedVec4Type { self.extend(v) }
	fn truncate(self) -> Self::SignedVec2Type { self.truncate() }
	fn cross(self, rhs: Self) -> Self { self.cross(rhs) }
}

impl SignedVec3 for glam::IVec3 {
	type Scalar = i32;
	type BVec3Type = glam::BVec3;
	type SignedVec2Type = glam::IVec2;
	type SignedVec4Type = glam::IVec4;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const NEG_ONE: Self = Self::NEG_ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
	const Z: Self = Self::Z;
	const NEG_X: Self = Self::NEG_X;
	const NEG_Y: Self = Self::NEG_Y;
	const NEG_Z: Self = Self::NEG_Z;
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
	fn abs(self) -> Self { self.abs() }
	fn signum(self) -> Self { self.signum() }
	fn is_negative_bitmask(self) -> u32 { self.is_negative_bitmask() }
	fn distance_squared(self, rhs: Self) -> Self::Scalar { self.distance_squared(rhs) }
	fn div_euclid(self, rhs: Self) -> Self { self.div_euclid(rhs) }
	fn rem_euclid(self, rhs: Self) -> Self { self.rem_euclid(rhs) }
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar) -> Self { Self::new(x, y, z) }
	fn from_array(a: [Self::Scalar; 3]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 3] { self.to_array() }
	fn extend(self, v: Self::Scalar) -> Self::SignedVec4Type { self.extend(v) }
	fn truncate(self) -> Self::SignedVec2Type { self.truncate() }
	fn cross(self, rhs: Self) -> Self { self.cross(rhs) }
}

impl SignedVec3 for glam::I64Vec3 {
	type Scalar = i64;
	type BVec3Type = glam::BVec3;
	type SignedVec2Type = glam::I64Vec2;
	type SignedVec4Type = glam::I64Vec4;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const NEG_ONE: Self = Self::NEG_ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
	const Z: Self = Self::Z;
	const NEG_X: Self = Self::NEG_X;
	const NEG_Y: Self = Self::NEG_Y;
	const NEG_Z: Self = Self::NEG_Z;
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
	fn abs(self) -> Self { self.abs() }
	fn signum(self) -> Self { self.signum() }
	fn is_negative_bitmask(self) -> u32 { self.is_negative_bitmask() }
	fn distance_squared(self, rhs: Self) -> Self::Scalar { self.distance_squared(rhs) }
	fn div_euclid(self, rhs: Self) -> Self { self.div_euclid(rhs) }
	fn rem_euclid(self, rhs: Self) -> Self { self.rem_euclid(rhs) }
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar) -> Self { Self::new(x, y, z) }
	fn from_array(a: [Self::Scalar; 3]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 3] { self.to_array() }
	fn extend(self, v: Self::Scalar) -> Self::SignedVec4Type { self.extend(v) }
	fn truncate(self) -> Self::SignedVec2Type { self.truncate() }
	fn cross(self, rhs: Self) -> Self { self.cross(rhs) }
}

impl SignedVec3 for glam::Vec3 {
	type Scalar = f32;
	type BVec3Type = glam::BVec3;
	type SignedVec2Type = glam::Vec2;
	type SignedVec4Type = glam::Vec4;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const NEG_ONE: Self = Self::NEG_ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
	const Z: Self = Self::Z;
	const NEG_X: Self = Self::NEG_X;
	const NEG_Y: Self = Self::NEG_Y;
	const NEG_Z: Self = Self::NEG_Z;
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
	fn abs(self) -> Self { self.abs() }
	fn signum(self) -> Self { self.signum() }
	fn is_negative_bitmask(self) -> u32 { self.is_negative_bitmask() }
	fn distance_squared(self, rhs: Self) -> Self::Scalar { self.distance_squared(rhs) }
	fn div_euclid(self, rhs: Self) -> Self { self.div_euclid(rhs) }
	fn rem_euclid(self, rhs: Self) -> Self { self.rem_euclid(rhs) }
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar) -> Self { Self::new(x, y, z) }
	fn from_array(a: [Self::Scalar; 3]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 3] { self.to_array() }
	fn extend(self, v: Self::Scalar) -> Self::SignedVec4Type { self.extend(v) }
	fn truncate(self) -> Self::SignedVec2Type { self.truncate() }
	fn cross(self, rhs: Self) -> Self { self.cross(rhs) }
}

impl SignedVec3 for glam::Vec3A {
	type Scalar = f32;
	type BVec3Type = glam::BVec3A;
	type SignedVec2Type = glam::Vec2;
	type SignedVec4Type = glam::Vec4;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const NEG_ONE: Self = Self::NEG_ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
	const Z: Self = Self::Z;
	const NEG_X: Self = Self::NEG_X;
	const NEG_Y: Self = Self::NEG_Y;
	const NEG_Z: Self = Self::NEG_Z;
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
	fn abs(self) -> Self { self.abs() }
	fn signum(self) -> Self { self.signum() }
	fn is_negative_bitmask(self) -> u32 { self.is_negative_bitmask() }
	fn distance_squared(self, rhs: Self) -> Self::Scalar { self.distance_squared(rhs) }
	fn div_euclid(self, rhs: Self) -> Self { self.div_euclid(rhs) }
	fn rem_euclid(self, rhs: Self) -> Self { self.rem_euclid(rhs) }
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar) -> Self { Self::new(x, y, z) }
	fn from_array(a: [Self::Scalar; 3]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 3] { self.to_array() }
	fn extend(self, v: Self::Scalar) -> Self::SignedVec4Type { self.extend(v) }
	fn truncate(self) -> Self::SignedVec2Type { self.truncate() }
	fn cross(self, rhs: Self) -> Self { self.cross(rhs) }
}

impl SignedVec3 for glam::DVec3 {
	type Scalar = f64;
	type BVec3Type = glam::BVec3;
	type SignedVec2Type = glam::DVec2;
	type SignedVec4Type = glam::DVec4;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const NEG_ONE: Self = Self::NEG_ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
	const Z: Self = Self::Z;
	const NEG_X: Self = Self::NEG_X;
	const NEG_Y: Self = Self::NEG_Y;
	const NEG_Z: Self = Self::NEG_Z;
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
	fn abs(self) -> Self { self.abs() }
	fn signum(self) -> Self { self.signum() }
	fn is_negative_bitmask(self) -> u32 { self.is_negative_bitmask() }
	fn distance_squared(self, rhs: Self) -> Self::Scalar { self.distance_squared(rhs) }
	fn div_euclid(self, rhs: Self) -> Self { self.div_euclid(rhs) }
	fn rem_euclid(self, rhs: Self) -> Self { self.rem_euclid(rhs) }
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar) -> Self { Self::new(x, y, z) }
	fn from_array(a: [Self::Scalar; 3]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 3] { self.to_array() }
	fn extend(self, v: Self::Scalar) -> Self::SignedVec4Type { self.extend(v) }
	fn truncate(self) -> Self::SignedVec2Type { self.truncate() }
	fn cross(self, rhs: Self) -> Self { self.cross(rhs) }
}
