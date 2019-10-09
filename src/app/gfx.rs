// wengwengweng

use std::mem;
use std::ops;
use std::rc::Rc;
use std::collections::HashMap;
use std::path::Path;
use std::io::Cursor;

#[cfg(feature = "img")]
use crate::img::Image;

use crate::*;
use crate::math::*;
use super::*;

pub use gl::VertexLayout;
pub use gl::Shape;

pub use gl::UniformType;
pub use gl::UniformValues;
pub use gl::FilterMode;
pub use gl::Surface;
pub use gl::Cmp;

pub trait Gfx {

	// clearing
	fn clear(&mut self);
	fn clear_ex(&mut self, s: Surface);
	fn clear_color(&self, c: Color);

	// stats
	fn draw_calls(&self) -> usize;

	// drawing
	fn draw(&mut self, t: impl Drawable) -> Result<()>;
	fn draw_on(&mut self, canvas: &Canvas, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;
	fn draw_2d_with(&mut self, shader: &Shader2D, uniform: &impl Uniform, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;
	fn draw_3d_with(&mut self, shader: &Shader3D, uniform: &impl Uniform, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;
	fn draw_masked(&mut self, mask: impl FnOnce(&mut Self) -> Result<()>, draw: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;
	fn draw_masked_ex(&mut self, f1: Cmp, f2: Cmp, mask: impl FnOnce(&mut Self) -> Result<()>, draw: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;

	// transform
	fn push(&mut self, t: &Transform, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;

	// coord
	fn coord(&self, coord: Origin) -> Vec2;

	// camera
	fn use_cam(&mut self, cam: &impl Camera, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;

}

impl Gfx for Ctx {

	fn clear(&mut self) {

		flush(self);
		self.gl.clear(Surface::Color);
		self.gl.clear(Surface::Depth);
		self.gl.clear(Surface::Stencil);

	}

	fn clear_ex(&mut self, s: Surface) {

		flush(self);
		self.gl.clear(s);

	}

	fn clear_color(&self, c: Color) {
		self.gl.clear_color(c);
	}

	fn draw_calls(&self) -> usize {
		return self.draw_calls_last;
	}

	fn push(&mut self, t: &Transform, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let ot = self.transform;

		self.transform = ot.apply(t);
		f(self)?;
		self.transform = ot;

		return Ok(());

	}

	fn draw(&mut self, thing: impl Drawable) -> Result<()> {
		return thing.draw(self);
	}

	fn draw_on(&mut self, canvas: &Canvas, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		if self.cur_canvas.is_some() {
			return Err(Error::Gfx(format!("cannot use canvas inside a canvas call")));
		}

		flush(self);

		let o_proj_2d = self.proj_2d;
		let o_proj_3d = self.proj_3d;
		let t = self.transform;

		self.gl.viewport(0, 0, canvas.width(), canvas.height());
		self.cur_canvas = Some(canvas.clone());

		self.proj_2d = flip_matrix(&o_proj_2d);
		self.proj_3d = flip_matrix(&o_proj_3d);
		self.transform = Transform::new();

		canvas.handle.with(|| -> Result<()> {
			f(self)?;
			return Ok(());
		})?;

		flush(self);

		self.transform = t;
		self.proj_2d = o_proj_2d;
		self.proj_3d = o_proj_3d;

		self.cur_canvas = None;
		self.gl.viewport(0, 0, self.width() * self.dpi() as i32, self.height() * self.dpi() as i32);

		return Ok(());

	}

	fn draw_2d_with(&mut self, shader: &Shader2D, uniform: &impl Uniform, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		flush(self);
		self.cur_shader_2d = shader.clone();
		self.cur_shader_2d.handle.send(&uniform.values());
		f(self)?;
		flush(self);
		self.cur_shader_2d = self.default_shader_2d.clone();

		return Ok(());

	}

	fn draw_3d_with(&mut self, shader: &Shader3D, uniform: &impl Uniform, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		flush(self);
		self.cur_shader_3d = shader.clone();
		self.cur_shader_3d.handle.send(&uniform.values());
		f(self)?;
		flush(self);
		self.cur_shader_3d = self.default_shader_3d.clone();

		return Ok(());

	}

	fn draw_masked(&mut self, mask: impl FnOnce(&mut Self) -> Result<()>, draw: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {
		return self.draw_masked_ex(Cmp::Never, Cmp::Equal, mask, draw);
	}

	// TODO: use gl::StencilDraw
	fn draw_masked_ex(&mut self, f1: Cmp, f2: Cmp, mask: impl FnOnce(&mut Self) -> Result<()>, draw: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

// 		let d1 = gl::StencilDraw {
// 			ops: gl::StencilOps {
// 				sfail: gl::StencilOp::Replace,
// 				dpfail: gl::StencilOp::Replace,
// 				dppass: gl::StencilOp::Replace,
// 			},
// 			func: gl::StencilFunc {
// 				cmp: gl::Cmp::Never,
// 				rf: 1,
// 				mask: 0xff,
// 			},
// 		};

		flush(self);
		self.gl.clear(gl::Surface::Stencil);
		self.gl.enable(gl::Capability::StencilTest);
		self.gl.stencil_func(f1);
		self.gl.stencil_op(gl::StencilOp::Replace, gl::StencilOp::Replace, gl::StencilOp::Replace);

		mask(self)?;
		flush(self);
		self.gl.stencil_func(f2);
		self.gl.stencil_op(gl::StencilOp::Keep, gl::StencilOp::Keep, gl::StencilOp::Keep);
		draw(self)?;
		flush(self);
		self.gl.disable(gl::Capability::StencilTest);

		return Ok(());

	}

	fn coord(&self, coord: Origin) -> Vec2 {

		let w = self.width();
		let h = self.height();
		let orig_pt = self.conf.origin.as_pt();
		let coord_pt = coord.as_pt();

		return (coord_pt - orig_pt) / 2.0 * vec2!(w, h);

	}

	fn use_cam(&mut self, cam: &impl Camera, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let oview_3d = self.view_3d;
		let oproj_3d = self.proj_3d;

		self.view_3d = cam.lookat();
		self.proj_3d = cam.projection();

		if self.cur_canvas.is_some() {
			self.proj_3d = flip_matrix(&self.proj_3d);
		}

		f(self)?;

		self.view_3d = oview_3d;
		self.proj_3d = oproj_3d;

		return Ok(());

	}

}

fn flip_matrix(m: &Mat4) -> Mat4 {

	let mut nm = m.clone();

	if let Some(val) = nm.get_mut(1, 1) {
		*val = -*val;
	}

	if let Some(val) = nm.get_mut(3, 1) {
		*val = -*val;
	}

	return nm;

}

pub(super) fn begin(ctx: &mut Ctx) {

	ctx.draw_calls_last = ctx.draw_calls;
	ctx.draw_calls = 0;
	ctx.clear();

}

pub(super) fn end(ctx: &mut Ctx) {

	flush(ctx);
	ctx.transform = Transform::new();
	ctx.draw_calls += ctx.renderer_2d.draw_count();
	ctx.draw_calls += ctx.renderer_3d.draw_count();
	ctx.renderer_2d.clear();
	ctx.renderer_3d.clear();

}

pub(super) fn flush(ctx: &mut Ctx) {
	ctx.renderer_2d.flush();
	ctx.renderer_3d.flush();
}

#[derive(Clone)]
pub struct Vertex2D {
	pos: Vec2,
	uv: Vec2,
	color: Color,
}

impl Vertex2D {
	pub fn new(pos: Vec2, uv: Vec2, color: Color) -> Self {
		return Self {
			pos: pos,
			uv: uv,
			color: color,
		};
	}
}

impl VertexLayout for Vertex2D {

	const STRIDE: usize = 8;

	fn push(&self, queue: &mut Vec<f32>) {
		queue.extend_from_slice(&[
			self.pos.x,
			self.pos.y,
			self.uv.x,
			self.uv.y,
			self.color.r,
			self.color.g,
			self.color.b,
			self.color.a,
		]);
	}

	fn attrs() -> gl::VertexAttrGroup {
		return &[
			("pos", 2),
			("uv", 2),
			("color", 4),
		];
	}

}

#[derive(Clone, PartialEq)]
pub(super) struct Uniform2D {
	pub proj: Mat4,
	pub tex: Texture,
}

impl gl::UniformLayout for Uniform2D {
	fn values(&self) -> UniformValues {
		return vec![
			("proj", self.proj.into()),
		];
	}
	fn texture(&self) -> Option<&gl::Texture> {
		return Some(&self.tex.handle);
	}
}

#[derive(Clone)]
pub struct Vertex3D {
	pos: Vec3,
	uv: Vec2,
	normal: Vec3,
	color: Color,
}

impl Vertex3D {
	pub fn new(pos: Vec3, uv: Vec2, normal: Vec3, color: Color) -> Self {
		return Self {
			pos: pos,
			uv: uv,
			normal: normal,
			color: color,
		};
	}
}

impl VertexLayout for Vertex3D {

	const STRIDE: usize = 12;

	fn push(&self, queue: &mut Vec<f32>) {
		queue.extend_from_slice(&[
			self.pos.x,
			self.pos.y,
			self.pos.z,
			self.uv.x,
			self.uv.y,
			self.normal.x,
			self.normal.y,
			self.normal.z,
			self.color.r,
			self.color.g,
			self.color.b,
			self.color.a,
		]);
	}

	fn attrs() -> gl::VertexAttrGroup {
		return &[
			("pos", 3),
			("uv", 2),
			("normal", 3),
			("color", 4),
		];
	}

}

#[derive(Clone, PartialEq)]
pub(super) struct Uniform3D {
	pub proj: Mat4,
	pub view: Mat4,
	pub model: Transform,
	pub color: Color,
	pub tex: Texture,
}

impl gl::UniformLayout for Uniform3D {

	fn values(&self) -> UniformValues {
		return vec![
			("proj", self.proj.into()),
			("view", self.view.into()),
			("model", self.model.as_mat4().into()),
			("color", self.color.into()),
		];
	}

	fn texture(&self) -> Option<&gl::Texture> {
		return Some(&self.tex.handle);
	}

}

pub(super) struct QuadShape {
	pub transform: Mat4,
	pub quad: Quad,
	pub color: Color,
	pub origin: Origin,
	pub flip: Flip,
}

impl QuadShape {
	pub fn new(t: Mat4, q: Quad, c: Color, o: Origin, f: Flip) -> Self {
		return Self {
			transform: t,
			quad: q,
			color: c,
			origin: o,
			flip: f,
		};
	}
}

impl Shape for QuadShape {

	type Vertex = Vertex2D;
	const COUNT: usize = 4;

	fn vertices(&self, queue: &mut Vec<f32>) {

		let t = self.transform;
		let q = self.quad;
		let c = self.color;
		let offset = self.origin.as_pt() * 0.5;
		let p1 = t * (vec2!(-0.5, 0.5) - offset);
		let p2 = t * (vec2!(0.5, 0.5) - offset);
		let p3 = t * (vec2!(0.5, -0.5) - offset);
		let p4 = t * (vec2!(-0.5, -0.5) - offset);

		let mut u1 = vec2!(q.x, q.y + q.h);
		let mut u2 = vec2!(q.x + q.w, q.y + q.h);
		let mut u3 = vec2!(q.x + q.w, q.y);
		let mut u4 = vec2!(q.x, q.y);

		match self.flip {
			Flip::X => {
				mem::swap(&mut u1, &mut u2);
				mem::swap(&mut u4, &mut u3);
			},
			Flip::Y => {
				mem::swap(&mut u2, &mut u3);
				mem::swap(&mut u1, &mut u4);
			},
			Flip::XY => {
				mem::swap(&mut u2, &mut u4);
				mem::swap(&mut u1, &mut u3);
			},
			_ => {},
		}

		Self::Vertex::new(p1, u1, c).push(queue);
		Self::Vertex::new(p2, u2, c).push(queue);
		Self::Vertex::new(p3, u3, c).push(queue);
		Self::Vertex::new(p4, u4, c).push(queue);

	}

	fn indices() -> &'static [u32] {
		return &[0, 1, 3, 1, 2, 3];
	}

}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Flip {
	None,
	X,
	Y,
	XY,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Origin {
	TopLeft,
	Top,
	TopRight,
	Left,
	Center,
	Right,
	BottomLeft,
	Bottom,
	BottomRight,
}

impl Origin {

	pub fn to_ortho(&self, w: i32, h: i32) -> Mat4 {

		use Origin::*;

		let w = w as f32;
		let h = h as f32;
		let near = -1.0;
		let far = 1.0;

		return match self {
			TopLeft => ortho(0.0, w, h, 0.0, near, far),
			Top => ortho(-w / 2.0, w / 2.0, h, 0.0, near, far),
			TopRight => ortho(-w, 0.0, h, 0.0, near, far),
			Left => ortho(0.0, w, h / 2.0, -h / 2.0, near, far),
			Center => ortho(-w / 2.0, w / 2.0, h / 2.0, -h / 2.0, near, far),
			Right => ortho(-w, 0.0, h / 2.0, -h / 2.0, near, far),
			BottomLeft => ortho(0.0, w, 0.0, -h, near, far),
			Bottom => ortho(-w / 2.0, w / 2.0, 0.0, -h, near, far),
			BottomRight => ortho(-w, 0.0, 0.0, -h, near, far),
		};

	}

	pub fn as_pt(&self) -> Vec2 {

		use Origin::*;

		return match self {
			TopLeft => vec2!(-1, -1),
			Top => vec2!(0, -1),
			TopRight => vec2!(1, -1),
			Left => vec2!(-1, 0),
			Center => vec2!(0, 0),
			Right => vec2!(1, 0),
			BottomLeft => vec2!(-1, 1),
			Bottom => vec2!(0, 1),
			BottomRight => vec2!(1, 1),
		};

	}

}

#[derive(Clone, PartialEq)]
pub struct Texture {
	pub(super) handle: Rc<gl::Texture>,
	width: i32,
	height: i32,
}

impl Texture {

	pub(super) fn from_handle(handle: gl::Texture, w: i32, h: i32) -> Self {
		return Self {
			handle: Rc::new(handle),
			width: w,
			height: h,
		};
	}

	pub fn new(ctx: &Ctx, w: i32, h: i32,) -> Result<Self> {
		return Ok(Self::from_handle(gl::Texture::new(&ctx.gl, w, h)?, w, h));
	}

	#[cfg(feature = "img")]
	pub fn from_img(ctx: &Ctx, img: Image) -> Result<Self> {

		let w = img.width();
		let h = img.height();
		let handle = gl::Texture::from(&ctx.gl, w, h, &img.into_raw())?;

		handle.filter(ctx.conf.texture_filter);

		return Ok(Self::from_handle(handle, w as i32, h as i32));

	}

	#[cfg(feature = "img")]
	pub fn from_file(ctx: &Ctx, path: impl AsRef<Path>) -> Result<Self> {
		return Self::from_img(ctx, Image::from_file(path)?);
	}

	#[cfg(feature = "img")]
	pub fn from_bytes(ctx: &Ctx, data: &[u8]) -> Result<Self> {
		return Self::from_img(ctx, Image::from_bytes(data)?);
	}

	pub fn from_pixels(ctx: &Ctx, w: i32, h: i32, pixels: &[u8]) -> Result<Self> {

		let handle = gl::Texture::from(&ctx.gl, w, h, &pixels)?;
		handle.filter(ctx.conf.texture_filter);
		return Ok(Self::from_handle(handle, w, h));

	}

	pub fn width(&self) -> i32 {
		return self.width;
	}

	pub fn height(&self) -> i32 {
		return self.height;
	}

	pub fn get_pixels(&self) -> Vec<u8> {
		return self.handle.get_data(self.width(), self.height());
	}

	#[cfg(feature = "img")]
	pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {

		image::save_buffer(
			path,
			&self.get_pixels(),
			self.width() as u32,
			self.height() as u32,
			image::ColorType::RGBA(8),
		)?;

		return Ok(());

	}

	pub fn data(&mut self, x: i32, y: i32, width: i32, height: i32, data: &[u8]) {
		self.width = width;
		self.height = height;
		self.handle.data(x, y, width, height, data);
	}

}

pub enum TextAlign {
	Left,
	Center,
	Right,
}

/// bitmap font
#[derive(Clone, PartialEq)]
pub struct BitmapFont {

	pub(super) tex: Texture,
	pub(super) map: HashMap<char, Quad>,
	pub(super) quad_size: Vec2,
	grid_width: i32,
	grid_height: i32,

}

impl BitmapFont {

	/// creat a bitmap font from a texture, and grid of characters
	pub fn from_tex(tex: Texture, cols: usize, rows: usize, chars: &str) -> Result<Self> {

		let mut map = HashMap::new();
		let quad_size = vec2!(1.0 / cols as f32, 1.0 / rows as f32);
		let tw = tex.width() as i32;
		let th = tex.height() as i32;

		if (tw % cols as i32 != 0 || th % rows as i32 != 0) {
			return Err(Error::Gfx("bitmap font texture size or column / row count not correct".into()));
		}

		for (i, ch) in chars.chars().enumerate() {

			map.insert(ch, quad!(

				(i % cols) as f32 * quad_size.x,
				(i / cols) as f32 * quad_size.y,
				quad_size.x,
				quad_size.y

			));

		}

		return Ok(Self {

			tex: tex,
			map: map,
			quad_size: quad_size,
			grid_width: tw as i32 / cols as i32,
			grid_height: th as i32 / rows as i32,

		});

	}

	/// get current font width for string
	pub fn width(&self) -> i32 {
		return self.grid_width;
	}

	/// get current text height
	pub fn height(&self) -> i32 {
		return self.grid_height;
	}

}

pub trait Uniform {
	fn values(&self) -> UniformValues;
}

#[derive(Clone, PartialEq)]
pub struct Shader2D {
	pub(super) handle: Rc<gl::Pipeline<Vertex2D, Uniform2D>>,
}

impl Shader2D {

	pub(super) fn from_handle(handle: gl::Pipeline<Vertex2D, Uniform2D>) -> Self {
		return Self {
			handle: Rc::new(handle),
		};
	}

	pub fn effect(ctx: &Ctx, frag: &str) -> Result<Self> {

		let vert_src = TEMPLATE_2D_VERT.replace("###REPLACE###", DEFAULT_2D_VERT);
		let frag_src = TEMPLATE_2D_FRAG.replace("###REPLACE###", frag);

		return Self::from_code(ctx, &vert_src, &frag_src);

	}

	pub fn from_code(ctx: &Ctx, vert: &str, frag: &str) -> Result<Self> {
		return Ok(Self::from_handle(gl::Pipeline::new(&ctx.gl, vert, frag)?));
	}

}

#[derive(Clone, PartialEq)]
pub struct Shader3D {
	pub(super) handle: Rc<gl::Pipeline<Vertex3D, Uniform3D>>,
}

impl Shader3D {

	pub(super) fn from_handle(handle: gl::Pipeline<Vertex3D, Uniform3D>) -> Self {
		return Self {
			handle: Rc::new(handle),
		};
	}

	pub fn effect(ctx: &Ctx, frag: &str) -> Result<Self> {

		let vert_src = TEMPLATE_3D_VERT.replace("###REPLACE###", DEFAULT_3D_VERT);
		let frag_src = TEMPLATE_3D_FRAG.replace("###REPLACE###", frag);

		return Self::from_code(ctx, &vert_src, &frag_src);

	}

	pub fn from_code(ctx: &Ctx, vert: &str, frag: &str) -> Result<Self> {
		return Ok(Self::from_handle(gl::Pipeline::new(&ctx.gl, vert, frag)?));
	}

}

#[derive(Clone, PartialEq)]
pub struct Canvas {

	pub(super) handle: Rc<gl::Framebuffer>,
	pub(super) tex: Texture,

}

impl Canvas {

	pub fn new(ctx: &Ctx, width: i32, height: i32) -> Result<Self> {

		let dpi = ctx.dpi();
		let tw = (width as f64 * dpi) as i32;
		let th = (height as f64 * dpi) as i32;
		let pixels = vec![0.0 as u8; (tw * th * 4) as usize];
		let tex = Texture::from_pixels(&ctx, tw, th, &pixels)?;
		let handle = gl::Framebuffer::new(&ctx.gl, &tex.handle)?;

		return Ok(Self {
			handle: Rc::new(handle),
			tex: tex,
		});

	}

	pub fn width(&self) -> i32 {
		return self.tex.width();
	}

	pub fn height(&self) -> i32 {
		return self.tex.height();
	}

	#[cfg(feature = "img")]
	pub fn capture(&self, path: impl AsRef<Path>) -> Result<()> {
		return self.tex.save(path);
	}

}

pub trait Camera {
	fn projection(&self) -> Mat4;
	fn lookat(&self) -> Mat4;
}

#[derive(Clone)]
pub struct PerspectiveCam {
	front: Vec3,
	pos: Vec3,
	yaw: f32,
	pitch: f32,
	fov: f32,
	aspect: f32,
	near: f32,
	far: f32,
}

impl PerspectiveCam {

	pub fn new(fov: f32, aspect: f32, near: f32, far: f32, pos: Vec3, yaw: f32, pitch: f32) -> Self {

		let mut c = Self {
			pos: vec3!(),
			front: vec3!(),
			yaw: 0.0,
			pitch: 0.0,
			fov: fov,
			aspect: aspect,
			near: near,
			far: far,
		};

		c.set_pos(pos);
		c.set_angle(yaw, pitch);

		return c;

	}

	pub fn set_pos(&mut self, pos: Vec3) {
		self.pos = pos;
	}

	pub fn set_front(&mut self, front: Vec3) {
		self.front = front;
	}

	pub fn set_angle(&mut self, yaw: f32, pitch: f32) {

		self.yaw = yaw;
		self.pitch = pitch;

		self.front = vec3!(
			self.pitch.cos() * (self.yaw + 90f32.to_radians()).cos(),
			self.pitch.sin(),
			self.pitch.cos() * (self.yaw + 90f32.to_radians()).sin(),
		).normalize();

	}

	pub fn front(&self) -> Vec3 {
		return self.front;
	}

	pub fn pos(&self) -> Vec3 {
		return self.pos;
	}

	pub fn yaw(&self) -> f32 {
		return self.yaw;
	}

	pub fn pitch(&self) -> f32 {
		return self.pitch;
	}

}

impl Camera for PerspectiveCam {
	fn projection(&self) -> Mat4 {
		return math::perspective(self.fov.to_radians(), self.aspect, self.near, self.far);
	}
	fn lookat(&self) -> Mat4 {
		return math::lookat(self.pos, self.pos + self.front, vec3!(0, 1, 0));
	}
}

#[derive(Clone)]
pub struct OrthoCam {
	front: Vec3,
	pos: Vec3,
	yaw: f32,
	pitch: f32,
	width: f32,
	height: f32,
	near: f32,
	far: f32,
}

impl OrthoCam {

	pub fn new(width: f32, height: f32, near: f32, far: f32, pos: Vec3, yaw: f32, pitch: f32) -> Self {

		let mut c = Self {
			pos: vec3!(),
			front: vec3!(),
			yaw: 0.0,
			pitch: 0.0,
			width: width,
			height: height,
			near: near,
			far: far,
		};

		c.set_pos(pos);
		c.set_angle(yaw, pitch);

		return c;

	}

	pub fn set_pos(&mut self, pos: Vec3) {
		self.pos = pos;
	}

	pub fn set_front(&mut self, front: Vec3) {
		self.front = front;
	}

	pub fn set_angle(&mut self, yaw: f32, pitch: f32) {

		self.yaw = yaw;
		self.pitch = pitch;

		self.front = vec3!(
			self.pitch.cos() * (self.yaw + 90f32.to_radians()).cos(),
			self.pitch.sin(),
			self.pitch.cos() * (self.yaw + 90f32.to_radians()).sin(),
		).normalize();

	}

	pub fn front(&self) -> Vec3 {
		return self.front;
	}

	pub fn pos(&self) -> Vec3 {
		return self.pos;
	}

	pub fn yaw(&self) -> f32 {
		return self.yaw;
	}

	pub fn pitch(&self) -> f32 {
		return self.pitch;
	}

}

impl Camera for OrthoCam {
	fn projection(&self) -> Mat4 {
		return math::ortho(-self.width / 2.0, self.width / 2.0, self.height / 2.0, -self.height / 2.0, self.near, self.far);
	}
	fn lookat(&self) -> Mat4 {
		return math::lookat(self.pos, self.pos + self.front, vec3!(0, 1, 0));
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NormalMode {
	Vertex,
	Surface,
}

// TODO: not correct
fn gen_vertex_normals(pos: &[f32], indices: &[u32]) -> Vec<Vec3> {

	let vert_count = pos.len() / 3;
	let tri_count = indices.len() / 3;
	let mut normals = vec![vec3!(0); vert_count];

	for i in 0..tri_count {

		let i1 = indices[i * 3] as usize;
		let i2 = indices[i * 3 + 1] as usize;
		let i3 = indices[i * 3 + 2] as usize;
		let v1 = vec3!(pos[i1 * 3], pos[i1 * 3 + 1], pos[i1 * 3 + 2]);
		let v2 = vec3!(pos[i2 * 3], pos[i2 * 3 + 1], pos[i2 * 3 + 2]);
		let v3 = vec3!(pos[i3 * 3], pos[i3 * 3 + 1], pos[i3 * 3 + 2]);
		let normal = Vec3::cross((v3 - v1), (v2 - v1)).normalize();

		normals[i1] += normal;
		normals[i2] += normal;
		normals[i3] += normal;

	}

	for n in &mut normals {
		*n = (*n / 3.0).normalize();
	}

	return normals;

}

// TODO: messy
#[derive(Clone)]
pub struct Model {
	pub(super) renderer: Rc<gl::Renderer<Vertex3D, Uniform3D>>,
}

impl Model {

	fn from_tobj(ctx: &Ctx, tobj: tobj::LoadResult) -> Result<Self> {

		let (models, _) = tobj?;
		let mesh = &models.get(0).ok_or(Error::Obj("no mesh found".into()))?.mesh;
		let positions = &mesh.positions;
		let indices = &mesh.indices;
		let vert_count = positions.len() / 3;
		let normals = gen_vertex_normals(&positions, &indices);
		let mut verts = Vec::with_capacity(vert_count * Vertex3D::STRIDE);

		for i in 0..vert_count {

			let vx = positions[i * 3 + 0];
			let vy = positions[i * 3 + 1];
			let vz = positions[i * 3 + 2];
			let vert = Vertex3D::new(vec3!(vx, vy, vz), vec2!(), normals[i], color!(rand!(), rand!(), rand!(), 1));

			vert.push(&mut verts);

		}

		let renderer = gl::Renderer::new(&ctx.gl, &verts, indices)?;

		return Ok(Self {
			renderer: Rc::new(renderer),
		});

	}

	pub fn from_obj(ctx: &Ctx, obj: &str) -> Result<Self> {
		return Self::from_tobj(ctx, tobj::load_obj_buf(&mut Cursor::new(obj), |_| {
			return Err(tobj::LoadError::GenericFailure);
		}));
	}

	pub fn from_obj_with_mtl(ctx: &Ctx, obj: &str, mtl: &str) -> Result<Self> {
		return Self::from_tobj(ctx, tobj::load_obj_buf(&mut Cursor::new(obj), |_| {
			return tobj::load_mtl_buf(&mut Cursor::new(mtl));
		}));
	}

	pub fn from_obj_file(ctx: &Ctx, path: impl AsRef<Path>) -> Result<Self> {
		return Self::from_tobj(ctx, tobj::load_obj(path.as_ref()));
	}

}

// TODO: messy
pub(super) struct FlagShape {
	pub transform: Mat4,
	pub quad: Quad,
	pub color: Color,
	pub origin: Origin,
	pub flip: Flip,
}

impl FlagShape {
	pub fn new(t: Mat4, q: Quad, c: Color, o: Origin, f: Flip) -> Self {
		return Self {
			transform: t,
			quad: q,
			color: c,
			origin: o,
			flip: f,
		};
	}
}

impl Shape for FlagShape {

	type Vertex = Vertex3D;
	const COUNT: usize = 4;

	fn vertices(&self, queue: &mut Vec<f32>) {

		let t = self.transform;
		let q = self.quad;
		let c = self.color;
		let offset = self.origin.as_pt() * 0.5;
		let p1 = t * (vec2!(-0.5, 0.5) - offset);
		let p2 = t * (vec2!(0.5, 0.5) - offset);
		let p3 = t * (vec2!(0.5, -0.5) - offset);
		let p4 = t * (vec2!(-0.5, -0.5) - offset);

		let mut u1 = vec2!(q.x, q.y + q.h);
		let mut u2 = vec2!(q.x + q.w, q.y + q.h);
		let mut u3 = vec2!(q.x + q.w, q.y);
		let mut u4 = vec2!(q.x, q.y);

		match self.flip {
			Flip::X => {
				mem::swap(&mut u1, &mut u2);
				mem::swap(&mut u4, &mut u3);
			},
			Flip::Y => {
				mem::swap(&mut u2, &mut u3);
				mem::swap(&mut u1, &mut u4);
			},
			Flip::XY => {
				mem::swap(&mut u2, &mut u4);
				mem::swap(&mut u1, &mut u3);
			},
			_ => {},
		}

		Self::Vertex::new(vec3!(p1.x, p1.y, 0.0), u1, vec3!(0, 0, -1), c).push(queue);
		Self::Vertex::new(vec3!(p2.x, p2.y, 0.0), u2, vec3!(0, 0, -1), c).push(queue);
		Self::Vertex::new(vec3!(p3.x, p3.y, 0.0), u3, vec3!(0, 0, -1), c).push(queue);
		Self::Vertex::new(vec3!(p4.x, p4.y, 0.0), u4, vec3!(0, 0, -1), c).push(queue);

	}

	fn indices() -> &'static [u32] {
		return &[0, 1, 3, 1, 2, 3];
	}

}

pub(super) struct CubeShape;

impl Shape for CubeShape {

	type Vertex = Vertex3D;
	const COUNT: usize = 8;

	fn vertices(&self, queue: &mut Vec<f32>) {

		Self::Vertex::new(vec3!(-0.5, -0.5, 0.5), vec2!(), vec3!(0.33, 0.33, -0.66), color!(1, 0, 0, 1)).push(queue);
		Self::Vertex::new(vec3!(0.5, -0.5, 0.5), vec2!(), vec3!(-0.66, 0.66, -0.33), color!(0, 1, 0, 1)).push(queue);
		Self::Vertex::new(vec3!(0.5, 0.5, 0.5), vec2!(), vec3!(-0.33, -0.33, -0.66), color!(0, 0, 1, 1)).push(queue);
		Self::Vertex::new(vec3!(-0.5, 0.5, 0.5), vec2!(), vec3!(0.66, -0.66, -0.33), color!(1, 1, 1, 1)).push(queue);
		Self::Vertex::new(vec3!(-0.5, -0.5, -0.5), vec2!(), vec3!(0.66, 0.66, 0.33), color!(1, 0, 0, 1)).push(queue);
		Self::Vertex::new(vec3!(0.5, -0.5, -0.5), vec2!(), vec3!(-0.33, 0.33, 0.66), color!(0, 1, 0, 1)).push(queue);
		Self::Vertex::new(vec3!(0.5, 0.5, -0.5), vec2!(), vec3!(-0.66, -0.66, 0.33), color!(0, 0, 1, 1)).push(queue);
		Self::Vertex::new(vec3!(-0.5, 0.5, -0.5), vec2!(), vec3!(0.33, -0.33, 0.66), color!(1, 1, 1, 1)).push(queue);

// 		Self::Vertex::new(vec3!(-0.5, -0.5, 0.5), vec3!(0.33, 0.33, -0.66), color!(1)).push(queue);
// 		Self::Vertex::new(vec3!(0.5, -0.5, 0.5), vec3!(-0.66, 0.66, -0.33), color!(1)).push(queue);
// 		Self::Vertex::new(vec3!(0.5, 0.5, 0.5), vec3!(-0.33, -0.33, -0.66), color!(1)).push(queue);
// 		Self::Vertex::new(vec3!(-0.5, 0.5, 0.5), vec3!(0.66, -0.66, -0.33), color!(1)).push(queue);
// 		Self::Vertex::new(vec3!(-0.5, -0.5, -0.5), vec3!(0.66, 0.66, 0.33), color!(1)).push(queue);
// 		Self::Vertex::new(vec3!(0.5, -0.5, -0.5), vec3!(-0.33, 0.33, 0.66), color!(1)).push(queue);
// 		Self::Vertex::new(vec3!(0.5, 0.5, -0.5), vec3!(-0.66, -0.66, 0.33), color!(1)).push(queue);
// 		Self::Vertex::new(vec3!(-0.5, 0.5, -0.5), vec3!(0.33, -0.33, 0.66), color!(1)).push(queue);

	}

	fn indices() -> &'static [u32] {
		return &[
			0, 1, 2,
			2, 3, 0,
			1, 5, 6,
			6, 2, 1,
			7, 6, 5,
			5, 4, 7,
			4, 0, 3,
			3, 7, 4,
			4, 5, 1,
			1, 0, 4,
			3, 2, 6,
			6, 7, 3,
		];
	}

}

use glyph_brush::GlyphBrush;
use glyph_brush::GlyphBrushBuilder;

#[derive(Clone)]
pub(super) struct FontQuad {
	pub(super) pos: Vec2,
	pub(super) quad: Quad,
}

// TODO: messy
pub struct TrueTypeFont {
	pub(super) cache: GlyphBrush<'static, FontQuad>,
	pub(super) tex: Texture,
	pub(super) quads: Vec<FontQuad>,
	pub(super) size: f32,
}

impl TrueTypeFont {

	pub fn new(ctx: &Ctx, bytes: &'static [u8], size: f32) -> Result<Self> {

		let font_cache = GlyphBrushBuilder::using_font_bytes(bytes).build();

		let (width, height) = font_cache.texture_dimensions();
		let font_cache_texture = gl::Texture::new(&ctx.gl, width as i32, height as i32)?;

		return Ok(Self {
			cache: font_cache,
			tex: Texture::from_handle(font_cache_texture, width as i32, height as i32),
			quads: Vec::with_capacity(64),
			size: size,
		})

	}

	pub fn push(&mut self, txt: &str) {

		use glyph_brush::BrushAction;
		use glyph_brush::Section;
		use glyph_brush::rusttype;

		let mut tex = self.tex.clone();

		self.cache.queue(Section {
			text: txt,
			scale: rusttype::Scale::uniform(self.size),
			..Section::default()
		});

		let mut update_texture = |rect: rusttype::Rect<u32>, data: &[u8]| {

			let mut padded_data = Vec::with_capacity(data.len() * 4);

			for a in data {
				padded_data.extend_from_slice(&[
					255,
					255,
					255,
					*a,
				]);
			}

			tex.data(
				rect.min.x as i32,
				rect.min.y as i32,
				rect.width() as i32,
				rect.height() as i32,
				&padded_data,
			);

		};

		let into_vertex = |verts: &glyph_brush::GlyphVertex| {

			let uv = verts.tex_coords;
			let pos = verts.pixel_coords.min;
			let x = uv.min.x;
			let y = uv.min.y;
			let w = uv.max.x - x;
			let h = uv.max.y - y;

			return FontQuad {
				pos: vec2!(pos.x, pos.y),
				quad: quad!(x, y, w, h),
			}

		};

		if let Ok(action) = self.cache.process_queued(
			|rect, tex_data| update_texture(rect, tex_data),
			|verts| into_vertex(&verts),
		) {

			if let BrushAction::Draw(quads) = action {
				self.quads = quads;
			}

		}

	}

}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Transform {
	matrix: Mat4,
}

impl Transform {

	pub fn new() -> Self {
		return Self::from_mat4(Mat4::identity());
	}

	pub fn from_mat4(m: Mat4) -> Self {
		return Self {
			matrix: m,
		};
	}

	pub fn translate(&self, p: Vec2) -> Self {
		return Self::from_mat4(self.matrix * Mat4::translate(vec3!(p.x, p.y, 0.0)));
	}

	pub fn rotate(&self, a: f32) -> Self {
		return Self::from_mat4(self.matrix * Mat4::rotate(a, vec3!(0, 0, 1)));
	}

	pub fn scale(&self, s: Vec2) -> Self {
		return Self::from_mat4(self.matrix * Mat4::scale(vec3!(s.x, s.y, 1.0)));
	}

	pub fn translate_3d(&self, p: Vec3) -> Self {
		return Self::from_mat4(self.matrix * Mat4::translate(p));
	}

	pub fn scale_3d(&self, s: Vec3) -> Self {
		return Self::from_mat4(self.matrix * Mat4::scale(s));
	}

	pub fn rotate_x(&self, a: f32) -> Self {
		return Self::from_mat4(self.matrix *  Mat4::rotate(a, vec3!(1, 0, 0)));
	}

	pub fn rotate_y(&self, a: f32) -> Self {
		return Self::from_mat4(self.matrix *  Mat4::rotate(a, vec3!(0, 1, 0)));
	}

	pub fn rotate_z(&self, a: f32) -> Self {
		return Self::from_mat4(self.matrix *  Mat4::rotate(a, vec3!(0, 0, 1)));
	}

	pub fn as_mat4(&self) -> Mat4 {
		return self.matrix;
	}

	pub fn invert(&self) -> Self {
		return Self::from_mat4(self.matrix.invert());
	}

	pub fn apply(self, other: &Self) -> Self {
		return Self::from_mat4(self.matrix * other.matrix);
	}

}

impl ops::Mul<Vec4> for Transform {
	type Output = Vec4;
	fn mul(self, pt: Self::Output) -> Self::Output {
		return self.matrix * pt;
	}
}

impl ops::Mul<Vec3> for Transform {
	type Output = Vec3;
	fn mul(self, pt: Self::Output) -> Self::Output {
		return self.matrix * pt;
	}
}

impl ops::Mul<Vec2> for Transform {
	type Output = Vec2;
	fn mul(self, pt: Self::Output) -> Self::Output {
		return self.matrix * pt;
	}
}


pub fn t() -> Transform {
	return Transform::new();
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineJoin {
	None,
	Round,
	Bevel,
	Miter,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineCap {
	Square,
	Butt,
	Round,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Fill {
	NoFill,
	Solid(Color),
	Gradient(Vec2, Vec2, Vec<(Color, f32)>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stroke {
	NoStroke,
	Solid(Color, LineJoin),
}

pub trait Drawable {
	fn draw(&self, ctx: &mut Ctx) -> Result<()>;
}

