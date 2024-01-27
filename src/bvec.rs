use std::{fmt::{Debug, Display}, hash::Hash, ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not}};

//necessary for GVec4, since different implementors use different BVecs
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
		From<[bool; 4]> +
	,
{
	const FALSE: Self;
	const TRUE: Self;
	fn new(x: bool, y: bool, z: bool, w: bool) -> Self;
	fn splat(v: bool) -> Self;
	fn from_array(a: [bool; 4]) -> Self;
	fn bitmask(self) -> u32;
	fn any(self) -> bool;
	fn all(self) -> bool;
	fn test(&self, index: usize) -> bool;
	fn set(&mut self, index: usize, value: bool);
}