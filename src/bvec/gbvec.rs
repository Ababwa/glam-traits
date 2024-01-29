use std::{
	fmt::{Debug, Display},
	hash::Hash,
	ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

pub trait GBVec
where
	Self:
		Clone +
		Copy +
		PartialEq +
		Eq +
		Hash +
		Default +
		BitAnd +
		BitAndAssign +
		BitOr +
		BitOrAssign +
		BitXor +
		BitXorAssign +
		Not +
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

impl GBVec for glam::BVec2 {
	const FALSE: Self = Self::FALSE;
	const TRUE: Self = Self::TRUE;
	const DIM: usize = 2;
	fn splat(v: bool) -> Self { Self::splat(v) }
	fn bitmask(self) -> u32 { self.bitmask() }
	fn any(self) -> bool { self.any() }
	fn all(self) -> bool { self.all() }
	fn test(&self, index: usize) -> bool { self.test(index) }
	fn set(&mut self, index: usize, value: bool) { self.set(index, value) }
}

impl GBVec for glam::BVec3 {
	const FALSE: Self = Self::FALSE;
	const TRUE: Self = Self::TRUE;
	const DIM: usize = 3;
	fn splat(v: bool) -> Self { Self::splat(v) }
	fn bitmask(self) -> u32 { self.bitmask() }
	fn any(self) -> bool { self.any() }
	fn all(self) -> bool { self.all() }
	fn test(&self, index: usize) -> bool { self.test(index) }
	fn set(&mut self, index: usize, value: bool) { self.set(index, value) }
}

impl GBVec for glam::BVec3A {
	const FALSE: Self = Self::FALSE;
	const TRUE: Self = Self::TRUE;
	const DIM: usize = 3;
	fn splat(v: bool) -> Self { Self::splat(v) }
	fn bitmask(self) -> u32 { self.bitmask() }
	fn any(self) -> bool { self.any() }
	fn all(self) -> bool { self.all() }
	fn test(&self, index: usize) -> bool { self.test(index) }
	fn set(&mut self, index: usize, value: bool) { self.set(index, value) }
}

impl GBVec for glam::BVec4 {
	const FALSE: Self = Self::FALSE;
	const TRUE: Self = Self::TRUE;
	const DIM: usize = 4;
	fn splat(v: bool) -> Self { Self::splat(v) }
	fn bitmask(self) -> u32 { self.bitmask() }
	fn any(self) -> bool { self.any() }
	fn all(self) -> bool { self.all() }
	fn test(&self, index: usize) -> bool { self.test(index) }
	fn set(&mut self, index: usize, value: bool) { self.set(index, value) }
}

impl GBVec for glam::BVec4A {
	const FALSE: Self = Self::FALSE;
	const TRUE: Self = Self::TRUE;
	const DIM: usize = 4;
	fn splat(v: bool) -> Self { Self::splat(v) }
	fn bitmask(self) -> u32 { self.bitmask() }
	fn any(self) -> bool { self.any() }
	fn all(self) -> bool { self.all() }
	fn test(&self, index: usize) -> bool { self.test(index) }
	fn set(&mut self, index: usize, value: bool) { self.set(index, value) }
}
