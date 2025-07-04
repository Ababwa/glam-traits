/*!
Traits for the vectors in [glam](https://github.com/bitshifter/glam-rs/).

There are traits for the following characteristics:
* "Any"
* Signed
* Float
* Int
* Signed int
* Unsigned int

For lengths "any", 2, 3 and 4:

|            |Any          |2             |3             |4             |
|------------|-------------|--------------|--------------|--------------|
|Any         |[`GVec`]     |[`GVec2`]     |[`GVec3`]     |[`GVec4`]     |
|Signed      |[`SignedVec`]|[`SignedVec2`]|[`SignedVec3`]|[`SignedVec4`]|
|Float       |[`FloatVec`] |[`FloatVec2`] |[`FloatVec3`] |[`FloatVec4`] |
|Int         |[`IntVec`]   |[`IntVec2`]   |[`IntVec3`]   |[`IntVec4`]   |
|Signed int  |[`SIntVec`]  |[`SIntVec2`]  |[`SIntVec3`]  |[`SIntVec4`]  |
|Unsigned int|[`UIntVec`]  |[`UIntVec2`]  |[`UIntVec3`]  |[`UIntVec4`]  |

As well as for concrete types of any length:

[`I8Vec`], [`U8Vec`], [`I16Vec`], [`U16Vec`], [`I32Vec`], [`U32Vec`], [`I64Vec`], [`U64Vec`], [`USizeVec`], [`F32Vec`], [`F64Vec`]

[`BVec`] is also provided to cover boolean vectors.

Traits are implemented for the appropriate `glam` types.
*/

use std::{
	fmt::{Debug, Display},
	iter::{Product, Sum},
	hash::Hash,
	ops::{
		Add, AddAssign,
		Sub, SubAssign,
		Mul, MulAssign,
		Div, DivAssign,
		Rem, RemAssign,
		BitAnd, BitAndAssign,
		BitOr, BitOrAssign,
		BitXor, BitXorAssign,
		Index, IndexMut,
		Neg, Not,
		Shl, Shr,
	},
};
use glam::{
	BVec2, BVec3, BVec3A, BVec4, BVec4A,
	I8Vec2, I8Vec3, I8Vec4,
	U8Vec2, U8Vec3, U8Vec4,
	I16Vec2, I16Vec3, I16Vec4,
	U16Vec2, U16Vec3, U16Vec4,
	IVec2, IVec3, IVec4,
	UVec2, UVec3, UVec4,
	I64Vec2, I64Vec3, I64Vec4,
	U64Vec2, U64Vec3, U64Vec4,
	USizeVec2, USizeVec3, USizeVec4,
	Vec2, Vec3, Vec3A, Vec4,
	DVec2, DVec3, DVec4,
};

mod private {
	pub trait Sealed {}
}
use private::Sealed;

/**
Vector of any length whose elements are [`bool`].
*/
pub trait BVec
where
	Self:
		Sealed +
		Clone +
		Copy +
		PartialEq +
		Eq +
		Hash +
		Default +
		Not<Output = Self> +
		BitAnd<Output = Self> +
		BitAndAssign +
		BitOr<Output = Self> +
		BitOrAssign +
		BitXor<Output = Self> +
		BitXorAssign +
		Debug +
		Display +
		From<Self::Array> +
		Into<Self::Array> +
	,
	Self::Array: Index<usize, Output = bool>,
{
	type Array;
	const FALSE: Self;
	const TRUE: Self;
	const DIM: usize;
	fn splat(v: bool) -> Self;
	fn from_array(a: Self::Array) -> Self;
	fn bitmask(self) -> u32;
	fn any(self) -> bool;
	fn all(self) -> bool;
	fn test(&self, index: usize) -> bool;
	fn set(&mut self, index: usize, value: bool);
}

macro_rules! impl_gbvec {
	($type:ty, $dim:literal) => {
		impl Sealed for $type {}
		impl BVec for $type {
			type Array = [bool; $dim];
			const FALSE: Self = Self::FALSE;
			const TRUE: Self = Self::TRUE;
			const DIM: usize = $dim;
			fn splat(v: bool) -> Self { Self::splat(v) }
			fn from_array(a: Self::Array) -> Self { Self::from_array(a) }
			fn bitmask(self) -> u32 { self.bitmask() }
			fn any(self) -> bool { self.any() }
			fn all(self) -> bool { self.all() }
			fn test(&self, index: usize) -> bool { self.test(index) }
			fn set(&mut self, index: usize, value: bool) { self.set(index, value) }
		}
	};
}

impl_gbvec!(BVec2, 2);
impl_gbvec!(BVec3, 3);
impl_gbvec!(BVec3A, 3);
impl_gbvec!(BVec4, 4);
impl_gbvec!(BVec4A, 4);

/**
Generic vector of any length.
*/
pub trait GVec
where
	for <'a> Self:
		Sealed +
		Clone +
		Copy +
		PartialEq +
		Default +
		Display +
		Debug +
		Add<Output = Self> +
		Add<&'a Self, Output = Self> +
		Add<Self::Scalar, Output = Self> +
		Add<&'a Self::Scalar, Output = Self> +
		AddAssign +
		AddAssign<&'a Self> +
		AddAssign<Self::Scalar> +
		AddAssign<&'a Self::Scalar> +
		Sub<Output = Self> +
		Sub<&'a Self, Output = Self> +
		Sub<Self::Scalar, Output = Self> +
		Sub<&'a Self::Scalar, Output = Self> +
		SubAssign +
		SubAssign<&'a Self> +
		SubAssign<Self::Scalar> +
		SubAssign<&'a Self::Scalar> +
		Mul<Output = Self> +
		Mul<&'a Self, Output = Self> +
		Mul<Self::Scalar, Output = Self> +
		Mul<&'a Self::Scalar, Output = Self> +
		MulAssign +
		MulAssign<&'a Self> +
		MulAssign<Self::Scalar> +
		MulAssign<&'a Self::Scalar> +
		Div<Output = Self> +
		Div<&'a Self, Output = Self> +
		Div<Self::Scalar, Output = Self> +
		Div<&'a Self::Scalar, Output = Self> +
		DivAssign +
		DivAssign<&'a Self> +
		DivAssign<Self::Scalar> +
		DivAssign<&'a Self::Scalar> +
		Rem<Output = Self> +
		Rem<&'a Self, Output = Self> +
		Rem<Self::Scalar, Output = Self> +
		Rem<&'a Self::Scalar, Output = Self> +
		RemAssign +
		RemAssign<&'a Self> +
		RemAssign<Self::Scalar> +
		RemAssign<&'a Self::Scalar> +
		AsRef<Self::Array> +
		AsMut<Self::Array> +
		From<Self::Array> +
		Into<Self::Array> +
		Sum +
		Sum<&'a Self> +
		Product +
		Product<&'a Self> +
		Index<usize, Output = Self::Scalar> +
		IndexMut<usize, Output = Self::Scalar> +
		From<Self::BVec> +
	,
	for <'a> Self::Scalar:
		'static +
		Copy +
		PartialOrd +
		Add<Self, Output = Self> +
		Add<&'a Self, Output = Self> +
		Sub<Self, Output = Self> +
		Sub<&'a Self, Output = Self> +
		Mul<Self, Output = Self> +
		Mul<&'a Self, Output = Self> +
		Div<Self, Output = Self> +
		Div<&'a Self, Output = Self> +
		Rem<Self, Output = Self> +
		Rem<&'a Self, Output = Self> +
	,
	Self::BVec: BVec,
	Self::Axes: Index<usize, Output = Self>,
	Self::Array: Index<usize, Output = Self::Scalar>,
	Self::I8Vec: I8Vec,
	Self::U8Vec: U8Vec,
	Self::I16Vec: I16Vec,
	Self::U16Vec: U16Vec,
	Self::I32Vec: I32Vec,
	Self::U32Vec: U32Vec,
	Self::I64Vec: I64Vec,
	Self::U64Vec: U64Vec,
	Self::USizeVec: USizeVec,
	Self::F32Vec: F32Vec,
	Self::F64Vec: F64Vec,
{
	type Scalar;
	type BVec;
	type Axes;
	type Array;
	type I8Vec;
	type U8Vec;
	type I16Vec;
	type U16Vec;
	type I32Vec;
	type U32Vec;
	type I64Vec;
	type U64Vec;
	type USizeVec;
	type F32Vec;
	type F64Vec;
	const ZERO: Self;
	const ONE: Self;
	const MIN: Self;
	const MAX: Self;
	const AXES: Self::Axes;
	const DIM: usize;
	fn splat(v: Self::Scalar) -> Self;
	fn map<F: Fn(Self::Scalar) -> Self::Scalar>(self, f: F) -> Self;
	fn from_array(a: Self::Array) -> Self;
	fn to_array(&self) -> Self::Array;
	fn from_slice(slice: &[Self::Scalar]) -> Self;
	fn write_to_slice(self, slice: &mut [Self::Scalar]);
	fn dot(self, rhs: Self) -> Self::Scalar;
	fn dot_into_vec(self, rhs: Self) -> Self;
	fn min(self, rhs: Self) -> Self;
	fn max(self, rhs: Self) -> Self;
	fn clamp(self, min: Self, max: Self) -> Self;
	fn min_element(self) -> Self::Scalar;
	fn max_element(self) -> Self::Scalar;
	fn min_position(self) -> usize;
	fn max_position(self) -> usize;
	fn element_sum(self) -> Self::Scalar;
	fn element_product(self) -> Self::Scalar;
	fn select(mask: Self::BVec, if_true: Self, if_false: Self) -> Self;
	fn cmpeq(self, rhs: Self) -> Self::BVec;
	fn cmpne(self, rhs: Self) -> Self::BVec;
	fn cmpge(self, rhs: Self) -> Self::BVec;
	fn cmpgt(self, rhs: Self) -> Self::BVec;
	fn cmple(self, rhs: Self) -> Self::BVec;
	fn cmplt(self, rhs: Self) -> Self::BVec;
	fn length_squared(self) -> Self::Scalar;
	fn as_i8vec(&self) -> Self::I8Vec;
	fn as_u8vec(&self) -> Self::U8Vec;
	fn as_i16vec(&self) -> Self::I16Vec;
	fn as_u16vec(&self) -> Self::U16Vec;
	fn as_ivec(&self) -> Self::I32Vec;
	fn as_uvec(&self) -> Self::U32Vec;
	fn as_i64vec(&self) -> Self::I64Vec;
	fn as_u64vec(&self) -> Self::U64Vec;
	fn as_usizevec(&self) -> Self::USizeVec;
	fn as_vec(&self) -> Self::F32Vec;
	fn as_dvec(&self) -> Self::F64Vec;
}

macro_rules! as_types {
	(2) => {
		type I8Vec = I8Vec2;
		type U8Vec = U8Vec2;
		type I16Vec = I16Vec2;
		type U16Vec = U16Vec2;
		type I32Vec = IVec2;
		type U32Vec = UVec2;
		type I64Vec = I64Vec2;
		type U64Vec = U64Vec2;
		type USizeVec = USizeVec2;
		type F32Vec = Vec2;
		type F64Vec = DVec2;
	};
	(3) => {
		type I8Vec = I8Vec3;
		type U8Vec = U8Vec3;
		type I16Vec = I16Vec3;
		type U16Vec = U16Vec3;
		type I32Vec = IVec3;
		type U32Vec = UVec3;
		type I64Vec = I64Vec3;
		type U64Vec = U64Vec3;
		type USizeVec = USizeVec3;
		type F32Vec = Vec3;
		type F64Vec = DVec3;
	};
	(4) => {
		type I8Vec = I8Vec4;
		type U8Vec = U8Vec4;
		type I16Vec = I16Vec4;
		type U16Vec = U16Vec4;
		type I32Vec = IVec4;
		type U32Vec = UVec4;
		type I64Vec = I64Vec4;
		type U64Vec = U64Vec4;
		type USizeVec = USizeVec4;
		type F32Vec = Vec4;
		type F64Vec = DVec4;
	};
}

macro_rules! impl_as {
	($fn_name:ident, $out:ty, ($($comp:ident),*)) => {
		fn $fn_name(&self) -> $out {
			<$out>::new($(self.$comp as <$out as GVec>::Scalar),*)
		}
	};
	($fn_name:ident, $out:ty, 2) => {
		impl_as!($fn_name, $out, (x, y));
	};
	($fn_name:ident, $out:ty, 3) => {
		impl_as!($fn_name, $out, (x, y, z));
	};
	($fn_name:ident, $out:ty, 4) => {
		impl_as!($fn_name, $out, (x, y, z, w));
	};
}

macro_rules! impl_gvec {
	($type:ty, $scalar:ty, $bvec:ty, $dim:tt) => {
		impl Sealed for $type {}
		impl GVec for $type {
			type Scalar = $scalar;
			type BVec = $bvec;
			type Axes = [Self; $dim];
			type Array = [$scalar; $dim];
			as_types!($dim);
			const ZERO: Self = Self::ZERO;
			const ONE: Self = Self::ONE;
			const MIN: Self = Self::MIN;
			const MAX: Self = Self::MAX;
			const AXES: Self::Axes = Self::AXES;
			const DIM: usize = $dim;
			fn splat(v: Self::Scalar) -> Self { Self::splat(v) }
			fn map<F: Fn(Self::Scalar) -> Self::Scalar>(self, f: F) -> Self { self.map(f) }
			fn from_array(a: Self::Array) -> Self { Self::from_array(a) }
			fn to_array(&self) -> Self::Array { self.to_array() }
			fn from_slice(slice: &[Self::Scalar]) -> Self { Self::from_slice(slice) }
			fn write_to_slice(self, slice: &mut [Self::Scalar]) { self.write_to_slice(slice) }
			fn dot(self, rhs: Self) -> Self::Scalar { self.dot(rhs) }
			fn dot_into_vec(self, rhs: Self) -> Self { self.dot_into_vec(rhs) }
			fn min(self, rhs: Self) -> Self { self.min(rhs) }
			fn max(self, rhs: Self) -> Self { self.max(rhs) }
			fn clamp(self, min: Self, max: Self) -> Self { self.clamp(min, max) }
			fn min_element(self) -> Self::Scalar { self.min_element() }
			fn max_element(self) -> Self::Scalar { self.max_element() }
			fn min_position(self) -> usize { self.min_position() }
			fn max_position(self) -> usize { self.max_position() }
			fn element_sum(self) -> Self::Scalar { self.element_sum() }
			fn element_product(self) -> Self::Scalar { self.element_product() }
			fn select(mask: Self::BVec, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
			fn cmpeq(self, rhs: Self) -> Self::BVec { self.cmpeq(rhs) }
			fn cmpne(self, rhs: Self) -> Self::BVec { self.cmpne(rhs) }
			fn cmpge(self, rhs: Self) -> Self::BVec { self.cmpge(rhs) }
			fn cmpgt(self, rhs: Self) -> Self::BVec { self.cmpgt(rhs) }
			fn cmple(self, rhs: Self) -> Self::BVec { self.cmple(rhs) }
			fn cmplt(self, rhs: Self) -> Self::BVec { self.cmplt(rhs) }
			fn length_squared(self) -> Self::Scalar { self.length_squared() }
			impl_as!(as_i8vec, Self::I8Vec, $dim);
			impl_as!(as_u8vec, Self::U8Vec, $dim);
			impl_as!(as_i16vec, Self::I16Vec, $dim);
			impl_as!(as_u16vec, Self::U16Vec, $dim);
			impl_as!(as_ivec, Self::I32Vec, $dim);
			impl_as!(as_uvec, Self::U32Vec, $dim);
			impl_as!(as_i64vec, Self::I64Vec, $dim);
			impl_as!(as_u64vec, Self::U64Vec, $dim);
			impl_as!(as_usizevec, Self::USizeVec, $dim);
			impl_as!(as_vec, Self::F32Vec, $dim);
			impl_as!(as_dvec, Self::F64Vec, $dim);
		}
	};
}
pub(crate) use impl_gvec;

impl_gvec!(I8Vec2, i8, BVec2, 2);
impl_gvec!(I8Vec3, i8, BVec3, 3);
impl_gvec!(I8Vec4, i8, BVec4, 4);
impl_gvec!(U8Vec2, u8, BVec2, 2);
impl_gvec!(U8Vec3, u8, BVec3, 3);
impl_gvec!(U8Vec4, u8, BVec4, 4);
impl_gvec!(I16Vec2, i16, BVec2, 2);
impl_gvec!(I16Vec3, i16, BVec3, 3);
impl_gvec!(I16Vec4, i16, BVec4, 4);
impl_gvec!(U16Vec2, u16, BVec2, 2);
impl_gvec!(U16Vec3, u16, BVec3, 3);
impl_gvec!(U16Vec4, u16, BVec4, 4);
impl_gvec!(IVec2, i32, BVec2, 2);
impl_gvec!(IVec3, i32, BVec3, 3);
impl_gvec!(IVec4, i32, BVec4, 4);
impl_gvec!(UVec2, u32, BVec2, 2);
impl_gvec!(UVec3, u32, BVec3, 3);
impl_gvec!(UVec4, u32, BVec4, 4);
impl_gvec!(I64Vec2, i64, BVec2, 2);
impl_gvec!(I64Vec3, i64, BVec3, 3);
impl_gvec!(I64Vec4, i64, BVec4, 4);
impl_gvec!(U64Vec2, u64, BVec2, 2);
impl_gvec!(U64Vec3, u64, BVec3, 3);
impl_gvec!(U64Vec4, u64, BVec4, 4);
impl_gvec!(USizeVec2, usize, BVec2, 2);
impl_gvec!(USizeVec3, usize, BVec3, 3);
impl_gvec!(USizeVec4, usize, BVec4, 4);
impl_gvec!(Vec2, f32, BVec2, 2);
impl_gvec!(Vec3, f32, BVec3, 3);
impl_gvec!(Vec3A, f32, BVec3A, 3);
impl_gvec!(Vec4, f32, BVec4A, 4);
impl_gvec!(DVec2, f64, BVec2, 2);
impl_gvec!(DVec3, f64, BVec3, 3);
impl_gvec!(DVec4, f64, BVec4, 4);

/**
Generic vector of length 2.
*/
pub trait GVec2
where
	Self:
		GVec<
			Axes = [Self; 2],
			Array = [<Self as GVec>::Scalar; 2],
			I8Vec = I8Vec2,
			U8Vec = U8Vec2,
			I16Vec = I16Vec2,
			U16Vec = U16Vec2,
			I32Vec = IVec2,
			U32Vec = UVec2,
			I64Vec = I64Vec2,
			U64Vec = U64Vec2,
			USizeVec = USizeVec2,
			F32Vec = Vec2,
			F64Vec = DVec2,
		> +
		From<(Self::Scalar, Self::Scalar)> +
		Into<(Self::Scalar, Self::Scalar)> +
	,
{
	type Extended;
	const X: Self;
	const Y: Self;
	fn x(self) -> Self::Scalar;
	fn y(self) -> Self::Scalar;
	fn new(x: Self::Scalar, y: Self::Scalar) -> Self;
	fn extend(self, z: Self::Scalar) -> Self::Extended;
	fn with_x(self, x: Self::Scalar) -> Self;
	fn with_y(self, y: Self::Scalar) -> Self;
}

macro_rules! impl_gvec2 {
	($type:ty, $extended:ty) => {
		impl GVec2 for $type {
			type Extended = $extended;
			const X: Self = Self::X;
			const Y: Self = Self::Y;
			fn x(self) -> Self::Scalar { self.x }
			fn y(self) -> Self::Scalar { self.y }
			fn new(x: Self::Scalar, y: Self::Scalar) -> Self { Self::new(x, y) }
			fn extend(self, z: Self::Scalar) -> Self::Extended { self.extend(z) }
			fn with_x(self, x: Self::Scalar) -> Self { self.with_x(x) }
			fn with_y(self, y: Self::Scalar) -> Self { self.with_y(y) }
		}
	};
}
pub(crate) use impl_gvec2;

impl_gvec2!(I8Vec2, I8Vec3);
impl_gvec2!(U8Vec2, U8Vec3);
impl_gvec2!(I16Vec2, I16Vec3);
impl_gvec2!(U16Vec2, U16Vec3);
impl_gvec2!(IVec2, IVec3);
impl_gvec2!(UVec2, UVec3);
impl_gvec2!(I64Vec2, I64Vec3);
impl_gvec2!(U64Vec2, U64Vec3);
impl_gvec2!(USizeVec2, USizeVec3);
impl_gvec2!(Vec2, Vec3);
impl_gvec2!(DVec2, DVec3);

/**
Generic vector of length 3.
*/
pub trait GVec3
where
	Self:
		GVec<
			Axes = [Self; 3],
			Array = [<Self as GVec>::Scalar; 3],
			I8Vec = I8Vec3,
			U8Vec = U8Vec3,
			I16Vec = I16Vec3,
			U16Vec = U16Vec3,
			I32Vec = IVec3,
			U32Vec = UVec3,
			I64Vec = I64Vec3,
			U64Vec = U64Vec3,
			USizeVec = USizeVec3,
			F32Vec = Vec3,
			F64Vec = DVec3,
		> +
		From<(Self::Scalar, Self::Scalar, Self::Scalar)> +
		Into<(Self::Scalar, Self::Scalar, Self::Scalar)> +
	,
{
	type Extended;
	type Truncated;
	const X: Self;
	const Y: Self;
	const Z: Self;
	fn x(self) -> Self::Scalar;
	fn y(self) -> Self::Scalar;
	fn z(self) -> Self::Scalar;
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar) -> Self;
	fn extend(self, w: Self::Scalar) -> Self::Extended;
	fn truncate(self) -> Self::Truncated;
	fn with_x(self, x: Self::Scalar) -> Self;
	fn with_y(self, y: Self::Scalar) -> Self;
	fn with_z(self, z: Self::Scalar) -> Self;
	fn cross(self, rhs: Self) -> Self;
}

macro_rules! impl_gvec3 {
	($type:ty, $extended:ty, $truncated: ty) => {
		impl GVec3 for $type {
			type Extended = $extended;
			type Truncated = $truncated;
			const X: Self = Self::X;
			const Y: Self = Self::Y;
			const Z: Self = Self::Z;
			fn x(self) -> Self::Scalar { self.x }
			fn y(self) -> Self::Scalar { self.y }
			fn z(self) -> Self::Scalar { self.z }
			fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar) -> Self { Self::new(x, y, z) }
			fn extend(self, w: Self::Scalar) -> Self::Extended { self.extend(w) }
			fn truncate(self) -> Self::Truncated { self.truncate() }
			fn with_x(self, x: Self::Scalar) -> Self { self.with_x(x) }
			fn with_y(self, y: Self::Scalar) -> Self { self.with_y(y) }
			fn with_z(self, z: Self::Scalar) -> Self { self.with_z(z) }
			fn cross(self, rhs: Self) -> Self { self.cross(rhs) }
		}
	};
}
pub(crate) use impl_gvec3;

impl_gvec3!(I8Vec3, I8Vec4, I8Vec2);
impl_gvec3!(U8Vec3, U8Vec4, U8Vec2);
impl_gvec3!(I16Vec3, I16Vec4, I16Vec2);
impl_gvec3!(U16Vec3, U16Vec4, U16Vec2);
impl_gvec3!(IVec3, IVec4, IVec2);
impl_gvec3!(UVec3, UVec4, UVec2);
impl_gvec3!(I64Vec3, I64Vec4, I64Vec2);
impl_gvec3!(U64Vec3, U64Vec4, U64Vec2);
impl_gvec3!(USizeVec3, USizeVec4, USizeVec2);
impl_gvec3!(Vec3, Vec4, Vec2);
impl_gvec3!(Vec3A, Vec4, Vec2);
impl_gvec3!(DVec3, DVec4, DVec2);

/**
Generic vector of length 4.
*/
pub trait GVec4
where
	Self:
		GVec<
			Axes = [Self; 4],
			Array = [<Self as GVec>::Scalar; 4],
			I8Vec = I8Vec4,
			U8Vec = U8Vec4,
			I16Vec = I16Vec4,
			U16Vec = U16Vec4,
			I32Vec = IVec4,
			U32Vec = UVec4,
			I64Vec = I64Vec4,
			U64Vec = U64Vec4,
			USizeVec = USizeVec4,
			F32Vec = Vec4,
			F64Vec = DVec4,
		> +
		From<(Self::Scalar, Self::Scalar, Self::Scalar, Self::Scalar)> +
		Into<(Self::Scalar, Self::Scalar, Self::Scalar, Self::Scalar)> +
	,
{
	type Truncated;
	const X: Self;
	const Y: Self;
	const Z: Self;
	const W: Self;
	fn x(self) -> Self::Scalar;
	fn y(self) -> Self::Scalar;
	fn z(self) -> Self::Scalar;
	fn w(self) -> Self::Scalar;
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar, w: Self::Scalar) -> Self;
	fn truncate(self) -> Self::Truncated;
	fn with_x(self, x: Self::Scalar) -> Self;
	fn with_y(self, y: Self::Scalar) -> Self;
	fn with_z(self, z: Self::Scalar) -> Self;
	fn with_w(self, w: Self::Scalar) -> Self;
}

macro_rules! impl_gvec4 {
	($type:ty, $truncated:ty) => {
		impl GVec4 for $type {
			type Truncated = $truncated;
			const X: Self = Self::X;
			const Y: Self = Self::Y;
			const Z: Self = Self::Z;
			const W: Self = Self::W;
			fn x(self) -> Self::Scalar { self.x }
			fn y(self) -> Self::Scalar { self.y }
			fn z(self) -> Self::Scalar { self.z }
			fn w(self) -> Self::Scalar { self.w }
			fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar, w: Self::Scalar) -> Self { Self::new(x, y, z, w) }
			fn truncate(self) -> Self::Truncated { self.truncate() }
			fn with_x(self, x: Self::Scalar) -> Self { self.with_x(x) }
			fn with_y(self, y: Self::Scalar) -> Self { self.with_y(y) }
			fn with_z(self, z: Self::Scalar) -> Self { self.with_z(z) }
			fn with_w(self, w: Self::Scalar) -> Self { self.with_w(w) }
		}
	};
}
pub(crate) use impl_gvec4;

impl_gvec4!(I8Vec4, I8Vec3);
impl_gvec4!(U8Vec4, U8Vec3);
impl_gvec4!(I16Vec4, I16Vec3);
impl_gvec4!(U16Vec4, U16Vec3);
impl_gvec4!(IVec4, IVec3);
impl_gvec4!(UVec4, UVec3);
impl_gvec4!(I64Vec4, I64Vec3);
impl_gvec4!(U64Vec4, U64Vec3);
impl_gvec4!(USizeVec4, USizeVec3);
impl_gvec4!(Vec4, Vec3);
impl_gvec4!(DVec4, DVec3);

/**
Vector of any length whose elements are a signed type.
*/
pub trait SignedVec: GVec + Neg {
	const NEG_ONE: Self;
	fn abs(self) -> Self;
	fn signum(self) -> Self;
	fn is_negative_bitmask(self) -> u32;
	fn distance_squared(self, rhs: Self) -> Self::Scalar;
	fn div_euclid(self, rhs: Self) -> Self;
	fn rem_euclid(self, rhs: Self) -> Self;
}

macro_rules! impl_signedvec {
	($type:ty) => {
		impl SignedVec for $type {
			const NEG_ONE: Self = Self::NEG_ONE;
			fn abs(self) -> Self { self.abs() }
			fn signum(self) -> Self { self.signum() }
			fn is_negative_bitmask(self) -> u32 { self.is_negative_bitmask() }
			fn distance_squared(self, rhs: Self) -> Self::Scalar { self.distance_squared(rhs) }
			fn div_euclid(self, rhs: Self) -> Self { self.div_euclid(rhs) }
			fn rem_euclid(self, rhs: Self) -> Self { self.rem_euclid(rhs) }
		}
	};
}
pub(crate) use impl_signedvec;

impl_signedvec!(I16Vec2);
impl_signedvec!(I16Vec3);
impl_signedvec!(I16Vec4);
impl_signedvec!(I8Vec2);
impl_signedvec!(I8Vec3);
impl_signedvec!(I8Vec4);
impl_signedvec!(IVec2);
impl_signedvec!(IVec3);
impl_signedvec!(IVec4);
impl_signedvec!(I64Vec2);
impl_signedvec!(I64Vec3);
impl_signedvec!(I64Vec4);
impl_signedvec!(Vec2);
impl_signedvec!(Vec3);
impl_signedvec!(Vec3A);
impl_signedvec!(Vec4);
impl_signedvec!(DVec2);
impl_signedvec!(DVec3);
impl_signedvec!(DVec4);

/**
Vector of length 2 whose elements are a signed type.
*/
pub trait SignedVec2: SignedVec + GVec2 {
	const NEG_X: Self;
	const NEG_Y: Self;
	fn perp(self) -> Self;
	fn perp_dot(self, rhs: Self) -> Self::Scalar;
	fn rotate(self, rhs: Self) -> Self;
}

macro_rules! impl_signedvec2 {
	($type:ty) => {
		impl SignedVec2 for $type {
			const NEG_X: Self = Self::NEG_X;
			const NEG_Y: Self = Self::NEG_Y;
			fn perp(self) -> Self { self.perp() }
			fn perp_dot(self, rhs: Self) -> Self::Scalar { self.perp_dot(rhs) }
			fn rotate(self, rhs: Self) -> Self { self.rotate(rhs) }
		}
	};
}
pub(crate) use impl_signedvec2;

impl_signedvec2!(I8Vec2);
impl_signedvec2!(I16Vec2);
impl_signedvec2!(IVec2);
impl_signedvec2!(I64Vec2);
impl_signedvec2!(Vec2);
impl_signedvec2!(DVec2);

/**
Vector of length 3 whose elements are a signed type.
*/
pub trait SignedVec3: SignedVec + GVec3 {
	const NEG_X: Self;
	const NEG_Y: Self;
	const NEG_Z: Self;
}

macro_rules! impl_signedvec3 {
	($type:ty) => {
		impl SignedVec3 for $type {
			const NEG_X: Self = Self::NEG_X;
			const NEG_Y: Self = Self::NEG_Y;
			const NEG_Z: Self = Self::NEG_Z;
		}
	};
}
pub(crate) use impl_signedvec3;

impl_signedvec3!(I8Vec3);
impl_signedvec3!(I16Vec3);
impl_signedvec3!(IVec3);
impl_signedvec3!(I64Vec3);
impl_signedvec3!(Vec3);
impl_signedvec3!(Vec3A);
impl_signedvec3!(DVec3);

/**
Vector of length 4 whose elements are a signed type.
*/
pub trait SignedVec4: SignedVec + GVec4 {
	const NEG_X: Self;
	const NEG_Y: Self;
	const NEG_Z: Self;
	const NEG_W: Self;
}

macro_rules! impl_signedvec4 {
	($type:ty) => {
		impl SignedVec4 for $type {
			const NEG_X: Self = Self::NEG_X;
			const NEG_Y: Self = Self::NEG_Y;
			const NEG_Z: Self = Self::NEG_Z;
			const NEG_W: Self = Self::NEG_W;
		}
	};
}
pub(crate) use impl_signedvec4;

impl_signedvec4!(I8Vec4);
impl_signedvec4!(I16Vec4);
impl_signedvec4!(IVec4);
impl_signedvec4!(I64Vec4);
impl_signedvec4!(Vec4);
impl_signedvec4!(DVec4);

/**
Vector of any length whose elements are a floating-point type.
*/
pub trait FloatVec: SignedVec {
	const NAN: Self;
	const INFINITY: Self;
	const NEG_INFINITY: Self;
	fn copysign(self, rhs: Self) -> Self;
	fn is_finite(self) -> bool;
	fn is_finite_mask(self) -> Self::BVec;
	fn is_nan(self) -> bool;
	fn is_nan_mask(self) -> Self::BVec;
	fn length(self) -> Self::Scalar;
	fn length_recip(self) -> Self::Scalar;
	fn distance(self, rhs: Self) -> Self::Scalar;
	fn normalize(self) -> Self;
	fn try_normalize(self) -> Option<Self>;
	fn normalize_or(self, fallback: Self) -> Self;
	fn normalize_or_zero(self) -> Self;
	fn normalize_and_length(self) -> (Self, Self::Scalar);
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
	fn fract_gl(self) -> Self;
	fn exp(self) -> Self;
	fn powf(self, n: Self::Scalar) -> Self;
	fn recip(self) -> Self;
	fn lerp(self, rhs: Self, s: Self::Scalar) -> Self;
	fn move_towards(&self, rhs: Self, d: Self::Scalar) -> Self;
	fn midpoint(self, rhs: Self) -> Self;
	fn abs_diff_eq(self, rhs: Self, max_abs_diff: Self::Scalar) -> bool;
	fn clamp_length(self, min: Self::Scalar, max: Self::Scalar) -> Self;
	fn clamp_length_max(self, max: Self::Scalar) -> Self;
	fn clamp_length_min(self, min: Self::Scalar) -> Self;
	fn mul_add(self, a: Self, b: Self) -> Self;
	fn reflect(self, normal: Self) -> Self;
	fn refract(self, normal: Self, eta: Self::Scalar) -> Self;
}

macro_rules! impl_floatvec {
	($type:ty) => {
		impl FloatVec for $type {
			const NAN: Self = Self::NAN;
			const INFINITY: Self = Self::INFINITY;
			const NEG_INFINITY: Self = Self::NEG_INFINITY;
			fn copysign(self, rhs: Self) -> Self { self.copysign(rhs) }
			fn is_finite(self) -> bool { self.is_finite() }
			fn is_finite_mask(self) -> Self::BVec { self.is_finite_mask() }
			fn is_nan(self) -> bool { self.is_nan() }
			fn is_nan_mask(self) -> Self::BVec { self.is_nan_mask() }
			fn length(self) -> Self::Scalar { self.length() }
			fn length_recip(self) -> Self::Scalar { self.length_recip() }
			fn distance(self, rhs: Self) -> Self::Scalar { self.distance(rhs) }
			fn normalize(self) -> Self { self.normalize() }
			fn try_normalize(self) -> Option<Self> { self.try_normalize() }
			fn normalize_or(self, fallback: Self) -> Self { self.normalize_or(fallback) }
			fn normalize_or_zero(self) -> Self { self.normalize_or_zero() }
			fn normalize_and_length(self) -> (Self, Self::Scalar) { self.normalize_and_length() }
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
			fn fract_gl(self) -> Self { self.fract_gl() }
			fn exp(self) -> Self { self.exp() }
			fn powf(self, n: Self::Scalar) -> Self { self.powf(n) }
			fn recip(self) -> Self { self.recip() }
			fn lerp(self, rhs: Self, s: Self::Scalar) -> Self { self.lerp(rhs, s) }
			fn move_towards(&self, rhs: Self, d: Self::Scalar) -> Self { self.move_towards(rhs, d) }
			fn midpoint(self, rhs: Self) -> Self { self.midpoint(rhs) }
			fn abs_diff_eq(self, rhs: Self, max_abs_diff: Self::Scalar) -> bool { self.abs_diff_eq(rhs, max_abs_diff) }
			fn clamp_length(self, min: Self::Scalar, max: Self::Scalar) -> Self { self.clamp_length(min, max) }
			fn clamp_length_max(self, max: Self::Scalar) -> Self { self.clamp_length_max(max) }
			fn clamp_length_min(self, min: Self::Scalar) -> Self { self.clamp_length_min(min) }
			fn mul_add(self, a: Self, b: Self) -> Self { self.mul_add(a, b) }
			fn reflect(self, normal: Self) -> Self { self.reflect(normal) }
			fn refract(self, normal: Self, eta: Self::Scalar) -> Self { self.refract(normal, eta) }
		}
	};
}

impl_floatvec!(Vec2);
impl_floatvec!(Vec3);
impl_floatvec!(Vec3A);
impl_floatvec!(Vec4);
impl_floatvec!(DVec2);
impl_floatvec!(DVec3);
impl_floatvec!(DVec4);

/**
Vector of length 2 whose elements are a floating-point type.
*/
pub trait FloatVec2: FloatVec + SignedVec2 {
	fn angle_to(self, rhs: Self) -> Self::Scalar;
	fn from_angle(angle: Self::Scalar) -> Self;
	fn to_angle(self) -> Self::Scalar;
	fn rotate_towards(&self, rhs: Self, max_angle: Self::Scalar) -> Self;
}

macro_rules! impl_floatvec2 {
	($type:ty) => {
		impl FloatVec2 for $type {
			fn angle_to(self, rhs: Self) -> Self::Scalar { self.angle_to(rhs) }
			fn from_angle(angle: Self::Scalar) -> Self { Self::from_angle(angle) }
			fn to_angle(self) -> Self::Scalar { self.to_angle() }
			fn rotate_towards(&self, rhs: Self, max_angle: Self::Scalar) -> Self { self.rotate_towards(rhs, max_angle) }
		}
	};
}

impl_floatvec2!(Vec2);
impl_floatvec2!(DVec2);

/**
Vector of length 3 whose elements are a floating-point type.
*/
pub trait FloatVec3: FloatVec + SignedVec3 {
	fn angle_between(self, rhs: Self) -> Self::Scalar;
	fn any_orthogonal_vector(&self) -> Self;
	fn any_orthonormal_vector(&self) -> Self;
	fn any_orthonormal_pair(&self) -> (Self, Self);
	fn rotate_towards(self, rhs: Self, max_angle: Self::Scalar) -> Self;
	fn slerp(self, rhs: Self, s: Self::Scalar) -> Self;
}

macro_rules! impl_floatvec3 {
	($type:ty) => {
		impl FloatVec3 for $type {
			fn angle_between(self, rhs: Self) -> Self::Scalar { self.angle_between(rhs) }
			fn any_orthogonal_vector(&self) -> Self { self.any_orthogonal_vector() }
			fn any_orthonormal_vector(&self) -> Self { self.any_orthonormal_vector() }
			fn any_orthonormal_pair(&self) -> (Self, Self) { self.any_orthonormal_pair() }
			fn rotate_towards(self, rhs: Self, max_angle: Self::Scalar) -> Self { self.rotate_towards(rhs, max_angle) }
			fn slerp(self, rhs: Self, s: Self::Scalar) -> Self { self.slerp(rhs, s) }
		}
	};
}

impl_floatvec3!(Vec3);
impl_floatvec3!(Vec3A);
impl_floatvec3!(DVec3);

/**
Vector of length 4 whose elements are a floating-point type.
*/
pub trait FloatVec4: FloatVec + SignedVec4 {}

impl FloatVec4 for Vec4 {}
impl FloatVec4 for DVec4 {}

/**
Vector of any length whose elements are an integer type.
*/
pub trait IntVec
where
	Self:
		GVec +
		Eq +
		Not<Output = Self> +
		BitAnd<Output = Self> +
		BitAnd<Self::Scalar, Output = Self> +
		BitOr<Output = Self> +
		BitOr<Self::Scalar, Output = Self> +
		BitXor<Output = Self> +
		BitXor<Self::Scalar, Output = Self> +
		Shl<i8, Output = Self> +
		Shr<i8, Output = Self> +
		Shl<i16, Output = Self> +
		Shr<i16, Output = Self> +
		Shl<i32, Output = Self> +
		Shr<i32, Output = Self> +
		Shl<i64, Output = Self> +
		Shr<i64, Output = Self> +
		Shl<u8, Output = Self> +
		Shr<u8, Output = Self> +
		Shl<u16, Output = Self> +
		Shr<u16, Output = Self> +
		Shl<u32, Output = Self> +
		Shr<u32, Output = Self> +
		Shl<u64, Output = Self> +
		Shr<u64, Output = Self> +
	,
{
	type UnsignedScalar;
	fn checked_add(self, rhs: Self) -> Option<Self>;
	fn checked_sub(self, rhs: Self) -> Option<Self>;
	fn checked_mul(self, rhs: Self) -> Option<Self>;
	fn checked_div(self, rhs: Self) -> Option<Self>;
	fn wrapping_add(self, rhs: Self) -> Self;
	fn wrapping_sub(self, rhs: Self) -> Self;
	fn wrapping_mul(self, rhs: Self) -> Self;
	fn wrapping_div(self, rhs: Self) -> Self;
	fn saturating_add(self, rhs: Self) -> Self;
	fn saturating_sub(self, rhs: Self) -> Self;
	fn saturating_mul(self, rhs: Self) -> Self;
	fn saturating_div(self, rhs: Self) -> Self;
	fn manhattan_distance(self, rhs: Self) -> Self::UnsignedScalar;
	fn checked_manhattan_distance(self, rhs: Self) -> Option<Self::UnsignedScalar>;
	fn chebyshev_distance(self, rhs: Self) -> Self::UnsignedScalar;
}

macro_rules! impl_intvec {
	($type:ty, $unsigned:ty) => {
		impl IntVec for $type {
			type UnsignedScalar = $unsigned;
			fn checked_add(self, rhs: Self) -> Option<Self> { self.checked_add(rhs) }
			fn checked_sub(self, rhs: Self) -> Option<Self> { self.checked_sub(rhs) }
			fn checked_mul(self, rhs: Self) -> Option<Self> { self.checked_mul(rhs) }
			fn checked_div(self, rhs: Self) -> Option<Self> { self.checked_div(rhs) }
			fn wrapping_add(self, rhs: Self) -> Self { self.wrapping_add(rhs) }
			fn wrapping_sub(self, rhs: Self) -> Self { self.wrapping_sub(rhs) }
			fn wrapping_mul(self, rhs: Self) -> Self { self.wrapping_mul(rhs) }
			fn wrapping_div(self, rhs: Self) -> Self { self.wrapping_div(rhs) }
			fn saturating_add(self, rhs: Self) -> Self { self.saturating_add(rhs) }
			fn saturating_sub(self, rhs: Self) -> Self { self.saturating_sub(rhs) }
			fn saturating_mul(self, rhs: Self) -> Self { self.saturating_mul(rhs) }
			fn saturating_div(self, rhs: Self) -> Self { self.saturating_div(rhs) }
			fn manhattan_distance(self, rhs: Self) -> Self::UnsignedScalar { self.manhattan_distance(rhs) }
			fn checked_manhattan_distance(self, rhs: Self) -> Option<Self::UnsignedScalar> { self.checked_manhattan_distance(rhs) }
			fn chebyshev_distance(self, rhs: Self) -> Self::UnsignedScalar { self.chebyshev_distance(rhs) }
		}
	};
}
pub(crate) use impl_intvec;

impl_intvec!(I8Vec2, u8);
impl_intvec!(I8Vec3, u8);
impl_intvec!(I8Vec4, u8);
impl_intvec!(U8Vec2, u8);
impl_intvec!(U8Vec3, u8);
impl_intvec!(U8Vec4, u8);
impl_intvec!(I16Vec2, u16);
impl_intvec!(I16Vec3, u16);
impl_intvec!(I16Vec4, u16);
impl_intvec!(U16Vec2, u16);
impl_intvec!(U16Vec3, u16);
impl_intvec!(U16Vec4, u16);
impl_intvec!(IVec2, u32);
impl_intvec!(IVec3, u32);
impl_intvec!(IVec4, u32);
impl_intvec!(UVec2, u32);
impl_intvec!(UVec3, u32);
impl_intvec!(UVec4, u32);
impl_intvec!(I64Vec2, u64);
impl_intvec!(I64Vec3, u64);
impl_intvec!(I64Vec4, u64);
impl_intvec!(U64Vec2, u64);
impl_intvec!(U64Vec3, u64);
impl_intvec!(U64Vec4, u64);
impl_intvec!(USizeVec2, usize);
impl_intvec!(USizeVec3, usize);
impl_intvec!(USizeVec4, usize);

/**
Vector of length 2 whose elements are an integer type.
*/
pub trait IntVec2
where
	Self:
		IntVec +
		GVec2 +
		Shl<IVec2, Output = Self> +
		Shr<IVec2, Output = Self> +
		Shl<UVec2, Output = Self> +
		Shr<UVec2, Output = Self> +
	,
{}

impl IntVec2 for I8Vec2 {}
impl IntVec2 for U8Vec2 {}
impl IntVec2 for I16Vec2 {}
impl IntVec2 for U16Vec2 {}
impl IntVec2 for IVec2 {}
impl IntVec2 for UVec2 {}
impl IntVec2 for I64Vec2 {}
impl IntVec2 for U64Vec2 {}
impl IntVec2 for USizeVec2 {}

/**
Vector of length 3 whose elements are an integer type.
*/
pub trait IntVec3
where
	Self:
		IntVec +
		GVec3<BVec = BVec3> +
		Shl<IVec3, Output = Self> +
		Shr<IVec3, Output = Self> +
		Shl<UVec3, Output = Self> +
		Shr<UVec3, Output = Self> +
	,
{}

impl IntVec3 for I8Vec3 {}
impl IntVec3 for U8Vec3 {}
impl IntVec3 for I16Vec3 {}
impl IntVec3 for U16Vec3 {}
impl IntVec3 for IVec3 {}
impl IntVec3 for UVec3 {}
impl IntVec3 for I64Vec3 {}
impl IntVec3 for U64Vec3 {}
impl IntVec3 for USizeVec3 {}

/**
Vector of length 4 whose elements are an integer type.
*/
pub trait IntVec4
where
	Self:
		IntVec +
		GVec4<BVec = BVec4> +
		Shl<IVec4, Output = Self> +
		Shr<IVec4, Output = Self> +
		Shl<UVec4, Output = Self> +
		Shr<UVec4, Output = Self> +
	,
{}

impl IntVec4 for I8Vec4 {}
impl IntVec4 for U8Vec4 {}
impl IntVec4 for I16Vec4 {}
impl IntVec4 for U16Vec4 {}
impl IntVec4 for IVec4 {}
impl IntVec4 for UVec4 {}
impl IntVec4 for I64Vec4 {}
impl IntVec4 for U64Vec4 {}
impl IntVec4 for USizeVec4 {}

/**
Vector of any length whose elements are a signed integer type.
*/
pub trait SIntVec: IntVec + SignedVec {}

impl SIntVec for I8Vec2 {}
impl SIntVec for I8Vec3 {}
impl SIntVec for I8Vec4 {}
impl SIntVec for I16Vec2 {}
impl SIntVec for I16Vec3 {}
impl SIntVec for I16Vec4 {}
impl SIntVec for IVec2 {}
impl SIntVec for IVec3 {}
impl SIntVec for IVec4 {}
impl SIntVec for I64Vec2 {}
impl SIntVec for I64Vec3 {}
impl SIntVec for I64Vec4 {}

/**
Vector of length 2 whose elements are a signed integer type.
*/
pub trait SIntVec2: SIntVec + IntVec2 {}

impl SIntVec2 for I8Vec2 {}
impl SIntVec2 for I16Vec2 {}
impl SIntVec2 for IVec2 {}
impl SIntVec2 for I64Vec2 {}

/**
Vector of length 3 whose elements are a signed integer type.
*/
pub trait SIntVec3: SIntVec + IntVec3 {}

impl SIntVec3 for I8Vec3 {}
impl SIntVec3 for I16Vec3 {}
impl SIntVec3 for IVec3 {}
impl SIntVec3 for I64Vec3 {}

/**
Vector of length 4 whose elements are a signed integer type.
*/
pub trait SIntVec4: SIntVec + IntVec4 {}

impl SIntVec4 for I8Vec4 {}
impl SIntVec4 for I16Vec4 {}
impl SIntVec4 for IVec4 {}
impl SIntVec4 for I64Vec4 {}

/**
Vector of any length whose elements are an unsigned integer type.
*/
pub trait UIntVec: IntVec {}

impl UIntVec for U8Vec2 {}
impl UIntVec for U8Vec3 {}
impl UIntVec for U8Vec4 {}
impl UIntVec for U16Vec2 {}
impl UIntVec for U16Vec3 {}
impl UIntVec for U16Vec4 {}
impl UIntVec for UVec2 {}
impl UIntVec for UVec3 {}
impl UIntVec for UVec4 {}
impl UIntVec for U64Vec2 {}
impl UIntVec for U64Vec3 {}
impl UIntVec for U64Vec4 {}
impl UIntVec for USizeVec2 {}
impl UIntVec for USizeVec3 {}
impl UIntVec for USizeVec4 {}

/**
Vector of length 2 whose elements are an unsigned integer type.
*/
pub trait UIntVec2: UIntVec + IntVec2 {}

impl UIntVec2 for U8Vec2 {}
impl UIntVec2 for U16Vec2 {}
impl UIntVec2 for UVec2 {}
impl UIntVec2 for U64Vec2 {}
impl UIntVec2 for USizeVec2 {}

/**
Vector of length 3 whose elements are an unsigned integer type.
*/
pub trait UIntVec3: UIntVec + IntVec3 {}

impl UIntVec3 for U8Vec3 {}
impl UIntVec3 for U16Vec3 {}
impl UIntVec3 for UVec3 {}
impl UIntVec3 for U64Vec3 {}
impl UIntVec3 for USizeVec3 {}

/**
Vector of length 4 whose elements are an unsigned integer type.
*/
pub trait UIntVec4: UIntVec + IntVec4 {}

impl UIntVec4 for U8Vec4 {}
impl UIntVec4 for U16Vec4 {}
impl UIntVec4 for UVec4 {}
impl UIntVec4 for U64Vec4 {}
impl UIntVec4 for USizeVec4 {}

/**
Vector of any length whose elements are [`i8`].
*/
pub trait I8Vec: SIntVec<Scalar = i8> {}

impl I8Vec for I8Vec2 {}
impl I8Vec for I8Vec3 {}
impl I8Vec for I8Vec4 {}

/**
Vector of any length whose elements are [`u8`].
*/
pub trait U8Vec: UIntVec<Scalar = u8> {}

impl U8Vec for U8Vec2 {}
impl U8Vec for U8Vec3 {}
impl U8Vec for U8Vec4 {}

/**
Vector of any length whose elements are [`i16`].
*/
pub trait I16Vec: SIntVec<Scalar = i16> {}

impl I16Vec for I16Vec2 {}
impl I16Vec for I16Vec3 {}
impl I16Vec for I16Vec4 {}

/**
Vector of any length whose elements are [`u16`].
*/
pub trait U16Vec: UIntVec<Scalar = u16> {}

impl U16Vec for U16Vec2 {}
impl U16Vec for U16Vec3 {}
impl U16Vec for U16Vec4 {}

/**
Vector of any length whose elements are [`i32`].
*/
pub trait I32Vec: SIntVec<Scalar = i32> {}

impl I32Vec for IVec2 {}
impl I32Vec for IVec3 {}
impl I32Vec for IVec4 {}

/**
Vector of any length whose elements are [`u32`].
*/
pub trait U32Vec: UIntVec<Scalar = u32> {}

impl U32Vec for UVec2 {}
impl U32Vec for UVec3 {}
impl U32Vec for UVec4 {}

/**
Vector of any length whose elements are [`i64`].
*/
pub trait I64Vec: SIntVec<Scalar = i64> {}

impl I64Vec for I64Vec2 {}
impl I64Vec for I64Vec3 {}
impl I64Vec for I64Vec4 {}

/**
Vector of any length whose elements are [`u64`].
*/
pub trait U64Vec: UIntVec<Scalar = u64> {}

impl U64Vec for U64Vec2 {}
impl U64Vec for U64Vec3 {}
impl U64Vec for U64Vec4 {}

/**
Vector of any length whose elements are [`usize`].
*/
pub trait USizeVec: UIntVec<Scalar = usize> {}

impl USizeVec for USizeVec2 {}
impl USizeVec for USizeVec3 {}
impl USizeVec for USizeVec4 {}

/**
Vector of any length whose elements are [`f32`].
*/
pub trait F32Vec: FloatVec<Scalar = f32> {}

impl F32Vec for Vec2 {}
impl F32Vec for Vec3 {}
impl F32Vec for Vec3A {}
impl F32Vec for Vec4 {}

/**
Vector of any length whose elements are [`f64`].
*/
pub trait F64Vec: FloatVec<Scalar = f64> {}

impl F64Vec for DVec2 {}
impl F64Vec for DVec3 {}
impl F64Vec for DVec4 {}
