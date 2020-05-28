// wengwengweng

use super::*;
use crate::Result;

// TODO: take blending and stuff into account
#[derive(Clone, PartialEq)]
struct RenderState<V: VertexLayout, U: UniformLayout> {
	pipeline: Pipeline<V, U>,
	prim: Primitive,
	uniform: U,
}

pub struct BatchedMesh<V: VertexLayout, U: UniformLayout> {
	vbuf: VertexBuffer<V>,
	ibuf: IndexBuffer,
	vqueue: Vec<V>,
	iqueue: Vec<u32>,
	cur_state: Option<RenderState<V, U>>,
	draw_count: usize,
}

impl<V: VertexLayout, U: UniformLayout> BatchedMesh<V, U> {

	pub fn new(ctx: &impl HasGL, max_vertices: usize, max_indices: usize) -> Result<Self> {

		let vbuf = VertexBuffer::new(ctx, max_vertices, BufferUsage::Dynamic)?;
		let ibuf = IndexBuffer::new(ctx, max_indices, BufferUsage::Dynamic)?;

		return Ok(Self {
			vbuf,
			ibuf,
			vqueue: Vec::with_capacity(max_vertices),
			iqueue: Vec::with_capacity(max_indices),
			cur_state: None,
			draw_count: 0,
		});

	}

	// TODO: review the logic here
	pub fn push(
		&mut self,
		prim: Primitive,
		verts: &[V],
		indices: &[u32],
		pipeline: &Pipeline<V, U>,
		uniform: &U,
	) -> Result<()> {

		let mut reset = false;

		if let Some(state) = &self.cur_state {
			if
				&state.pipeline != pipeline
				|| &state.uniform != uniform
				|| state.prim != prim
			{
				reset = true;
			}
		} else {
			reset = true;
		}

		if reset {
			self.flush();
			self.cur_state = Some(RenderState {
				pipeline: pipeline.clone(),
				uniform: uniform.clone(),
				prim,
			});
		}

		if self.vqueue.len() + verts.len() >= self.vqueue.capacity() {
			self.flush();
		}

		if self.iqueue.len() + indices.len() >= self.iqueue.capacity() {
			self.flush();
		}

		let offset = (self.vqueue.len()) as u32;

		let indices = indices
			.iter()
			.map(|i| *i + offset)
			.collect::<Vec<u32>>();

		self.vqueue.extend_from_slice(&verts);
		self.iqueue.extend_from_slice(&indices);

		return Ok(());

	}

	pub fn flush(&mut self) {

		if self.empty() {
			return;
		}

		let state = match &self.cur_state {
			Some(s) => s,
			None => return,
		};

		self.vbuf.data(0, &self.vqueue);
		self.ibuf.data(0, &self.iqueue);

		state.pipeline.draw(
			Some(&self.vbuf),
			Some(&self.ibuf),
			&state.uniform,
			self.iqueue.len() as u32,
			state.prim,
		);

		self.cur_state = None;
		self.vqueue.clear();
		self.iqueue.clear();
		self.draw_count += 1;

	}

	pub fn empty(&self) -> bool {
		return self.vqueue.is_empty();
	}

	pub fn clear_draw_count(&mut self) {
		self.draw_count = 0;
	}

	pub fn draw_count(&self) -> usize {
		return self.draw_count;
	}

	pub fn free(self) {
		self.vbuf.free();
		self.ibuf.free();
	}

}



