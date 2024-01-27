use glam::{BVec4, BVec4A};
use crate::bvec::GBVec4;

macro_rules! impl_gbvec4 {
	($type:ty) => {
		impl GBVec4 for $type {
			const FALSE: Self = Self::FALSE;
			const TRUE: Self = Self::TRUE;
			fn new(x: bool, y: bool, z: bool, w: bool) -> Self { Self::new(x, y, z, w) }
			fn splat(v: bool) -> Self { Self::splat(v) }
			fn from_array(a: [bool; 4]) -> Self { Self::from_array(a) }
			fn bitmask(self) -> u32 { self.bitmask() }
			fn any(self) -> bool { self.any() }
			fn all(self) -> bool { self.all() }
			fn test(&self, index: usize) -> bool { self.test(index) }
			fn set(&mut self, index: usize, value: bool) { self.set(index, value) }
		}
	};
}

impl_gbvec4!(BVec4);
impl_gbvec4!(BVec4A);
