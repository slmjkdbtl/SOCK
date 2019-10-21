// wengwengweng

use std::rc::Rc;
use std::io::Cursor;

use crate::*;
use super::*;
use super::gfx::*;

/// mesh data
pub type MeshData = Vec<gl::MeshData<Vertex3D>>;

/// 3d mesh
#[derive(Clone)]
pub struct Mesh {
	data: MeshData,
	meshes: Vec<Rc<gl::Mesh<Vertex3D, Uniform3D>>>,
}

impl Mesh {

	/// load mesh data with materials that's safe to send between threads
	pub fn prepare_obj(obj: &str, mtl: Option<&str>) -> Result<MeshData> {

		let (models, materials) = tobj::load_obj_buf(&mut Cursor::new(obj), |_| {
			return mtl
				.map(|m| tobj::load_mtl_buf(&mut Cursor::new(m)))
				.unwrap_or(Ok((vec![], hashmap![])));
		})?;

		let mut meshes = Vec::with_capacity(models.len());

		for m in models {

			let m = m.mesh;
			let vert_count = m.positions.len() / 3;
			let mut verts = Vec::with_capacity(vert_count);

			let normals = if m.normals.is_empty() {
				gen_normals(&m.positions, &m.indices)
			} else {
				m.normals
					.chunks(3)
					.map(|n| vec3!(n[0], n[1], n[2]))
					.collect()
			};

			let mtl = m.material_id
				.map(|id| materials.get(id))
				.flatten();

			let color = mtl
				.map(|m| m.diffuse)
				.map(|d| color!(d[0], d[1], d[2], 1.0))
				.unwrap_or(color!(rand!(), rand!(), rand!(), 1));

			for i in 0..vert_count {

				let vx = m.positions[i * 3 + 0];
				let vy = m.positions[i * 3 + 1];
				let vz = m.positions[i * 3 + 2];

				verts.push(Vertex3D {
					pos: vec3!(vx, vy, vz),
					normal: normals[i],
					uv: vec2!(),
					color: color,
				});

			}

			meshes.push(gl::MeshData {
				vertices: verts,
				indices: m.indices,
			});

		}

		return Ok(meshes);

	}

	/// create model with mesh data
	pub fn from(ctx: &Ctx, meshdata: MeshData) -> Result<Self> {

		let mut meshes = Vec::with_capacity(meshdata.len());

		for m in &meshdata {
			meshes.push(Rc::new(gl::Mesh::from_meshdata(&ctx.gl, m.clone())?));
		}

		return Ok(Self {
			data: meshdata,
			meshes: meshes,
		});

	}

	/// create model from obj
	pub fn from_obj(ctx: &Ctx, obj: &str, mtl: Option<&str>) -> Result<Self> {
		return Self::from(ctx, Self::prepare_obj(obj, mtl)?);
	}

	pub(super) fn meshes(&self) -> &[Rc<gl::Mesh<Vertex3D, Uniform3D>>] {
		return &self.meshes;
	}

	pub fn meshdata(&self) -> &MeshData {
		return &self.data;
	}

	pub fn bbox(&self) -> (Vec3, Vec3) {

		let mut min = vec3!();
		let mut max = vec3!();

		for m in &self.data {

			for v in &m.vertices {

				let pos = v.pos;

				if pos.x < min.x {
					min.x = pos.x;
				}

				if pos.y < min.y {
					min.y = pos.y;
				}

				if pos.z < min.z {
					min.z = pos.z;
				}

				if pos.x > max.x {
					max.x = pos.x;
				}

				if pos.y > max.y {
					max.y = pos.y;
				}

				if pos.z > max.z {
					max.z = pos.z;
				}

			}

		}

		return (min, max);

	}

}

fn gen_normals(pos: &[f32], indices: &[u32]) -> Vec<Vec3> {

	let vert_count = pos.len() / 3;
	let mut normals = vec![vec3!(0); vert_count];

	indices
		.chunks(3)
		.for_each(|tri| {

			let i1 = tri[0] as usize;
			let i2 = tri[1] as usize;
			let i3 = tri[2] as usize;
			let v1 = vec3!(pos[i1 * 3], pos[i1 * 3 + 1], pos[i1 * 3 + 2]);
			let v2 = vec3!(pos[i2 * 3], pos[i2 * 3 + 1], pos[i2 * 3 + 2]);
			let v3 = vec3!(pos[i3 * 3], pos[i3 * 3 + 1], pos[i3 * 3 + 2]);
			let normal = Vec3::cross((v2 - v1), (v3 - v1));

			normals[i1] += normal;
			normals[i2] += normal;
			normals[i3] += normal;

		});

	return normals
		.into_iter()
		.map(|p| p.normalize())
		.collect();

}

