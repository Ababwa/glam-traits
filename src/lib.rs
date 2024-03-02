/*!
This crate provides traits for the vectors in [glam](https://docs.rs/glam).
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

[`I16Vec`], [`U16Vec`], [`I32Vec`], [`U32Vec`], [`I64Vec`], [`U64Vec`], [`F32Vec`], [`F64Vec`]

[`GBVec`] is also provided to cover boolean vectors.

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
	I16Vec2, I16Vec3, I16Vec4,
	U16Vec2, U16Vec3, U16Vec4,
	IVec2, IVec3, IVec4,
	UVec2, UVec3, UVec4,
	I64Vec2, I64Vec3, I64Vec4,
	U64Vec2, U64Vec3, U64Vec4,
	Vec2, Vec3, Vec3A, Vec4,
	DVec2, DVec3, DVec4,
};

/**
Boolean vector of any length. Behavior common to all `glam` boolean vectors is included in this
trait.
*/
pub trait GBVec
where
	Self:
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
	,
{
	const FALSE: Self;
	const TRUE: Self;
	const DIM: usize;
	fn splat(v: bool) -> Self;
	fn bitmask(self) -> u32;
	fn any(self) -> bool;
	fn all(self) -> bool;
	fn test(&self, index: usize) -> bool;
	fn set(&mut self, index: usize, value: bool);
}

macro_rules! impl_gbvec {
	($type:ty, $dim:literal) => {
		impl GBVec for $type {
			const FALSE: Self = Self::FALSE;
			const TRUE: Self = Self::TRUE;
			const DIM: usize = $dim;
			fn splat(v: bool) -> Self { Self::splat(v) }
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
Generic vector of any length. Behavior common to all `glam` vectors is included in this trait.
*/
pub trait GVec
where
	for <'a> Self:
		Clone +
		Copy +
		PartialEq +
		Default +
		Display +
		Debug +
		Add<Output = Self> +
		Add<Self::Scalar, Output = Self> +
		AddAssign +
		AddAssign<Self::Scalar> +
		Sub<Output = Self> +
		Sub<Self::Scalar, Output = Self> +
		SubAssign +
		SubAssign<Self::Scalar> +
		Mul<Output = Self> +
		Mul<Self::Scalar, Output = Self> +
		MulAssign +
		MulAssign<Self::Scalar> +
		Div<Output = Self> +
		Div<Self::Scalar, Output = Self> +
		DivAssign +
		DivAssign<Self::Scalar> +
		Rem<Output = Self> +
		Rem<Self::Scalar, Output = Self> +
		RemAssign +
		RemAssign<Self::Scalar> +
		Sum +
		Sum<&'a Self> +
		Product +
		Product<&'a Self> +
		Index<usize, Output = Self::Scalar> +
		IndexMut<usize, Output = Self::Scalar> +
	,
	Self::Scalar:
		'static +
		Copy +
		PartialOrd +
		Add<Self, Output = Self> +
		Sub<Self, Output = Self> +
		Mul<Self, Output = Self> +
		Div<Self, Output = Self> +
		Rem<Self, Output = Self> +
	,
	Self::BVecType: GBVec,
{
	type Scalar;
	type BVecType;
	const ZERO: Self;
	const ONE: Self;
	const MIN: Self;
	const MAX: Self;
	const DIM: usize;
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
	fn select(mask: Self::BVecType, if_true: Self, if_false: Self) -> Self;
	fn cmpeq(self, rhs: Self) -> Self::BVecType;
	fn cmpne(self, rhs: Self) -> Self::BVecType;
	fn cmpge(self, rhs: Self) -> Self::BVecType;
	fn cmpgt(self, rhs: Self) -> Self::BVecType;
	fn cmple(self, rhs: Self) -> Self::BVecType;
	fn cmplt(self, rhs: Self) -> Self::BVecType;
	fn length_squared(self) -> Self::Scalar;
}

macro_rules! impl_gvec {
	($type:ty, $scalar:ty, $bvec:ty, $dim:literal) => {
		impl GVec for $type {
			type Scalar = $scalar;
			type BVecType = $bvec;
			const ZERO: Self = Self::ZERO;
			const ONE: Self = Self::ONE;
			const MIN: Self = Self::MIN;
			const MAX: Self = Self::MAX;
			const DIM: usize = $dim;
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
			fn select(mask: Self::BVecType, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
			fn cmpeq(self, rhs: Self) -> Self::BVecType { self.cmpeq(rhs) }
			fn cmpne(self, rhs: Self) -> Self::BVecType { self.cmpne(rhs) }
			fn cmpge(self, rhs: Self) -> Self::BVecType { self.cmpge(rhs) }
			fn cmpgt(self, rhs: Self) -> Self::BVecType { self.cmpgt(rhs) }
			fn cmple(self, rhs: Self) -> Self::BVecType { self.cmple(rhs) }
			fn cmplt(self, rhs: Self) -> Self::BVecType { self.cmplt(rhs) }
			fn length_squared(self) -> Self::Scalar { self.length_squared() }
		}
	};
}

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
impl_gvec!(Vec2, f32, BVec2, 2);
impl_gvec!(Vec3, f32, BVec3, 3);
impl_gvec!(Vec3A, f32, BVec3A, 3);
impl_gvec!(Vec4, f32, BVec4A, 4);
impl_gvec!(DVec2, f64, BVec2, 2);
impl_gvec!(DVec3, f64, BVec3, 3);
impl_gvec!(DVec4, f64, BVec4, 4);

/**
Generic vector of length 2. Behavior common to all `glam` vectors of length 2 is included in this
trait.
*/
pub trait GVec2
where
	Self:
		GVec<BVecType = BVec2> +
		AsRef<[Self::Scalar; 2]> +
		AsMut<[Self::Scalar; 2]> +
		From<[Self::Scalar; 2]> +
		Into<[Self::Scalar; 2]> +
		From<(Self::Scalar, Self::Scalar)> +
		Into<(Self::Scalar, Self::Scalar)> +
	,
{
	const X: Self;
	const Y: Self;
	const AXES: [Self; 2];
	fn x(self) -> Self::Scalar;
	fn y(self) -> Self::Scalar;
	fn new(x: Self::Scalar, y: Self::Scalar) -> Self;
	fn from_array(a: [Self::Scalar; 2]) -> Self;
	fn to_array(&self) -> [Self::Scalar; 2];
}

macro_rules! impl_gvec2 {
	($type:ty) => {
		impl GVec2 for $type {
			const X: Self = Self::X;
			const Y: Self = Self::Y;
			const AXES: [Self; 2] = Self::AXES;
			fn x(self) -> Self::Scalar { self.x }
			fn y(self) -> Self::Scalar { self.y }
			fn new(x: Self::Scalar, y: Self::Scalar) -> Self { Self::new(x, y) }
			fn from_array(a: [Self::Scalar; 2]) -> Self { Self::from_array(a) }
			fn to_array(&self) -> [Self::Scalar; 2] { self.to_array() }
		}
	};
}

impl_gvec2!(I16Vec2);
impl_gvec2!(U16Vec2);
impl_gvec2!(IVec2);
impl_gvec2!(UVec2);
impl_gvec2!(I64Vec2);
impl_gvec2!(U64Vec2);
impl_gvec2!(Vec2);
impl_gvec2!(DVec2);

/**
Generic vector of length 3. Behavior common to all `glam` vectors of length 3 is included in this trait.
*/
pub trait GVec3
where
	Self:
		GVec +
		AsRef<[Self::Scalar; 3]> +
		AsMut<[Self::Scalar; 3]> +
		From<[Self::Scalar; 3]> +
		Into<[Self::Scalar; 3]> +
		From<(Self::Scalar, Self::Scalar, Self::Scalar)> +
		Into<(Self::Scalar, Self::Scalar, Self::Scalar)> +
	,
{
	const X: Self;
	const Y: Self;
	const Z: Self;
	const AXES: [Self; 3];
	fn x(self) -> Self::Scalar;
	fn y(self) -> Self::Scalar;
	fn z(self) -> Self::Scalar;
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar) -> Self;
	fn from_array(a: [Self::Scalar; 3]) -> Self;
	fn to_array(&self) -> [Self::Scalar; 3];
	fn cross(self, rhs: Self) -> Self;
}

macro_rules! impl_gvec3 {
	($type:ty) => {
		impl GVec3 for $type {
			const X: Self = Self::X;
			const Y: Self = Self::Y;
			const Z: Self = Self::Z;
			const AXES: [Self; 3] = Self::AXES;
			fn x(self) -> Self::Scalar { self.x }
			fn y(self) -> Self::Scalar { self.y }
			fn z(self) -> Self::Scalar { self.z }
			fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar) -> Self { Self::new(x, y, z) }
			fn from_array(a: [Self::Scalar; 3]) -> Self { Self::from_array(a) }
			fn to_array(&self) -> [Self::Scalar; 3] { self.to_array() }
			fn cross(self, rhs: Self) -> Self { self.cross(rhs) }
		}
	};
}

impl_gvec3!(I16Vec3);
impl_gvec3!(U16Vec3);
impl_gvec3!(IVec3);
impl_gvec3!(UVec3);
impl_gvec3!(I64Vec3);
impl_gvec3!(U64Vec3);
impl_gvec3!(Vec3);
impl_gvec3!(Vec3A);
impl_gvec3!(DVec3);

/**
Generic vector of length 4. Behavior common to all `glam` vectors of length 4 is included in this
trait.
*/
pub trait GVec4
where
	Self:
		GVec +
		AsRef<[Self::Scalar; 4]> +
		AsMut<[Self::Scalar; 4]> +
		From<[Self::Scalar; 4]> +
		Into<[Self::Scalar; 4]> +
		From<(Self::Scalar, Self::Scalar, Self::Scalar, Self::Scalar)> +
		Into<(Self::Scalar, Self::Scalar, Self::Scalar, Self::Scalar)> +
	,
{
	const X: Self;
	const Y: Self;
	const Z: Self;
	const W: Self;
	const AXES: [Self; 4];
	fn x(self) -> Self::Scalar;
	fn y(self) -> Self::Scalar;
	fn z(self) -> Self::Scalar;
	fn w(self) -> Self::Scalar;
	fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar, w: Self::Scalar) -> Self;
	fn from_array(a: [Self::Scalar; 4]) -> Self;
	fn to_array(&self) -> [Self::Scalar; 4];
}

macro_rules! impl_gvec4 {
	($type:ty) => {
		impl GVec4 for $type {
			const X: Self = Self::X;
			const Y: Self = Self::Y;
			const Z: Self = Self::Z;
			const W: Self = Self::W;
			const AXES: [Self; 4] = Self::AXES;
			fn x(self) -> Self::Scalar { self.x }
			fn y(self) -> Self::Scalar { self.y }
			fn z(self) -> Self::Scalar { self.z }
			fn w(self) -> Self::Scalar { self.w }
			fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar, w: Self::Scalar) -> Self { Self::new(x, y, z, w) }
			fn from_array(a: [Self::Scalar; 4]) -> Self { Self::from_array(a) }
			fn to_array(&self) -> [Self::Scalar; 4] { self.to_array() }
		}
	};
}

impl_gvec4!(I16Vec4);
impl_gvec4!(U16Vec4);
impl_gvec4!(IVec4);
impl_gvec4!(UVec4);
impl_gvec4!(I64Vec4);
impl_gvec4!(U64Vec4);
impl_gvec4!(Vec4);
impl_gvec4!(DVec4);

/**
Vector of any length whose elements are a signed type. Behavior common to all `glam` vectors of
signed types is included in this trait.
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

impl_signedvec!(I16Vec2);
impl_signedvec!(I16Vec3);
impl_signedvec!(I16Vec4);
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
Vector of length 2 whose elements are a signed type. Behavior common to all `glam` vectors of
length 2 of signed types is included in this trait.
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

impl_signedvec2!(I16Vec2);
impl_signedvec2!(IVec2);
impl_signedvec2!(I64Vec2);
impl_signedvec2!(Vec2);
impl_signedvec2!(DVec2);

/**
Vector of length 3 whose elements are a signed type. Behavior common to all `glam` vectors of
length 3 of signed types is included in this trait.
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

impl_signedvec3!(I16Vec3);
impl_signedvec3!(IVec3);
impl_signedvec3!(I64Vec3);
impl_signedvec3!(Vec3);
impl_signedvec3!(Vec3A);
impl_signedvec3!(DVec3);

/**
Vector of length 4 whose elements are a signed type. Behavior common to all `glam` vectors of
length 4 of signed types is included in this trait.
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

impl_signedvec4!(I16Vec4);
impl_signedvec4!(IVec4);
impl_signedvec4!(I64Vec4);
impl_signedvec4!(Vec4);
impl_signedvec4!(DVec4);

/**
Vector of any length whose elements are a floating-point type. Behavior common to all `glam`
vectors of floating-point types is included in this trait.
*/
pub trait FloatVec: SignedVec {
	const NAN: Self;
	const INFINITY: Self;
	const NEG_INFINITY: Self;
	fn copysign(self, rhs: Self) -> Self;
	fn is_finite(self) -> bool;
	fn is_nan(self) -> bool;
	fn is_nan_mask(self) -> Self::BVecType;
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
	fn abs_diff_eq(self, rhs: Self, max_abs_diff: Self::Scalar) -> bool;
	fn clamp_length(self, min: Self::Scalar, max: Self::Scalar) -> Self;
	fn clamp_length_max(self, max: Self::Scalar) -> Self;
	fn clamp_length_min(self, min: Self::Scalar) -> Self;
	fn mul_add(self, a: Self, b: Self) -> Self;
}

macro_rules! impl_floatvec {
	($type:ty) => {
		impl FloatVec for $type {
			const NAN: Self = Self::NAN;
			const INFINITY: Self = Self::INFINITY;
			const NEG_INFINITY: Self = Self::NEG_INFINITY;
			fn copysign(self, rhs: Self) -> Self { self.copysign(rhs) }
			fn is_finite(self) -> bool { self.is_finite() }
			fn is_nan(self) -> bool { self.is_nan() }
			fn is_nan_mask(self) -> Self::BVecType { self.is_nan_mask() }
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
			fn abs_diff_eq(self, rhs: Self, max_abs_diff: Self::Scalar) -> bool { self.abs_diff_eq(rhs, max_abs_diff) }
			fn clamp_length(self, min: Self::Scalar, max: Self::Scalar) -> Self { self.clamp_length(min, max) }
			fn clamp_length_max(self, max: Self::Scalar) -> Self { self.clamp_length_max(max) }
			fn clamp_length_min(self, min: Self::Scalar) -> Self { self.clamp_length_min(min) }
			fn mul_add(self, a: Self, b: Self) -> Self { self.mul_add(a, b) }
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
Vector of length 2 whose elements are a floating-point type. Behavior common to all `glam` vectors
of length 2 of floating-point types is included in this trait.
*/
pub trait FloatVec2: FloatVec + SignedVec2 {
	fn angle_between(self, rhs: Self) -> Self::Scalar;
	fn from_angle(angle: Self::Scalar) -> Self;
	fn to_angle(self) -> Self::Scalar;
}

macro_rules! impl_floatvec2 {
	($type:ty) => {
		impl FloatVec2 for $type {
			fn angle_between(self, rhs: Self) -> Self::Scalar { self.angle_between(rhs) }
			fn from_angle(angle: Self::Scalar) -> Self { Self::from_angle(angle) }
			fn to_angle(self) -> Self::Scalar { self.to_angle() }
		}
	};
}

impl_floatvec2!(Vec2);
impl_floatvec2!(DVec2);

/**
Vector of length 3 whose elements are a floating-point type. Behavior common to all `glam` vectors
of length 3 of floating-point types is included in this trait.
*/
pub trait FloatVec3: FloatVec + SignedVec3 {
	fn angle_between(self, rhs: Self) -> Self::Scalar;
	fn any_orthogonal_vector(&self) -> Self;
	fn any_orthonormal_vector(&self) -> Self;
	fn any_orthonormal_pair(&self) -> (Self, Self);
}

macro_rules! impl_floatvec3 {
	($type:ty) => {
		impl FloatVec3 for $type {
			fn angle_between(self, rhs: Self) -> Self::Scalar { self.angle_between(rhs) }
			fn any_orthogonal_vector(&self) -> Self { self.any_orthogonal_vector() }
			fn any_orthonormal_vector(&self) -> Self { self.any_orthonormal_vector() }
			fn any_orthonormal_pair(&self) -> (Self, Self) { self.any_orthonormal_pair() }
		}
	};
}

impl_floatvec3!(Vec3);
impl_floatvec3!(Vec3A);
impl_floatvec3!(DVec3);

/**
Vector of length 4 whose elements are a floating-point type. This is a marker trait as there is no
behavior specific to `glam` vectors of length 4 of floating-point types.
*/
pub trait FloatVec4: FloatVec + SignedVec4 {}

impl FloatVec4 for Vec4 {}
impl FloatVec4 for DVec4 {}

/**
Vector of any length whose elements are an integer type. Behavior common to all `glam` vectors of
integer types is included in this type.
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
	fn wrapping_add(self, rhs: Self) -> Self;
	fn wrapping_sub(self, rhs: Self) -> Self;
	fn wrapping_mul(self, rhs: Self) -> Self;
	fn wrapping_div(self, rhs: Self) -> Self;
	fn saturating_add(self, rhs: Self) -> Self;
	fn saturating_sub(self, rhs: Self) -> Self;
	fn saturating_mul(self, rhs: Self) -> Self;
	fn saturating_div(self, rhs: Self) -> Self;
}

macro_rules! impl_intvec {
	($type:ty) => {
		impl IntVec for $type {
			fn wrapping_add(self, rhs: Self) -> Self { self.wrapping_add(rhs) }
			fn wrapping_sub(self, rhs: Self) -> Self { self.wrapping_sub(rhs) }
			fn wrapping_mul(self, rhs: Self) -> Self { self.wrapping_mul(rhs) }
			fn wrapping_div(self, rhs: Self) -> Self { self.wrapping_div(rhs) }
			fn saturating_add(self, rhs: Self) -> Self { self.saturating_add(rhs) }
			fn saturating_sub(self, rhs: Self) -> Self { self.saturating_sub(rhs) }
			fn saturating_mul(self, rhs: Self) -> Self { self.saturating_mul(rhs) }
			fn saturating_div(self, rhs: Self) -> Self { self.saturating_div(rhs) }
		}
	};
}

impl_intvec!(I16Vec2);
impl_intvec!(I16Vec3);
impl_intvec!(I16Vec4);
impl_intvec!(U16Vec2);
impl_intvec!(U16Vec3);
impl_intvec!(U16Vec4);
impl_intvec!(IVec2);
impl_intvec!(IVec3);
impl_intvec!(IVec4);
impl_intvec!(UVec2);
impl_intvec!(UVec3);
impl_intvec!(UVec4);
impl_intvec!(I64Vec2);
impl_intvec!(I64Vec3);
impl_intvec!(I64Vec4);
impl_intvec!(U64Vec2);
impl_intvec!(U64Vec3);
impl_intvec!(U64Vec4);

/**
Vector of length 2 whose elements are an integer type. This is a marker trait as there is no
behavior specific to `glam` vectors of length 2 of integer types.
*/
pub trait IntVec2: IntVec + GVec2 {}

impl IntVec2 for I16Vec2 {}
impl IntVec2 for U16Vec2 {}
impl IntVec2 for IVec2 {}
impl IntVec2 for UVec2 {}
impl IntVec2 for I64Vec2 {}
impl IntVec2 for U64Vec2 {}

/**
Vector of length 3 whose elements are an integer type. This is a marker trait as there is no
behavior specific to `glam` vectors of length 3 of integer types.
*/
pub trait IntVec3: IntVec + GVec3<BVecType = BVec3> {}

impl IntVec3 for I16Vec3 {}
impl IntVec3 for U16Vec3 {}
impl IntVec3 for IVec3 {}
impl IntVec3 for UVec3 {}
impl IntVec3 for I64Vec3 {}
impl IntVec3 for U64Vec3 {}


/**
Vector of length 4 whose elements are an integer type. This is a marker trait as there is no
behavior specific to `glam` vectors of length 4 of integer types.
*/
pub trait IntVec4: IntVec + GVec4<BVecType = BVec4> {}

impl IntVec4 for I16Vec4 {}
impl IntVec4 for U16Vec4 {}
impl IntVec4 for IVec4 {}
impl IntVec4 for UVec4 {}
impl IntVec4 for I64Vec4 {}
impl IntVec4 for U64Vec4 {}

/**
Vector of any length whose elements are a signed integer type. This is a marker trait as there is
no behavior specific to `glam` vectors of signed integer types.
*/
pub trait SIntVec: IntVec + SignedVec {}

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
Vector of length 2 whose elements are a signed integer type. This is a marker trait as there is no
behavior specific to `glam` vectors of length 2 of signed integer types.
*/
pub trait SIntVec2: SIntVec + IntVec2 {}

impl SIntVec2 for I16Vec2 {}
impl SIntVec2 for IVec2 {}
impl SIntVec2 for I64Vec2 {}

/**
Vector of length 3 whose elements are a signed integer type. This is a marker trait as there is no
behavior specific to `glam` vectors of length 3 of signed integer types.
*/
pub trait SIntVec3: SIntVec + IntVec3 {}

impl SIntVec3 for I16Vec3 {}
impl SIntVec3 for IVec3 {}
impl SIntVec3 for I64Vec3 {}

/**
Vector of length 4 whose elements are a signed integer type. This is a marker trait as there is no
behavior specific to `glam` vectors of length 4 of signed integer types.
*/
pub trait SIntVec4: SIntVec + IntVec4 {}

impl SIntVec4 for I16Vec4 {}
impl SIntVec4 for IVec4 {}
impl SIntVec4 for I64Vec4 {}

/**
Vector of any length whose elements are an unsigned integer type. This is a marker trait as there
is no behavior specific to `glam` vectors of unsigned integer types.
*/
pub trait UIntVec: IntVec {}

impl UIntVec for U16Vec2 {}
impl UIntVec for U16Vec3 {}
impl UIntVec for U16Vec4 {}
impl UIntVec for UVec2 {}
impl UIntVec for UVec3 {}
impl UIntVec for UVec4 {}
impl UIntVec for U64Vec2 {}
impl UIntVec for U64Vec3 {}
impl UIntVec for U64Vec4 {}

/**
Vector of length 2 whose elements are an unsigned integer type. This is a marker trait as there is
no behavior specific to `glam` vectors of length 2 of unsigned integer types.
*/
pub trait UIntVec2: UIntVec + IntVec2 {}

impl UIntVec2 for U16Vec2 {}
impl UIntVec2 for UVec2 {}
impl UIntVec2 for U64Vec2 {}

/**
Vector of length 3 whose elements are an unsigned integer type. This is a marker trait as there is
no behavior specific to `glam` vectors of length 3 of unsigned integer types.
*/
pub trait UIntVec3: UIntVec + IntVec3 {}

impl UIntVec3 for U16Vec3 {}
impl UIntVec3 for UVec3 {}
impl UIntVec3 for U64Vec3 {}

/**
Vector of length 4 whose elements are an unsigned integer type. This is a marker trait as there is
no behavior specific to `glam` vectors of length 4 of unsigned integer types.
*/
pub trait UIntVec4: UIntVec + IntVec4 {}

impl UIntVec4 for U16Vec4 {}
impl UIntVec4 for UVec4 {}
impl UIntVec4 for U64Vec4 {}

/**
Vector of any length whose elements are [`i16`]. This is a marker trait as there is no behavior
specific to `glam` vectors of [`i16`].
*/
pub trait I16Vec: SIntVec<Scalar = i16> {}

impl I16Vec for I16Vec2 {}
impl I16Vec for I16Vec3 {}
impl I16Vec for I16Vec4 {}

/**
Vector of any length whose elements are [`u16`]. This is a marker trait as there is no behavior
specific to `glam` vectors of [`u16`].
*/
pub trait U16Vec: UIntVec<Scalar = u16> {}

impl U16Vec for U16Vec2 {}
impl U16Vec for U16Vec3 {}
impl U16Vec for U16Vec4 {}

/**
Vector of any length whose elements are [`i32`]. This is a marker trait as there is no behavior
specific to `glam` vectors of [`i32`].
*/
pub trait I32Vec: SIntVec<Scalar = i32> {}

impl I32Vec for IVec2 {}
impl I32Vec for IVec3 {}
impl I32Vec for IVec4 {}

/**
Vector of any length whose elements are [`u32`]. This is a marker trait as there is no behavior
specific to `glam` vectors of [`u32`].
*/
pub trait U32Vec: UIntVec<Scalar = u32> {}

impl U32Vec for UVec2 {}
impl U32Vec for UVec3 {}
impl U32Vec for UVec4 {}

/**
Vector of any length whose elements are [`i64`]. This is a marker trait as there is no behavior
specific to `glam` vectors of [`i64`].
*/
pub trait I64Vec: SIntVec<Scalar = i64> {}

impl I64Vec for I64Vec2 {}
impl I64Vec for I64Vec3 {}
impl I64Vec for I64Vec4 {}

/**
Vector of any length whose elements are [`u64`]. This is a marker trait as there is no behavior
specific to `glam` vectors of [`u64`].
*/
pub trait U64Vec: UIntVec<Scalar = u64> {}

impl U64Vec for U64Vec2 {}
impl U64Vec for U64Vec3 {}
impl U64Vec for U64Vec4 {}

/**
Vector of any length whose elements are [`f32`]. This is a marker trait as there is no behavior
specific to `glam` vectors of [`f32`].
*/
pub trait F32Vec: FloatVec<Scalar = f32> {}

impl F32Vec for Vec2 {}
impl F32Vec for Vec3 {}
impl F32Vec for Vec3A {}
impl F32Vec for Vec4 {}

/**
Vector of any length whose elements are [`f64`]. This is a marker trait as there is no behavior
specific to `glam` vectors of [`f64`].
*/
pub trait F64Vec: FloatVec<Scalar = f64> {}

impl F64Vec for DVec2 {}
impl F64Vec for DVec3 {}
impl F64Vec for DVec4 {}
