// wengwengweng

use std::mem;
use std::rc::Rc;
use std::marker::PhantomData;

use glow::Context;

use crate::Error;
use crate::Result;
use crate::math::*;

type GLCtx = glow::native::Context;
type BufferID = <GLCtx as Context>::Buffer;
type ProgramID = <GLCtx as Context>::Program;
type TextureID = <GLCtx as Context>::Texture;
type FramebufferID = <GLCtx as Context>::Framebuffer;
type VertexArrayID = <GLCtx as Context>::VertexArray;

pub struct Device {
	ctx: Rc<GLCtx>,
}

impl Device {

	pub fn from_loader<F: FnMut(&str) -> *const std::os::raw::c_void>(f: F) -> Self {

		return Self {
			ctx: Rc::new(GLCtx::from_loader_function(f)),
		};

	}

	pub fn enable(&self, cap: Capability) {
		unsafe {
			self.ctx.enable(cap.into());
		}
	}

	pub fn disable(&self, cap: Capability) {
		unsafe {
			self.ctx.disable(cap.into());
		}
	}

	pub fn blend_func(&self, src: BlendFac, dest: BlendFac) {
		unsafe {
			self.ctx.blend_func(src.into(), dest.into());
		}
	}

	pub fn blend_func_sep(&self, src_rgb: BlendFac, dest_rgb: BlendFac, src_a: BlendFac, dest_a: BlendFac) {
		unsafe {
			self.ctx.blend_func_separate(src_rgb.into(), dest_rgb.into(), src_a.into(), dest_a.into());
		}
	}

	pub fn depth_func(&self, f: Cmp) {
		unsafe {
			self.ctx.depth_func(f.into());
		}
	}

	pub fn get_error(&self) -> Result<()> {

		unsafe {

			use Error::OpenGL;

			return match self.ctx.get_error() {
				glow::NO_ERROR => Ok(()),
				glow::INVALID_ENUM => Err(OpenGL("INVALID_ENUM".to_owned())),
				glow::INVALID_VALUE => Err(OpenGL("INVALID_VALUE".to_owned())),
				glow::INVALID_OPERATION => Err(OpenGL("INVALID_OPERATION".to_owned())),
				glow::STACK_OVERFLOW => Err(OpenGL("STACK_OVERFLOW".to_owned())),
				glow::STACK_UNDERFLOW => Err(OpenGL("STACK_UNDERFLOW".to_owned())),
				glow::OUT_OF_MEMORY => Err(OpenGL("OUT_OF_MEMORY".to_owned())),
				glow::INVALID_FRAMEBUFFER_OPERATION => Err(OpenGL("INVALID_FRAMEBUFFER_OPERATION".to_owned())),
				_ => Err(OpenGL("UNKNOWN".to_owned())),
			};

		}

	}

	pub fn clear_color(&self, c: Color) {
		unsafe {
			self.ctx.clear_color(c.r, c.g, c.b, c.a);
		}
	}

	pub fn clear(&self, buf: Buffer) {
		unsafe {
			self.ctx.clear(buf.into());
		}
	}

	pub fn stencil_op(&self, sfail: StencilOp, dpfail: StencilOp, dppass: StencilOp) {
		unsafe {
			self.ctx.stencil_op(sfail.into(), dpfail.into(), dppass.into());
		}
	}

	pub fn stencil_mask(&self, m: u32) {
		unsafe {
			self.ctx.stencil_mask(m);
		}
	}

	pub fn stencil_func(&self, f: Cmp) {
		unsafe {
			self.ctx.stencil_func(f.into(), 1, 0xff);
		}
	}

	pub fn viewport(&self, x: u32, y: u32, width: u32, height: u32) {
		unsafe {
			self.ctx.viewport(x as i32, y as i32, width as i32, height as i32);
		}
	}

}

#[cfg(feature="gl3")]
fn draw(ctx: &GLCtx, vao: &VertexArray, ibuf: &IndexBuffer, program: &Program, count: u32, mode: DrawMode) {

	vao.bind();
	ibuf.bind();
	program.bind();

	unsafe {
		ctx.draw_elements(mode.into(), count as i32, glow::UNSIGNED_INT, 0);
	}

	program.unbind();
	ibuf.unbind();
	vao.unbind();

}

#[cfg(not(feature="gl3"))]
fn draw<V: VertexLayout>(ctx: &GLCtx, vbuf: &VertexBuffer<V>, ibuf: &IndexBuffer, program: &Program, count: u32, mode: DrawMode) {

	program.bind();
	vbuf.bind();
	vbuf.bind_attrs(program);
	ibuf.bind();

	unsafe {
		ctx.draw_elements(mode.into(), count as i32, glow::UNSIGNED_INT, 0);
	}

	ibuf.unbind();
	vbuf.unbind();
	program.unbind();

}

pub struct Renderer<V: VertexLayout> {

	ctx: Rc<GLCtx>,
	vbuf: VertexBuffer<V>,
	ibuf: IndexBuffer,
	#[cfg(feature="gl3")]
	vao: VertexArray,
	count: usize,
	mode: DrawMode,
	vertex: PhantomData<V>,

}

impl<V: VertexLayout> Renderer<V> {

	pub fn new(device: &Device, verts: &[f32], indices: &[u32]) -> Result<Self> {

		let vbuf = VertexBuffer::<V>::init(&device, &verts)?;
		let ibuf = IndexBuffer::init(&device, &indices)?;

		#[cfg(feature="gl3")]
		let vao = VertexArray::init(&device, &vbuf)?;

		return Ok(Self {
			ctx: device.ctx.clone(),
			vbuf: vbuf,
			ibuf: ibuf,
			#[cfg(feature="gl3")]
			vao: vao,
			count: indices.len(),
			mode: DrawMode::Triangles,
			vertex: PhantomData,
		});

	}

	pub fn draw(&self, program: &Program) {
		draw(
			&self.ctx,
			#[cfg(feature="gl3")]
			&self.vao,
			#[cfg(not(feature="gl3"))]
			&self.vbuf,
			&self.ibuf,
			&program,
			self.count as u32,
			self.mode,
		);

	}

}

pub struct BatchedRenderer<S: Shape> {

	ctx: Rc<GLCtx>,
	vbuf: VertexBuffer<S::Vertex>,
	ibuf: IndexBuffer,
	#[cfg(feature="gl3")]
	vao: VertexArray,
	queue: Vec<f32>,
	mode: DrawMode,
	cur_texture: Option<Texture>,
	pub cur_program: Option<Program>,
	shape: PhantomData<S>,

}

impl<S: Shape> BatchedRenderer<S> {

	pub fn new(device: &Device, max: usize) -> Result<Self> {

		let indices = S::indices();
		let vert_count = S::COUNT;
		let vert_stride = S::Vertex::STRIDE;
		let max_vertices = max * vert_stride * vert_count;
		let max_indices = max * indices.len();

		let indices_batch: Vec<u32> = indices
			.iter()
			.cycle()
			.take(max_indices)
			.enumerate()
			.map(|(i, vertex)| vertex + i as u32 / 6 * 4)
			.collect();

		let vbuf = VertexBuffer::new(&device, vert_count * vert_stride * max, BufferUsage::Dynamic)?;
		let ibuf = IndexBuffer::init(&device, &indices_batch)?;

		#[cfg(feature="gl3")]
		let vao = VertexArray::init(&device, &vbuf)?;

		return Ok(Self {
			ctx: device.ctx.clone(),
			vbuf: vbuf,
			ibuf: ibuf,
			#[cfg(feature="gl3")]
			vao: vao,
			queue: Vec::with_capacity(max_vertices),
			mode: DrawMode::Triangles,
			cur_texture: None,
			cur_program: None,
			shape: PhantomData,
		});

	}

	pub fn push(&mut self, shape: S, program: &Program, tex: &Texture) -> Result<()> {

		if let Some(cur_tex) = &self.cur_texture {
			if cur_tex != tex {
				self.flush();
				self.cur_texture = Some(tex.clone());
			}
		} else {
			self.cur_texture = Some(tex.clone());
		}

		if let Some(cur_program) = &self.cur_program {
			if cur_program != program {
				self.flush();
				self.cur_program = Some(program.clone());
			}
		} else {
			self.cur_program = Some(program.clone());
		}

		if self.queue.len() >= self.queue.capacity() {
			self.queue.clear();
			return Err(Error::MaxDraw);
		}

		shape.push(&mut self.queue);

		return Ok(());

	}

	pub fn flush(&mut self) {

		if self.empty() {
			return;
		}

		let program = match &self.cur_program {
			Some(p) => p,
			None => return,
		};

		self.vbuf.data(0, &self.queue);

		if let Some(tex) = &self.cur_texture {
			tex.bind();
		}

		draw(
			&self.ctx,
			#[cfg(feature="gl3")]
			&self.vao,
			#[cfg(not(feature="gl3"))]
			&self.vbuf,
			&self.ibuf,
			&program,
			(self.queue.len() * S::indices().len() / S::Vertex::STRIDE / S::COUNT) as u32,
			self.mode,
		);

		if let Some(tex) = &self.cur_texture {
			tex.unbind();
		}

		self.cur_texture = None;
		self.cur_program = None;
		self.queue.clear();

	}

	pub fn empty(&self) -> bool {
		return self.queue.is_empty();
	}

	pub fn frame_end(&mut self) {
		self.flush();
	}

}

pub trait Shape {

	type Vertex: VertexLayout;
	const COUNT: usize;
	fn push(&self, queue: &mut Vec<f32>);
	fn indices() -> Vec<u32>;

}

pub trait VertexLayout {

	const STRIDE: usize;
	fn push(&self, queue: &mut Vec<f32>);
	fn attrs() -> VertexAttrGroup;

}

#[derive(Clone, Debug)]
pub struct VertexAttrGroup {
	attrs: Vec<VertexAttr>,
	cur_offset: usize,
}

impl VertexAttrGroup {

	pub fn build() -> Self {
		return Self {
			attrs: Vec::new(),
			cur_offset: 0,
		};
	}

	pub fn iter(&self) -> std::slice::Iter<VertexAttr> {
		return self.attrs.iter();
	}

	pub fn add(mut self, name: &str, size: u8) -> Self {

		self.attrs.push(VertexAttr {
			name: name.to_owned(),
			size: size as i32,
			offset: self.cur_offset,
		});

		self.cur_offset += size as usize;

		return self;

	}

}

impl<'a> IntoIterator for &'a VertexAttrGroup {

	type Item = &'a VertexAttr;
	type IntoIter = std::slice::Iter<'a, VertexAttr>;

	fn into_iter(self) -> Self::IntoIter {
		return self.attrs.iter();
	}

}

#[derive(Clone, Debug)]
pub struct VertexAttr {

	name: String,
	size: i32,
	offset: usize,

}

#[derive(Clone, Debug)]
struct VertexArray {
	ctx: Rc<GLCtx>,
	id: VertexArrayID,
}

impl VertexArray {

	pub fn new(device: &Device) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
			let id = ctx.create_vertex_array()?;

			let buf = Self {
				ctx: ctx,
				id: id,
			};

			return Ok(buf);

		}

	}

	fn bind(&self) {
		unsafe {
			self.ctx.bind_vertex_array(Some(self.id));
		}
	}

	fn unbind(&self) {
		unsafe {
			self.ctx.bind_vertex_array(None);
		}
	}

	pub fn init<V: VertexLayout>(device: &Device, vbuf: &VertexBuffer<V>) -> Result<Self> {

		let vao = Self::new(device)?;
		vao.attr(vbuf);
		return Ok(vao);

	}

	pub fn attr<V: VertexLayout>(&self, vbuf: &VertexBuffer<V>) {

		unsafe {

			self.bind();
			vbuf.bind();

			for (i, attr) in V::attrs().iter().enumerate() {

				self.ctx.vertex_attrib_pointer_f32(
					i as u32,
					attr.size,
					glow::FLOAT,
					false,
					(V::STRIDE * mem::size_of::<f32>()) as i32,
					(attr.offset * mem::size_of::<f32>()) as i32,
				);

				self.ctx.enable_vertex_attrib_array(i as u32);

			}

			vbuf.unbind();
			self.unbind();

		}

	}

}

impl Drop for VertexArray {
	fn drop(&mut self) {
		unsafe {
			self.ctx.delete_vertex_array(self.id);
		}
	}
}

impl PartialEq for VertexArray {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

#[derive(Clone, Debug)]
struct VertexBuffer<V: VertexLayout> {

	ctx: Rc<GLCtx>,
	id: BufferID,
	stride: usize,
	attrs: VertexAttrGroup,
	layout: PhantomData<V>,

}

impl<V: VertexLayout> VertexBuffer<V> {

	pub fn new(device: &Device, count: usize, usage: BufferUsage) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
			let id = ctx.create_buffer()?;

			let buf = Self {
				ctx: ctx,
				id: id,
				stride: V::STRIDE,
				attrs: V::attrs(),
				layout: PhantomData,
			};

			buf.bind();

			buf.ctx.buffer_data_size(
				glow::ARRAY_BUFFER,
				(count * mem::size_of::<f32>()) as i32,
				usage.into(),
			);

			buf.unbind();

			return Ok(buf);

		}

	}

	pub fn init(device: &Device, data: &[f32]) -> Result<Self> {

		let buf = Self::new(device, data.len(), BufferUsage::Static)?;
		buf.data(0, data);
		return Ok(buf);

	}

	fn bind(&self) {
		unsafe {
			self.ctx.bind_buffer(glow::ARRAY_BUFFER, Some(self.id));
		}
	}

	fn unbind(&self) {
		unsafe {
			self.ctx.bind_buffer(glow::ARRAY_BUFFER, None);
		}
	}

	fn bind_attrs(&self, program: &Program) {

		unsafe {

			for attr in &self.attrs {

				let index = self.ctx.get_attrib_location(program.id, &attr.name) as u32;

				self.ctx.vertex_attrib_pointer_f32(
					index,
					attr.size,
					glow::FLOAT,
					false,
					(self.stride * mem::size_of::<f32>()) as i32,
					(attr.offset * mem::size_of::<f32>()) as i32,
				);

				self.ctx.enable_vertex_attrib_array(index);

			}

		}

	}

	// TODO: change this to take V?
	pub fn data(&self, offset: usize, data: &[f32]) {

		unsafe {

			let byte_len = mem::size_of_val(data) / mem::size_of::<u8>();
			let byte_slice = std::slice::from_raw_parts(data.as_ptr() as *const u8, byte_len);

			self.bind();

			self.ctx.buffer_sub_data_u8_slice(
				glow::ARRAY_BUFFER,
				(offset * mem::size_of::<f32>()) as i32,
				byte_slice,
			);

			self.unbind();

		}

	}

}

impl<V: VertexLayout> Drop for VertexBuffer<V> {
	fn drop(&mut self) {
		unsafe {
			self.ctx.delete_buffer(self.id);
		}
	}
}

impl<V: VertexLayout> PartialEq for VertexBuffer<V> {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

#[derive(Clone, Debug)]
pub struct IndexBuffer {

	ctx: Rc<GLCtx>,
	id: BufferID,

}

impl IndexBuffer {

	pub fn new(device: &Device, count: usize, usage: BufferUsage) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
			let id = ctx.create_buffer()?;

			let buf = Self {
				ctx: ctx,
				id: id,
			};

			buf.bind();

			buf.ctx.buffer_data_size(
				glow::ELEMENT_ARRAY_BUFFER,
				(count * mem::size_of::<u32>()) as i32,
				usage.into(),
			);

			buf.unbind();

			return Ok(buf);

		}

	}

	pub fn init(device: &Device, data: &[u32]) -> Result<Self> {

		let buf = Self::new(device, data.len(), BufferUsage::Static)?;
		buf.data(0, data);
		return Ok(buf);

	}

	fn bind(&self) {
		unsafe {
			self.ctx.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.id));
		}
	}

	fn unbind(&self) {
		unsafe {
			self.ctx.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
		}
	}

	pub fn data(&self, offset: usize, data: &[u32]) {

		unsafe {

			let byte_len = mem::size_of_val(data) / mem::size_of::<u8>();
			let byte_slice = std::slice::from_raw_parts(data.as_ptr() as *const u8, byte_len);

			self.bind();

			self.ctx.buffer_sub_data_u8_slice(
				glow::ELEMENT_ARRAY_BUFFER,
				(offset * mem::size_of::<u32>()) as i32,
				byte_slice,
			);

			self.unbind();

		}

	}

}

impl Drop for IndexBuffer {
	fn drop(&mut self) {
		unsafe {
			self.ctx.delete_buffer(self.id);
		}
	}
}

impl PartialEq for IndexBuffer {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

#[derive(Clone, Debug)]
pub struct Texture {
	ctx: Rc<GLCtx>,
	id: TextureID,
	width: u32,
	height: u32,
}

impl Texture {

	pub fn new(device: &Device, width: u32, height: u32) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
			let id = ctx.create_texture()?;

			let tex = Self {
				ctx: ctx,
				id: id,
				width: width,
				height: height,
			};

			tex.bind();

			tex.ctx.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_WRAP_S,
				glow::CLAMP_TO_EDGE as i32
			);

			tex.ctx.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_WRAP_T,
				glow::CLAMP_TO_EDGE as i32
			);

			tex.ctx.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MIN_FILTER,
				FilterMode::Nearest.into(),
			);

			tex.ctx.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MAG_FILTER,
				FilterMode::Nearest.into(),
			);

			tex.ctx.tex_image_2d(
				glow::TEXTURE_2D,
				0,
				glow::RGBA as i32,
				width as i32,
				height as i32,
				0,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				None,
			);

			tex.unbind();

			return Ok(tex);

		}

	}

	pub fn init(device: &Device, width: u32, height: u32, data: &[u8]) -> Result<Self> {

		let tex = Self::new(device, width, height)?;
		tex.data(0, 0, width, height, data);
		return Ok(tex);

	}

	pub fn filter(&self, f: FilterMode) {

		unsafe {

			self.bind();

			self.ctx.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MIN_FILTER,
				f.into(),
			);

			self.ctx.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MAG_FILTER,
				f.into(),
			);

			self.unbind();

		}

	}

	fn bind(&self) {
		unsafe {
			self.ctx.bind_texture(glow::TEXTURE_2D, Some(self.id));
		}
	}

	fn unbind(&self) {
		unsafe {
			self.ctx.bind_texture(glow::TEXTURE_2D, None);
		}
	}

	pub fn data(&self, x: u32, y: u32, width: u32, height: u32, data: &[u8]) {

		unsafe {

			self.bind();

			self.ctx.tex_sub_image_2d_u8_slice(
				glow::TEXTURE_2D,
				0,
				x as i32,
				y as i32,
				width as i32,
				height as i32,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				Some(data),
			);

			self.unbind();

		}

	}

	pub fn get_data(&self, width: u32, height: u32) -> Vec<u8> {

		let size = (width * height * 4) as usize;
		let pixels = vec![0.0 as u8; size];

		self.bind();

		unsafe {

			self.ctx.get_tex_image_u8_slice(
				glow::TEXTURE_2D,
				0,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				Some(&pixels),
			);

		}

		self.unbind();

		return pixels;

	}

}

// TODO
impl Drop for Texture {
	fn drop(&mut self) {
		unsafe {
// 			self.ctx.delete_texture(self.id);
		}
	}
}

impl PartialEq for Texture {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

#[derive(Clone, Debug)]
pub struct Program {
	ctx: Rc<GLCtx>,
	id: ProgramID,
}

impl Program {

	pub fn new(device: &Device, vert_src: &str, frag_src: &str) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
			let program_id = ctx.create_program()?;

			let vert_id = ctx.create_shader(ShaderType::Vertex.into())?;

			ctx.shader_source(vert_id, vert_src);
			ctx.compile_shader(vert_id);
			ctx.attach_shader(program_id, vert_id);

			if !ctx.get_shader_compile_status(vert_id) {
				return Err(Error::OpenGL(ctx.get_shader_info_log(vert_id)));
			}

			let frag_id = ctx.create_shader(ShaderType::Fragment.into())?;

			ctx.shader_source(frag_id, frag_src);
			ctx.compile_shader(frag_id);
			ctx.attach_shader(program_id, frag_id);

			if !ctx.get_shader_compile_status(frag_id) {
				return Err(Error::OpenGL(ctx.get_shader_info_log(frag_id)));
			}

			ctx.link_program(program_id);

			if !ctx.get_program_link_status(program_id) {
				return Err(Error::OpenGL(ctx.get_program_info_log(program_id)));
			}

			ctx.delete_shader(vert_id);
			ctx.delete_shader(frag_id);

			let program = Self {
				ctx: ctx,
				id: program_id,
			};

			return Ok(program);

		}

	}

	pub fn send(&self, name: &str, value: impl UniformValue) {

		unsafe {

			self.bind();

			use UniformType::*;

			let loc = self.ctx.get_uniform_location(self.id, name);

			match value.get() {
				F1(f) => self.ctx.uniform_1_f32(loc, f),
				F2(f1, f2) => self.ctx.uniform_2_f32(loc, f1, f2),
				F3(f1, f2, f3) => self.ctx.uniform_3_f32(loc, f1, f2, f3),
				F4(f1, f2, f3, f4) => self.ctx.uniform_4_f32(loc, f1, f2, f3, f4),
				Mat4(a) => self.ctx.uniform_matrix_4_f32_slice(loc, false, &a),
			}

			self.unbind();

		}

	}

	pub fn with(&self, f: impl FnOnce()) {

		unsafe {
			self.ctx.use_program(Some(self.id));
			f();
			self.ctx.use_program(None);
		}

	}

	fn bind(&self) {
		unsafe {
			self.ctx.use_program(Some(self.id));
		}
	}

	fn unbind(&self) {
		unsafe {
			self.ctx.use_program(None);
		}
	}

}

// TODO
impl Drop for Program {
	fn drop(&mut self) {
		unsafe {
// 			self.ctx.delete_program(self.id);
		}
	}
}

impl PartialEq for Program {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

#[derive(Clone, Debug)]
pub struct Framebuffer {

	ctx: Rc<GLCtx>,
	id: FramebufferID,

}

impl Framebuffer {

	pub fn new(device: &Device, tex: &Texture) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
			let id = ctx.create_framebuffer()?;

			let rbo = ctx.create_renderbuffer()?;

			ctx.bind_renderbuffer(glow::RENDERBUFFER, Some(rbo));

			ctx.renderbuffer_storage(
				glow::RENDERBUFFER,
				glow::DEPTH24_STENCIL8,
				tex.width as i32,
				tex.height as i32
			);

			ctx.bind_renderbuffer(glow::RENDERBUFFER, None);

			let fbuf = Self {
				ctx: ctx,
				id: id,
			};

			fbuf.bind();

			fbuf.ctx.framebuffer_texture_2d(
				glow::FRAMEBUFFER,
				glow::COLOR_ATTACHMENT0,
				glow::TEXTURE_2D,
				Some(tex.id),
				0,
			);

			fbuf.ctx.framebuffer_renderbuffer(
				glow::FRAMEBUFFER,
				glow::DEPTH_STENCIL_ATTACHMENT,
				glow::RENDERBUFFER,
				Some(rbo),
			);

			if fbuf.ctx.check_framebuffer_status(glow::FRAMEBUFFER) != glow::FRAMEBUFFER_COMPLETE {
				return Err(Error::OpenGL("failed to create framebuffer".to_owned()));
			}

			device.clear(Buffer::Color);
			device.clear(Buffer::Depth);
			device.clear(Buffer::Stencil);

			fbuf.unbind();

			return Ok(fbuf);

		}
	}

	pub fn with<R>(&self, f: impl FnOnce() -> R) -> R {

		unsafe {

			self.ctx.bind_framebuffer(glow::FRAMEBUFFER, Some(self.id));
			let r = f();
			self.ctx.bind_framebuffer(glow::FRAMEBUFFER, None);

			return r;

		}

	}

	fn bind(&self) {
		unsafe {
			self.ctx.bind_framebuffer(glow::FRAMEBUFFER, Some(self.id));
		}
	}

	fn unbind(&self) {
		unsafe {
			self.ctx.bind_framebuffer(glow::FRAMEBUFFER, None);
		}
	}

}

impl Drop for Framebuffer {
	fn drop(&mut self) {
		unsafe {
			self.ctx.delete_framebuffer(self.id);
		}
	}
}

impl PartialEq for Framebuffer {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

macro_rules! bind_enum {

	($name:ident($type:ty) { $($member:ident => $dest:expr),+$(,)? }) => {

		#[allow(missing_docs)]
		#[derive(Clone, Copy, Debug, Eq, PartialEq)]
		pub enum $name {
			$($member,)+
		}

		impl From<$name> for $type {

			fn from(usage: $name) -> $type {

				match usage {
					$($name::$member => $dest,)+
				}

			}

		}

	};

}

bind_enum!(BufferUsage(u32) {
	Static => glow::STATIC_DRAW,
	Dynamic => glow::DYNAMIC_DRAW,
	Stream => glow::STREAM_DRAW,
});

bind_enum!(FilterMode(i32) {
	Nearest => glow::NEAREST as i32,
	Linear => glow::LINEAR as i32,
});

bind_enum!(Capability(u32) {
	Blend => glow::BLEND,
	CullFace => glow::CULL_FACE,
	DepthTest => glow::DEPTH_TEST,
	StencilTest => glow::STENCIL_TEST,
	ScissorTest => glow::SCISSOR_TEST,
});

bind_enum!(BlendFac(u32) {
	Zero => glow::ZERO,
	One => glow::ONE,
	SrcColor => glow::SRC_COLOR,
	OneMinusSrcColor => glow::ONE_MINUS_SRC_COLOR,
	DestColor => glow::DST_COLOR,
	OneMinusDestColor => glow::ONE_MINUS_DST_COLOR,
	SrcAlpha => glow::SRC_ALPHA,
	OneMinusSrcAlpha => glow::ONE_MINUS_SRC_ALPHA,
	DestAlpha => glow::DST_ALPHA,
	OneMinusDestAlpha => glow::ONE_MINUS_DST_ALPHA,
	SrcAlphaSaturate => glow::SRC_ALPHA_SATURATE,
	ConstantColor => glow::CONSTANT_COLOR,
	OneMinusConstantColor => glow::ONE_MINUS_CONSTANT_COLOR,
	ConstantAlpha => glow::CONSTANT_ALPHA,
	OneMinusConstantAlpha => glow::ONE_MINUS_CONSTANT_ALPHA,
});

bind_enum!(StencilOp(u32) {
	Keep => glow::KEEP,
	Zero => glow::ZERO,
	Replace => glow::REPLACE,
	Increment => glow::INCR,
	Decrement => glow::DECR,
	IncWrap => glow::INCR_WRAP,
	DecWrap => glow::DECR_WRAP,
	Invert => glow::INVERT,
});

bind_enum!(Face(u32) {
	Front => glow::FRONT,
	Back => glow::BACK,
	FrontAndBack => glow::FRONT_AND_BACK,
});

bind_enum!(Buffer(u32) {
	Color => glow::COLOR_BUFFER_BIT,
	Stencil => glow::STENCIL_BUFFER_BIT,
	Depth => glow::DEPTH_BUFFER_BIT,
});

bind_enum!(Cmp(u32) {
	Never => glow::NEVER,
	Less => glow::LESS,
	LessOrEqual => glow::LEQUAL,
	Greater => glow::GREATER,
	GreaterOrEqual => glow::GEQUAL,
	Equal => glow::EQUAL,
	NotEqual => glow::NOTEQUAL,
	Always => glow::ALWAYS,
});

bind_enum!(DrawMode(u32) {
	Point => glow::POINT,
	Line => glow::LINE,
	Triangles => glow::TRIANGLES,
	LineStrip => glow::LINE_STRIP,
	TriangleFan => glow::TRIANGLE_FAN,
	TriangleStrip => glow::TRIANGLE_STRIP,
});

bind_enum!(ShaderType(u32) {
	Vertex => glow::VERTEX_SHADER,
	Fragment => glow::FRAGMENT_SHADER,
});

pub enum UniformType {
	F1(f32),
	F2(f32, f32),
	F3(f32, f32, f32),
	F4(f32, f32, f32, f32),
	Mat4([f32; 16]),
}

pub trait UniformValue {
	fn get(&self) -> UniformType;
}

impl UniformValue for f32 {
	fn get(&self) -> UniformType {
		return UniformType::F1(*self);
	}
}

impl UniformValue for [f32; 2] {
	fn get(&self) -> UniformType {
		return UniformType::F2(self[0], self[1]);
	}
}

impl UniformValue for [f32; 3] {
	fn get(&self) -> UniformType {
		return UniformType::F3(self[0], self[1], self[2]);
	}
}

impl UniformValue for [f32; 4] {
	fn get(&self) -> UniformType {
		return UniformType::F4(self[0], self[1], self[2], self[3]);
	}
}

impl UniformValue for Vec2 {
	fn get(&self) -> UniformType {
		return UniformType::F2(self.x, self.y);
	}
}

impl UniformValue for Vec3 {
	fn get(&self) -> UniformType {
		return UniformType::F3(self.x, self.y, self.z);
	}
}

impl UniformValue for Vec4 {
	fn get(&self) -> UniformType {
		return UniformType::F4(self.x, self.y, self.z, self.w);
	}
}

impl UniformValue for Color {
	fn get(&self) -> UniformType {
		return UniformType::F4(self.r, self.g, self.b, self.a);
	}
}

impl UniformValue for Quad {
	fn get(&self) -> UniformType {
		return UniformType::F4(self.x, self.y, self.w, self.h);
	}
}

impl UniformValue for Mat4 {
	fn get(&self) -> UniformType {
		return UniformType::Mat4(self.as_arr());
	}
}

