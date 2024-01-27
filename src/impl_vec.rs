use glam::{
	BVec2, BVec3, BVec4, BVec4A, DVec2, DVec3, DVec4, I16Vec2, I16Vec3, I16Vec4, I64Vec2, I64Vec3,
	I64Vec4, IVec2, IVec3, IVec4, U16Vec2, U16Vec3, U16Vec4, U64Vec2, U64Vec3, U64Vec4, UVec2,
	UVec3, UVec4, Vec2, Vec3, Vec4
};
use crate::vec::{
	GVec, SignedVec, FloatVec, IntVec,
	GVec2, SignedVec2, FloatVec2, IntVec2,
	GVec3, SignedVec3, FloatVec3, IntVec3,
	GVec4, SignedVec4, FloatVec4, IntVec4,
};

macro_rules! impl_gvec {
	($scalar:ty, $($type:ty),*) => {
		$(impl GVec for $type {
			type Scalar = $scalar;
			const ZERO: Self = Self::ZERO;
			const ONE: Self = Self::ONE;
			const MIN: Self = Self::MIN;
			const MAX: Self = Self::MAX;
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
			fn length_squared(self) -> Self::Scalar { self.length_squared() }
		})*
	};
}

impl_gvec!(f32, Vec2, Vec3, Vec4);
impl_gvec!(f64, DVec2, DVec3, DVec4);
impl_gvec!(i16, I16Vec2, I16Vec3, I16Vec4);
impl_gvec!(u16, U16Vec2, U16Vec3, U16Vec4);
impl_gvec!(i32, IVec2, IVec3, IVec4);
impl_gvec!(u32, UVec2, UVec3, UVec4);
impl_gvec!(i64, I64Vec2, I64Vec3, I64Vec4);
impl_gvec!(u64, U64Vec2, U64Vec3, U64Vec4);

macro_rules! impl_signed_vec {
	($($type:ty),*) => {
		$(impl SignedVec for $type {
			const NEG_ONE: Self = Self::NEG_ONE;
			fn abs(self) -> Self { self.abs() }
			fn signum(self) -> Self { self.signum() }
			fn is_negative_bitmask(self) -> u32 { self.is_negative_bitmask() }
			fn distance_squared(self, rhs: Self) -> Self::Scalar { self.distance_squared(rhs) }
			fn div_euclid(self, rhs: Self) -> Self { self.div_euclid(rhs) }
		})*
	};
}

impl_signed_vec!(Vec2, Vec3, Vec4);
impl_signed_vec!(DVec2, DVec3, DVec4);
impl_signed_vec!(I16Vec2, I16Vec3, I16Vec4);
impl_signed_vec!(IVec2, IVec3, IVec4);
impl_signed_vec!(I64Vec2, I64Vec3, I64Vec4);

macro_rules! impl_float_vec {
	($($type:ty),*) => {
		$(impl FloatVec for $type {
			const NAN: Self = Self::NAN;
			const INFINITY: Self = Self::INFINITY;
			const NEG_INFINITY: Self = Self::NEG_INFINITY;
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
		})*
	};
}

impl_float_vec!(Vec2, Vec3, Vec4);
impl_float_vec!(DVec2, DVec3, DVec4);

macro_rules! impl_int_vec {
	($($type:ty),*) => {
		$(impl IntVec for $type {
			fn wrapping_add(self, rhs: Self) -> Self { self.wrapping_add(rhs) }
			fn wrapping_sub(self, rhs: Self) -> Self { self.wrapping_sub(rhs) }
			fn wrapping_mul(self, rhs: Self) -> Self { self.wrapping_mul(rhs) }
			fn wrapping_div(self, rhs: Self) -> Self { self.wrapping_div(rhs) }
			fn saturating_add(self, rhs: Self) -> Self { self.saturating_add(rhs) }
			fn saturating_sub(self, rhs: Self) -> Self { self.saturating_sub(rhs) }
			fn saturating_mul(self, rhs: Self) -> Self { self.saturating_mul(rhs) }
			fn saturating_div(self, rhs: Self) -> Self { self.saturating_div(rhs) }
		})*
	};
}

impl_int_vec!(I16Vec2, I16Vec3, I16Vec4);
impl_int_vec!(U16Vec2, U16Vec3, U16Vec4);
impl_int_vec!(IVec2, IVec3, IVec4);
impl_int_vec!(UVec2, UVec3, UVec4);
impl_int_vec!(I64Vec2, I64Vec3, I64Vec4);
impl_int_vec!(U64Vec2, U64Vec3, U64Vec4);

macro_rules! impl_gvec2 {
	($($type:ty),*) => {
		$(impl GVec2 for $type {
			const X: Self = Self::X;
			const Y: Self = Self::Y;
			const AXES: [Self; 2] = Self::AXES;
			fn new(x: Self::Scalar, y: Self::Scalar) -> Self { Self::new(x, y) }
			fn from_array(a: [Self::Scalar; 2]) -> Self { Self::from_array(a) }
			fn to_array(&self) -> [Self::Scalar; 2] { self.to_array() }
			fn select(mask: BVec2, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
			fn cmpeq(self, rhs: Self) -> BVec2 { self.cmpeq(rhs) }
			fn cmpne(self, rhs: Self) -> BVec2 { self.cmpne(rhs) }
			fn cmpge(self, rhs: Self) -> BVec2 { self.cmpge(rhs) }
			fn cmpgt(self, rhs: Self) -> BVec2 { self.cmpgt(rhs) }
			fn cmple(self, rhs: Self) -> BVec2 { self.cmple(rhs) }
			fn cmplt(self, rhs: Self) -> BVec2 { self.cmplt(rhs) }
		})*
	};
}

impl_gvec2!(Vec2, DVec2, I16Vec2, U16Vec2, IVec2, UVec2, I64Vec2, U64Vec2);

macro_rules! impl_signed_vec2 {
	($($type:ty),*) => {
		$(impl SignedVec2 for $type {
			const NEG_X: Self = Self::NEG_X;
			const NEG_Y: Self = Self::NEG_Y;
			fn perp(self) -> Self { self.perp() }
			fn perp_dot(self, rhs: Self) -> Self::Scalar { self.perp_dot(rhs) }
			fn rotate(self, rhs: Self) -> Self { self.rotate(rhs) }
		})*
	};
}

impl_signed_vec2!(Vec2, DVec2, I16Vec2, IVec2, I64Vec2);

macro_rules! impl_float_vec2 {
	($($type:ty),*) => {
		$(impl FloatVec2 for $type {
			fn is_nan_mask(self) -> BVec2 { self.is_nan_mask() }
			fn angle_between(self, rhs: Self) -> Self::Scalar { self.angle_between(rhs) }
			fn from_angle(angle: Self::Scalar) -> Self { Self::from_angle(angle) }
			fn to_angle(self) -> Self::Scalar { self.to_angle() }
		})*
	};
}

impl_float_vec2!(Vec2, DVec2);

macro_rules! impl_int_vec2 {
	($($type:ty),*) => {
		$(impl IntVec2 for $type {})*
	};
}

impl_int_vec2!(I16Vec2, U16Vec2, IVec2, UVec2, I64Vec2, U64Vec2);

macro_rules! impl_gvec3 {
	($($type:ty),*) => {
		$(impl GVec3 for $type {
			const X: Self = Self::X;
			const Y: Self = Self::Y;
			const Z: Self = Self::Z;
			const AXES: [Self; 3] = Self::AXES;
			fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar) -> Self { Self::new(x, y, z) }
			fn from_array(a: [Self::Scalar; 3]) -> Self { Self::from_array(a) }
			fn to_array(&self) -> [Self::Scalar; 3] { self.to_array() }
			fn select(mask: BVec3, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
			fn cmpeq(self, rhs: Self) -> BVec3 { self.cmpeq(rhs) }
			fn cmpne(self, rhs: Self) -> BVec3 { self.cmpne(rhs) }
			fn cmpge(self, rhs: Self) -> BVec3 { self.cmpge(rhs) }
			fn cmpgt(self, rhs: Self) -> BVec3 { self.cmpgt(rhs) }
			fn cmple(self, rhs: Self) -> BVec3 { self.cmple(rhs) }
			fn cmplt(self, rhs: Self) -> BVec3 { self.cmplt(rhs) }
			fn cross(self, rhs: Self) -> Self { self.cross(rhs) }
		})*
	};
}

impl_gvec3!(Vec3, DVec3, I16Vec3, U16Vec3, IVec3, UVec3, I64Vec3, U64Vec3);

macro_rules! impl_signed_vec3 {
	($($type:ty),*) => {
		$(impl SignedVec3 for $type {
			const NEG_X: Self = Self::NEG_X;
			const NEG_Y: Self = Self::NEG_Y;
			const NEG_Z: Self = Self::NEG_Z;
		})*
	};
}

impl_signed_vec3!(Vec3, DVec3, I16Vec3, IVec3, I64Vec3);

macro_rules! impl_float_vec3 {
	($($type:ty),*) => {
		$(impl FloatVec3 for $type {
			fn is_nan_mask(self) -> BVec3 { self.is_nan_mask() }
			fn angle_between(self, rhs: Self) -> Self::Scalar { self.angle_between(rhs) }
			fn any_orthogonal_vector(&self) -> Self { self.any_orthogonal_vector() }
			fn any_orthonormal_vector(&self) -> Self { self.any_orthonormal_vector() }
			fn any_orthonormal_pair(&self) -> (Self, Self) { self.any_orthonormal_pair() }
		})*
	};
}

impl_float_vec3!(Vec3, DVec3);

macro_rules! impl_int_vec3 {
	($($type:ty),*) => {
		$(impl IntVec3 for $type {})*
	};
}

impl_int_vec3!(I16Vec3, U16Vec3, IVec3, UVec3, I64Vec3, U64Vec3);

macro_rules! impl_gvec4 {
	($bvec:ty, $($type:ty),*) => {
		$(impl GVec4 for $type {
			type BVec4Type = $bvec;
			const X: Self = Self::X;
			const Y: Self = Self::Y;
			const Z: Self = Self::Z;
			const W: Self = Self::W;
			const AXES: [Self; 4] = Self::AXES;
			fn new(x: Self::Scalar, y: Self::Scalar, z: Self::Scalar, w: Self::Scalar) -> Self { Self::new(x, y, z, w) }
			fn from_array(a: [Self::Scalar; 4]) -> Self { Self::from_array(a) }
			fn to_array(&self) -> [Self::Scalar; 4] { self.to_array() }
			fn select(mask: Self::BVec4Type, if_true: Self, if_false: Self) -> Self { Self::select(mask, if_true, if_false) }
			fn cmpeq(self, rhs: Self) -> Self::BVec4Type { self.cmpeq(rhs) }
			fn cmpne(self, rhs: Self) -> Self::BVec4Type { self.cmpne(rhs) }
			fn cmpge(self, rhs: Self) -> Self::BVec4Type { self.cmpge(rhs) }
			fn cmpgt(self, rhs: Self) -> Self::BVec4Type { self.cmpgt(rhs) }
			fn cmple(self, rhs: Self) -> Self::BVec4Type { self.cmple(rhs) }
			fn cmplt(self, rhs: Self) -> Self::BVec4Type { self.cmplt(rhs) }
		})*
	};
}

impl_gvec4!(BVec4, DVec4, I16Vec4, U16Vec4, IVec4, UVec4, I64Vec4, U64Vec4);
impl_gvec4!(BVec4A, Vec4);

macro_rules! impl_signed_vec4 {
	($($type:ty),*) => {
		$(impl SignedVec4 for $type {
			const NEG_X: Self = Self::NEG_X;
			const NEG_Y: Self = Self::NEG_Y;
			const NEG_Z: Self = Self::NEG_Z;
			const NEG_W: Self = Self::NEG_W;
		})*
	};
}

impl_signed_vec4!(Vec4, DVec4, I16Vec4, IVec4, I64Vec4);

macro_rules! impl_float_vec4 {
	($($type:ty),*) => {
		$(impl FloatVec4 for $type {
			fn is_nan_mask(self) -> Self::BVec4Type { self.is_nan_mask() }
		})*
	};
}

impl_float_vec4!(Vec4, DVec4);

macro_rules! impl_int_vec4 {
	($($type:ty),*) => {
		$(impl IntVec4 for $type {})*
	};
}

impl_int_vec4!(I16Vec4, U16Vec4, IVec4, UVec4, I64Vec4, U64Vec4);
