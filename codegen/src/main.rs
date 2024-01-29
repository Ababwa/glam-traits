use std::{fs::File, slice};
use anyhow::Result;
use tera::{Context, Tera};

fn main() -> Result<()> {
	let tera = Tera::new("codegen/templates/*")?;
	for (dim, impls) in [
		(None, &[("BVec2", 2), ("BVec3", 3), ("BVec3A", 3), ("BVec4", 4), ("BVec4A", 4)] as &[_]),
		(Some(3), &[("BVec3", 3), ("BVec3A", 3)]),
		(Some(4), &[("BVec4", 4), ("BVec4A", 4)]),
	] {
		let mut context = Context::new();
		let d = match dim {
			Some(dim) => {
				context.insert("dim", &dim);
				dim.to_string()
			},
			None => "".to_string(),
		};
		let name = format!("src/bvec/gbvec{}.rs", d);
		let file = File::create(&name)?;
		tera.render_to("bvec.rs.tera", &context, &file)?;
		for (impl_t, impl_dim) in impls {
			context.insert("impl_t", impl_t);
			context.insert("impl_dim", impl_dim);
			tera.render_to("bvec_impl.rs.tera", &context, &file)?;
		}
	}
	let i16v = ("I16", "i16");
	let u16v = ("U16", "u16");
	let i32v = ("I", "i32");
	let u32v = ("U", "u32");
	let i64v = ("I64", "i64");
	let u64v = ("U64", "u64");
	let f32v = ("", "f32");
	let f64v = ("D", "f64");
	for dim in [None, Some(2), Some(3), Some(4)] {
		let impl_dims = match &dim {
			Some(dim) => slice::from_ref(dim),
			None => &[2, 3, 4],
		};
		for (name, is_signed, is_float, is_int, is_unsigned, impls) in [
			("gen", false, false, false, false, &[i16v, u16v, i32v, u32v, i64v, u64v, f32v, f64v] as &[_]),
			("signed", true, false, false, false, &[i16v, i32v, i64v, f32v, f64v]),
			("float", true, true, false, false, &[f32v, f64v]),
			("int", false, false, true, false, &[i16v, u16v, i32v, u32v, i64v, u64v]),
			("sint", true, false, true, false, &[i16v, i32v, i64v]),
			("uint", false, false, true, true, &[u16v, u32v, u64v]),
		] {
			let mut context = Context::new();
			for (key, val) in [
				("is_signed", is_signed),
				("is_float", is_float),
				("is_int", is_int),
				("is_unsigned", is_unsigned),
			] {
				context.insert(key, &val);
			}
			let d = match dim {
				Some(dim) => {
					context.insert("dim", &dim);
					dim.to_string()
				},
				None => "".to_string(),
			};
			let name = format!("src/vec/{}/{}_vec{}.rs", name, name, d);
			let file = File::create(&name).unwrap();
			tera.render_to("vec.rs.tera", &context, &file)?;
			for &(prefix, scalar) in impls {
				for &impl_dim in impl_dims {
					let impl_t = format!("{}Vec{}", prefix, impl_dim);
					let impl_minus_one_t = format!("{}Vec{}", prefix, impl_dim - 1);
					let impl_plus_one_t = format!("{}Vec{}", prefix, impl_dim + 1);
					let impl_bvec_t = if scalar == "f32" && impl_dim == 4 {
						"BVec4A".to_string()
					} else {
						format!("BVec{}", impl_dim)
					};
					context.insert("impl_t", &impl_t);
					context.insert("impl_scalar_t", scalar);
					context.insert("impl_dim", &impl_dim);
					context.insert("impl_bvec_t", &impl_bvec_t);
					context.insert("impl_minus_one_t", &impl_minus_one_t);
					context.insert("impl_plus_one_t", &impl_plus_one_t);
					tera.render_to("vec_impl.rs.tera", &context, &file)?;
					if scalar == "f32" && impl_dim == 3 {
						context.insert("impl_t", "Vec3A");
						context.insert("impl_bvec_t", "BVec3A");
						tera.render_to("vec_impl.rs.tera", &context, &file)?;
					}
				}
			}
		}
	}
	for (prefix, scalar) in [i16v, u16v, i32v, u32v, i64v, u64v, f32v, f64v] {
		let mut context = Context::new();
		context.insert("strong_t", scalar);
		let name = format!("src/vec/typed/{}_vec.rs", scalar);
		let file = File::create(&name).unwrap();
		tera.render_to("vec.rs.tera", &context, &file)?;
		for impl_dim in 2..=4 {
			let impl_t = format!("{}Vec{}", prefix, impl_dim);
			let impl_bvec_t = if scalar == "f32" && impl_dim == 4 {
				"BVec4A".to_string()
			} else {
				format!("BVec{}", impl_dim)
			};
			context.insert("impl_t", &impl_t);
			context.insert("impl_dim", &impl_dim);
			context.insert("impl_bvec_t", &impl_bvec_t);
			tera.render_to("vec_impl.rs.tera", &context, &file)?;
		}
		if scalar == "f32" {
			context.insert("impl_t", "Vec3A");
			context.insert("impl_dim", &3);
			context.insert("impl_bvec_t", "BVec3A");
			tera.render_to("vec_impl.rs.tera", &context, &file)?;
		}
	}
	Ok(())
}
