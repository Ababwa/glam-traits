mod bvec;
mod vec;

pub use bvec::gbvec::GBVec;
pub use bvec::gbvec3::GBVec3;
pub use bvec::gbvec4::GBVec4;

pub use vec::gen::gen_vec::GVec;
pub use vec::gen::gen_vec2::GVec2;
pub use vec::gen::gen_vec3::GVec3;
pub use vec::gen::gen_vec4::GVec4;

pub use vec::signed::signed_vec::SignedVec;
pub use vec::signed::signed_vec2::SignedVec2;
pub use vec::signed::signed_vec3::SignedVec3;
pub use vec::signed::signed_vec4::SignedVec4;

pub use vec::float::float_vec::FloatVec;
pub use vec::float::float_vec2::FloatVec2;
pub use vec::float::float_vec3::FloatVec3;
pub use vec::float::float_vec4::FloatVec4;

pub use vec::int::int_vec::IntVec;
pub use vec::int::int_vec2::IntVec2;
pub use vec::int::int_vec3::IntVec3;
pub use vec::int::int_vec4::IntVec4;

pub use vec::sint::sint_vec::SIntVec;
pub use vec::sint::sint_vec2::SIntVec2;
pub use vec::sint::sint_vec3::SIntVec3;
pub use vec::sint::sint_vec4::SIntVec4;

pub use vec::uint::uint_vec::UIntVec;
pub use vec::uint::uint_vec2::UIntVec2;
pub use vec::uint::uint_vec3::UIntVec3;
pub use vec::uint::uint_vec4::UIntVec4;

pub use vec::typed::i16_vec::I16Vec;
pub use vec::typed::u16_vec::U16Vec;
pub use vec::typed::i32_vec::I32Vec;
pub use vec::typed::u32_vec::U32Vec;
pub use vec::typed::i64_vec::I64Vec;
pub use vec::typed::u64_vec::U64Vec;
pub use vec::typed::f32_vec::F32Vec;
pub use vec::typed::f64_vec::F64Vec;
