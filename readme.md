# glam-traits

[![Latest Version]][crates.io] [![docs]][docs.rs]

[Latest Version]: https://img.shields.io/crates/v/glam-traits.svg
[crates.io]: https://crates.io/crates/glam-traits/
[docs]: https://docs.rs/glam-traits/badge.svg
[docs.rs]: https://docs.rs/glam-traits/

Traits for the vectors in [glam](https://github.com/bitshifter/glam-rs/).

There are traits for the following characteristics:
* "Any"
* Signed
* Float
* Int
* Signed int
* Unsigned int

For lengths "any", 2, 3 and 4:

<table>
	<tr>
		<td></td>
		<td>Any</td>
		<td>2</td>
		<td>3</td>
		<td>4</td>
	</tr>
	<tr>
		<td>Any</td>
		<td><code>GVec</code></td>
		<td><code>GVec2</code></td>
		<td><code>GVec3</code></td>
		<td><code>GVec4</code></td>
	</tr>
	<tr>
		<td>Signed</td>
		<td><code>SignedVec</code></td>
		<td><code>SignedVec2</code></td>
		<td><code>SignedVec3</code></td>
		<td><code>SignedVec4</code></td>
	</tr>
	<tr>
		<td>Float</td>
		<td><code>FloatVec</code></td>
		<td><code>FloatVec2</code></td>
		<td><code>FloatVec3</code></td>
		<td><code>FloatVec4</code></td>
	</tr>
	<tr>
		<td>Int</td>
		<td><code>IntVec</code></td>
		<td><code>IntVec2</code></td>
		<td><code>IntVec3</code></td>
		<td><code>IntVec4</code></td>
	</tr>
	<tr>
		<td>Signed int</td>
		<td><code>SIntVec</code></td>
		<td><code>SIntVec2</code></td>
		<td><code>SIntVec3</code></td>
		<td><code>SIntVec4</code></td>
	</tr>
	<tr>
		<td>Unsigned int</td>
		<td><code>UIntVec</code></td>
		<td><code>UIntVec2</code></td>
		<td><code>UIntVec3</code></td>
		<td><code>UIntVec4</code></td>
	</tr>
</table>

As well as for concrete types of any length:

`I16Vec`, `U16Vec`, `I32Vec`, `U32Vec`, `I64Vec`, `U64Vec`, `F32Vec`, `F64Vec`

`GBVec` is also provided to cover boolean vectors.

Traits are implemented for the appropriate glam types.
