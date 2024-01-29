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
	},
};
use num_traits::{
	NumAssign,
	FromPrimitive,
	ToPrimitive,
};
use crate::GBVec4;
use crate::GVec3;

pub trait GVec4
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
		AsRef<[Self::Scalar; 4]> +
		AsMut<[Self::Scalar; 4]> +
		From<[Self::Scalar; 4]> +
		Into<[Self::Scalar; 4]> +
		From<(Self::Scalar, Self::Scalar, Self::Scalar, Self::Scalar)> +
		Into<(Self::Scalar, Self::Scalar, Self::Scalar, Self::Scalar)> +
		From<(Self::GVec3Type, Self::Scalar)> +
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
	,
	Self::BVec4Type: GBVec4,
	Self::GVec3Type: GVec3<Scalar = Self::Scalar>,
{
	type Scalar;
	type BVec4Type;
	type GVec3Type;
	const ZERO: Self;
	const ONE: Self;
	const MIN: Self;
	const MAX: Self;
	const X: Self;
	const Y: Self;
	const Z: Self;
	const W: Self;
	const AXES: [Self; 4];
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
	fn select(mask: Self::BVec4Type, if_true: Self, if_false: Self) -> Self;
	fn cmpeq(self, rhs: Self) -> Self::BVec4Type;
	fn cmpne(self, rhs: Self) -> Self::BVec4Type;
	fn cmpge(self, rhs: Self) -> Self::BVec4Type;
	fn cmpgt(self, rhs: Self) -> Self::BVec4Type;
	fn cmple(self, rhs: Self) -> Self::BVec4Type;
	fn cmplt(self, rhs: Self) -> Self::BVec4Type;
	fn length_squared(self) -> Self::Scalar;
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar, w: Self::Scalar) -> Self;
	fn from_array(a: [Self::Scalar; 4]) -> Self;
	fn to_array(&self) -> [Self::Scalar; 4];
	fn truncate(self) -> Self::GVec3Type;
}

impl GVec4 for glam::I16Vec4 {
	type Scalar = i16;
	type BVec4Type = glam::BVec4;
	type GVec3Type = glam::I16Vec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
	const Z: Self = Self::Z;
	const W: Self = Self::W;
	const AXES: [Self; 4] = Self::AXES;
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
	fn select(mask: Self::BVec4Type, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVec4Type { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVec4Type { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVec4Type { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVec4Type { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVec4Type { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVec4Type { self.cmplt(rhs) }
	fn length_squared(self) -> Self::Scalar { self.length_squared() }
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar, w: Self::Scalar) -> Self { Self::new(x, y, z, w) }
	fn from_array(a: [Self::Scalar; 4]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 4] { self.to_array() }
	fn truncate(self) -> Self::GVec3Type { self.truncate() }
}

impl GVec4 for glam::U16Vec4 {
	type Scalar = u16;
	type BVec4Type = glam::BVec4;
	type GVec3Type = glam::U16Vec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
	const Z: Self = Self::Z;
	const W: Self = Self::W;
	const AXES: [Self; 4] = Self::AXES;
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
	fn select(mask: Self::BVec4Type, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVec4Type { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVec4Type { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVec4Type { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVec4Type { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVec4Type { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVec4Type { self.cmplt(rhs) }
	fn length_squared(self) -> Self::Scalar { self.length_squared() }
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar, w: Self::Scalar) -> Self { Self::new(x, y, z, w) }
	fn from_array(a: [Self::Scalar; 4]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 4] { self.to_array() }
	fn truncate(self) -> Self::GVec3Type { self.truncate() }
}

impl GVec4 for glam::IVec4 {
	type Scalar = i32;
	type BVec4Type = glam::BVec4;
	type GVec3Type = glam::IVec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
	const Z: Self = Self::Z;
	const W: Self = Self::W;
	const AXES: [Self; 4] = Self::AXES;
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
	fn select(mask: Self::BVec4Type, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVec4Type { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVec4Type { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVec4Type { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVec4Type { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVec4Type { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVec4Type { self.cmplt(rhs) }
	fn length_squared(self) -> Self::Scalar { self.length_squared() }
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar, w: Self::Scalar) -> Self { Self::new(x, y, z, w) }
	fn from_array(a: [Self::Scalar; 4]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 4] { self.to_array() }
	fn truncate(self) -> Self::GVec3Type { self.truncate() }
}

impl GVec4 for glam::UVec4 {
	type Scalar = u32;
	type BVec4Type = glam::BVec4;
	type GVec3Type = glam::UVec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
	const Z: Self = Self::Z;
	const W: Self = Self::W;
	const AXES: [Self; 4] = Self::AXES;
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
	fn select(mask: Self::BVec4Type, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVec4Type { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVec4Type { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVec4Type { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVec4Type { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVec4Type { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVec4Type { self.cmplt(rhs) }
	fn length_squared(self) -> Self::Scalar { self.length_squared() }
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar, w: Self::Scalar) -> Self { Self::new(x, y, z, w) }
	fn from_array(a: [Self::Scalar; 4]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 4] { self.to_array() }
	fn truncate(self) -> Self::GVec3Type { self.truncate() }
}

impl GVec4 for glam::I64Vec4 {
	type Scalar = i64;
	type BVec4Type = glam::BVec4;
	type GVec3Type = glam::I64Vec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
	const Z: Self = Self::Z;
	const W: Self = Self::W;
	const AXES: [Self; 4] = Self::AXES;
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
	fn select(mask: Self::BVec4Type, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVec4Type { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVec4Type { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVec4Type { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVec4Type { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVec4Type { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVec4Type { self.cmplt(rhs) }
	fn length_squared(self) -> Self::Scalar { self.length_squared() }
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar, w: Self::Scalar) -> Self { Self::new(x, y, z, w) }
	fn from_array(a: [Self::Scalar; 4]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 4] { self.to_array() }
	fn truncate(self) -> Self::GVec3Type { self.truncate() }
}

impl GVec4 for glam::U64Vec4 {
	type Scalar = u64;
	type BVec4Type = glam::BVec4;
	type GVec3Type = glam::U64Vec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
	const Z: Self = Self::Z;
	const W: Self = Self::W;
	const AXES: [Self; 4] = Self::AXES;
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
	fn select(mask: Self::BVec4Type, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVec4Type { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVec4Type { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVec4Type { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVec4Type { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVec4Type { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVec4Type { self.cmplt(rhs) }
	fn length_squared(self) -> Self::Scalar { self.length_squared() }
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar, w: Self::Scalar) -> Self { Self::new(x, y, z, w) }
	fn from_array(a: [Self::Scalar; 4]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 4] { self.to_array() }
	fn truncate(self) -> Self::GVec3Type { self.truncate() }
}

impl GVec4 for glam::Vec4 {
	type Scalar = f32;
	type BVec4Type = glam::BVec4A;
	type GVec3Type = glam::Vec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
	const Z: Self = Self::Z;
	const W: Self = Self::W;
	const AXES: [Self; 4] = Self::AXES;
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
	fn select(mask: Self::BVec4Type, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVec4Type { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVec4Type { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVec4Type { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVec4Type { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVec4Type { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVec4Type { self.cmplt(rhs) }
	fn length_squared(self) -> Self::Scalar { self.length_squared() }
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar, w: Self::Scalar) -> Self { Self::new(x, y, z, w) }
	fn from_array(a: [Self::Scalar; 4]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 4] { self.to_array() }
	fn truncate(self) -> Self::GVec3Type { self.truncate() }
}

impl GVec4 for glam::DVec4 {
	type Scalar = f64;
	type BVec4Type = glam::BVec4;
	type GVec3Type = glam::DVec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
	const Z: Self = Self::Z;
	const W: Self = Self::W;
	const AXES: [Self; 4] = Self::AXES;
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
	fn select(mask: Self::BVec4Type, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
	fn cmpeq(self, rhs: Self) -> Self::BVec4Type { self.cmpeq(rhs) }
	fn cmpne(self, rhs: Self) -> Self::BVec4Type { self.cmpne(rhs) }
	fn cmpge(self, rhs: Self) -> Self::BVec4Type { self.cmpge(rhs) }
	fn cmpgt(self, rhs: Self) -> Self::BVec4Type { self.cmpgt(rhs) }
	fn cmple(self, rhs: Self) -> Self::BVec4Type { self.cmple(rhs) }
	fn cmplt(self, rhs: Self) -> Self::BVec4Type { self.cmplt(rhs) }
	fn length_squared(self) -> Self::Scalar { self.length_squared() }
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar, w: Self::Scalar) -> Self { Self::new(x, y, z, w) }
	fn from_array(a: [Self::Scalar; 4]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 4] { self.to_array() }
	fn truncate(self) -> Self::GVec3Type { self.truncate() }
}
