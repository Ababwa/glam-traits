use std::{
	fmt::{Debug, Display},
	hash::Hash,
	ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

pub trait GBVec4
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
		Into<[bool; 4]> +
		Into<[u32; 4]> +
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
	fn new(x: bool, y: bool, z: bool, w: bool) -> Self;
}

impl GBVec4 for glam::BVec4 {
	const FALSE: Self = Self::FALSE;
	const TRUE: Self = Self::TRUE;
	fn splat(v: bool) -> Self { Self::splat(v) }
	fn bitmask(self) -> u32 { self.bitmask() }
	fn any(self) -> bool { self.any() }
	fn all(self) -> bool { self.all() }
	fn test(&self, index: usize) -> bool { self.test(index) }
	fn set(&mut self, index: usize, value: bool) { self.set(index, value) }
	fn new(x: bool, y: bool, z: bool, w: bool) -> Self { Self::new(x, y, z, w) }
}

impl GBVec4 for glam::BVec4A {
	const FALSE: Self = Self::FALSE;
	const TRUE: Self = Self::TRUE;
	fn splat(v: bool) -> Self { Self::splat(v) }
	fn bitmask(self) -> u32 { self.bitmask() }
	fn any(self) -> bool { self.any() }
	fn all(self) -> bool { self.all() }
	fn test(&self, index: usize) -> bool { self.test(index) }
	fn set(&mut self, index: usize, value: bool) { self.set(index, value) }
	fn new(x: bool, y: bool, z: bool, w: bool) -> Self { Self::new(x, y, z, w) }
}
