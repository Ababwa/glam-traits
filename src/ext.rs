use crate::GVec;

/**
Generalization of the glam `select()` function for vectors that selects each component from a
number of vectors, using the provided incides.

# Panics
Panics if the length of `indices` is less than `V::DIM`.

Panics if an index is greater than the length of `vecs`.
*/
pub fn select<V: GVec>(vecs: &[V], indices: &[usize]) -> V {
	V::from_slice(&(0..V::DIM).map(|i| vecs[indices[i]][i]).collect::<Vec<_>>())
}
