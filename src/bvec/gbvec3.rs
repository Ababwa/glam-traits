use std::{
	fmt::{Debug, Display},
	hash::Hash,
	ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

pub trait GBVec3
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
		Into<[bool; 3]> +
		Into<[u32; 3]> +
	,
{
	const FALSE: Self;
	const TRUE: Self;
	fn splat(v: bool) -> Self;
	fn bitmask(self) -> u32;
	fn any(self) -> bool;
	fn all(self) -> bool;
	fn test(&self, index: usize) -> bool;
	fn set(&mut self, index: usize, value: bool);
	fn new(x: bool, y: bool, z: bool) -> Self;
}

impl GBVec3 for glam::BVec3 {
	const FALSE: Self = Self::FALSE;
	const TRUE: Self = Self::TRUE;
	fn splat(v: bool) -> Self { Self::splat(v) }
	fn bitmask(self) -> u32 { self.bitmask() }
	fn any(self) -> bool { self.any() }
	fn all(self) -> bool { self.all() }
	fn test(&self, index: usize) -> bool { self.test(index) }
	fn set(&mut self, index: usize, value: bool) { self.set(index, value) }
	fn new(x: bool, y: bool, z: bool) -> Self { Self::new(x, y, z) }
}

impl GBVec3 for glam::BVec3A {
	const FALSE: Self = Self::FALSE;
	const TRUE: Self = Self::TRUE;
	fn splat(v: bool) -> Self { Self::splat(v) }
	fn bitmask(self) -> u32 { self.bitmask() }
	fn any(self) -> bool { self.any() }
	fn all(self) -> bool { self.all() }
	fn test(&self, index: usize) -> bool { self.test(index) }
	fn set(&mut self, index: usize, value: bool) { self.set(index, value) }
	fn new(x: bool, y: bool, z: bool) -> Self { Self::new(x, y, z) }
}
