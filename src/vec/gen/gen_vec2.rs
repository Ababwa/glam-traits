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
use crate::GVec3;

pub trait GVec2
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
	,
	Self::GVec3Type: GVec3<Scalar = Self::Scalar>,
{
	type Scalar;
	type GVec3Type;
	const ZERO: Self;
	const ONE: Self;
	const MIN: Self;
	const MAX: Self;
	const X: Self;
	const Y: Self;
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
	fn new(x: Self::Scalar, y: Self::Scalar) -> Self;
	fn from_array(a: [Self::Scalar; 2]) -> Self;
	fn to_array(&self) -> [Self::Scalar; 2];
	fn extend(self, v: Self::Scalar) -> Self::GVec3Type;
}

impl GVec2 for glam::I16Vec2 {
	type Scalar = i16;
	type GVec3Type = glam::I16Vec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
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
	fn new(x: Self::Scalar, y: Self::Scalar) -> Self { Self::new(x, y) }
	fn from_array(a: [Self::Scalar; 2]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 2] { self.to_array() }
	fn extend(self, v: Self::Scalar) -> Self::GVec3Type { self.extend(v) }
}

impl GVec2 for glam::U16Vec2 {
	type Scalar = u16;
	type GVec3Type = glam::U16Vec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
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
	fn new(x: Self::Scalar, y: Self::Scalar) -> Self { Self::new(x, y) }
	fn from_array(a: [Self::Scalar; 2]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 2] { self.to_array() }
	fn extend(self, v: Self::Scalar) -> Self::GVec3Type { self.extend(v) }
}

impl GVec2 for glam::IVec2 {
	type Scalar = i32;
	type GVec3Type = glam::IVec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
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
	fn new(x: Self::Scalar, y: Self::Scalar) -> Self { Self::new(x, y) }
	fn from_array(a: [Self::Scalar; 2]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 2] { self.to_array() }
	fn extend(self, v: Self::Scalar) -> Self::GVec3Type { self.extend(v) }
}

impl GVec2 for glam::UVec2 {
	type Scalar = u32;
	type GVec3Type = glam::UVec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
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
	fn new(x: Self::Scalar, y: Self::Scalar) -> Self { Self::new(x, y) }
	fn from_array(a: [Self::Scalar; 2]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 2] { self.to_array() }
	fn extend(self, v: Self::Scalar) -> Self::GVec3Type { self.extend(v) }
}

impl GVec2 for glam::I64Vec2 {
	type Scalar = i64;
	type GVec3Type = glam::I64Vec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
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
	fn new(x: Self::Scalar, y: Self::Scalar) -> Self { Self::new(x, y) }
	fn from_array(a: [Self::Scalar; 2]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 2] { self.to_array() }
	fn extend(self, v: Self::Scalar) -> Self::GVec3Type { self.extend(v) }
}

impl GVec2 for glam::U64Vec2 {
	type Scalar = u64;
	type GVec3Type = glam::U64Vec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
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
	fn new(x: Self::Scalar, y: Self::Scalar) -> Self { Self::new(x, y) }
	fn from_array(a: [Self::Scalar; 2]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 2] { self.to_array() }
	fn extend(self, v: Self::Scalar) -> Self::GVec3Type { self.extend(v) }
}

impl GVec2 for glam::Vec2 {
	type Scalar = f32;
	type GVec3Type = glam::Vec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
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
	fn new(x: Self::Scalar, y: Self::Scalar) -> Self { Self::new(x, y) }
	fn from_array(a: [Self::Scalar; 2]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 2] { self.to_array() }
	fn extend(self, v: Self::Scalar) -> Self::GVec3Type { self.extend(v) }
}

impl GVec2 for glam::DVec2 {
	type Scalar = f64;
	type GVec3Type = glam::DVec3;
	const ZERO: Self = Self::ZERO;
	const ONE: Self = Self::ONE;
	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;
	const X: Self = Self::X;
	const Y: Self = Self::Y;
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
	fn new(x: Self::Scalar, y: Self::Scalar) -> Self { Self::new(x, y) }
	fn from_array(a: [Self::Scalar; 2]) -> Self { Self::from_array(a) }
	fn to_array(&self) -> [Self::Scalar; 2] { self.to_array() }
	fn extend(self, v: Self::Scalar) -> Self::GVec3Type { self.extend(v) }
}