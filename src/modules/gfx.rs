// wengwengweng

//! 2D Rendering

use std::collections::HashMap;
use std::any::TypeId;
use std::rc::Rc;

use crate::*;
use crate::math::*;
use crate::backends::gl;

const MAX_DRAWS: usize = 65536;
const MAX_STATE_STACK: usize = 64;

include!("../res/resources.rs");

// context
ctx!(GFX: GfxCtx);

struct GfxCtx {

	projection: Mat4,
	state: State,
	state_stack: Vec<State>,
	default_shader: Shader,
	current_shader: Shader,
	default_font: Font,
	current_font: Font,
	empty_tex: Texture,
	current_canvas: Option<Canvas>,
	renderer: BatchRenderer,

}

pub(super) fn init() {

	let renderer = BatchRenderer::new::<QuadVert>(MAX_DRAWS);
	let default_shader = Shader::from_code(DEFAULT_2D_VERT, DEFAULT_2D_FRAG);

	default_shader.bind();

	let default_font = Font::new(
		Texture::from_bytes(DEFAULT_FONT),
		DEFAULT_FONT_COLS,
		DEFAULT_FONT_ROWS,
		DEFAULT_FONT_CHARS,
	);

	let (width, height) = window::size();
	let projection = Mat4::ortho(0.0, (width as f32), (height as f32), 0.0, -1.0, 1.0);

	ctx_init(GfxCtx {

		projection: projection,
		state_stack: Vec::with_capacity(MAX_STATE_STACK),
		state: State::default(),
		default_shader: default_shader.clone(),
		current_shader: default_shader,
		default_font: default_font.clone(),
		current_font: default_font,
		empty_tex: Texture::from_color(color!(1), 1, 1),
		current_canvas: None,
		renderer: renderer,

	});

	gl::set_blend(gl::BlendFac::SourceAlpha, gl::BlendFac::OneMinusSourceAlpha);
	gl::set_depth(gl::DepthFunc::LessOrEqual);
	gl::clear_color(color!(0, 0, 0, 1));
	clear();
	window::swap();

}

/// check if gfx is initiated
pub fn enabled() -> bool {
	return ctx_ok();
}

struct BatchRenderer {

	queue: Vec<f32>,
	max: usize,
	ibuf: gl::IndexBuffer,
	vbuf: gl::VertexBuffer,
	vertex_type: TypeId,
	vert_stride: usize,
	vert_count: usize,
	index_count: usize,
	current_tex: Option<Texture>,

}

impl BatchRenderer {

	fn new<V: VertexLayout + 'static>(max: usize) -> Self {

		let index = V::index();
		let vert_count = V::COUNT;
		let vert_stride = V::STRIDE;
		let max_vertices = max * vert_stride * vert_count;
		let max_indices = max * index.len();
		let queue: Vec<f32> = Vec::with_capacity(max_vertices);

		let indices: Vec<u32> = index
			.iter()
			.cycle()
			.take(max_indices)
			.enumerate()
			.map(|(i, vertex)| vertex + i as u32 / 6 * 4)
			.collect();

		let ibuf = gl::IndexBuffer::new(max_indices, gl::BufferUsage::Static);

		ibuf
			.data(&indices, 0);

		let vbuf = gl::VertexBuffer::new(max_vertices, vert_stride, gl::BufferUsage::Dynamic);

		for attr in V::attr() {
			vbuf.attr(attr);
		}

		return Self {

			queue: queue,
			max: max,
			ibuf: ibuf,
			vbuf: vbuf,
			vertex_type: TypeId::of::<V>(),
			index_count: index.len(),
			vert_stride: vert_stride,
			vert_count: vert_count,
			current_tex: None,

		};

	}

	fn push<V: VertexLayout + 'static>(&mut self, v: V) {

		if TypeId::of::<V>() != self.vertex_type {
			panic!("invalid vertex");
		}

		if self.queue.len() >= self.queue.capacity() {
			self.queue.clear();
			panic!("reached maximum draw count");
		}

		v.push(&mut self.queue);

	}

	fn update_tex(&mut self, tex: &Texture) {

		let wrapped_tex = Some(tex.clone());

		if self.current_tex != wrapped_tex {
			if self.current_tex.is_some() {
				flush();
			}
			self.current_tex = wrapped_tex;
		}

	}

	fn flush(&mut self, program: &gl::Program) {

		if self.queue.is_empty() {
			return;
		}

		if let Some(tex) = &self.current_tex {

			self.vbuf.data(&self.queue, 0);

			gl::draw(
				&self.vbuf,
				&self.ibuf,
				&program,
				&tex.handle,
				self.queue.len() / self.vert_stride / self.vert_count * self.index_count
			);

			self.queue.clear();
			self.current_tex = None;

		}

	}

}

trait VertexLayout {

	const STRIDE: usize;
	const COUNT: usize;
	fn push(&self, queue: &mut Vec<f32>);
	// wait for https://github.com/rust-lang/rust/issues/42863 to use const arrays
	fn attr() -> Vec<gl::VertexAttr>;
	fn index() -> Vec<u32>;

}

struct QuadVert {

	pos: Vec2,
	uv: Vec2,
	color: Color,

}

impl QuadVert {
	fn new(pos: Vec2, uv: Vec2, color: Color) -> Self {
		return Self { pos, uv, color };
	}
}

impl VertexLayout for QuadVert {

	const STRIDE: usize = 8;
	const COUNT: usize = 4;

	fn push(&self, queue: &mut Vec<f32>){

		queue.push(self.pos.x);
		queue.push(self.pos.y);
		queue.push(self.uv.x);
		queue.push(self.uv.y);
		queue.push(self.color.r);
		queue.push(self.color.g);
		queue.push(self.color.b);
		queue.push(self.color.a);

	}

	fn attr() -> Vec<gl::VertexAttr> {

		return vec![
			gl::VertexAttr::new(0, 2, 0),
			gl::VertexAttr::new(1, 2, 2),
			gl::VertexAttr::new(2, 4, 4),
		];

	}

	fn index() -> Vec<u32> {
		return vec![0, 1, 3, 1, 2, 3];
	}

}

#[derive(Clone, Copy)]
struct State {

	transform: Mat4,
	tint: Color,
	line_width: u8,
	text_wrap: Option<u32>,

}

impl Default for State {

	fn default() -> Self {

		return Self {

			transform: Mat4::identity(),
			tint: color!(),
			line_width: 1,
			text_wrap: None,

		}

	}

}

/// reset global transforms and style states
pub fn reset() {

	let gfx_mut = ctx_get_mut();

	gfx_mut.state = State::default();

}

pub(super) fn flush() {

	let gfx = ctx_get();
	let gfx_mut = ctx_get_mut();
	let shader = &gfx.current_shader;

	shader.send_mat4("projection", gfx.projection);
	gfx_mut.renderer.flush(&shader.program);

}

/// draw a texture with visible quad area
pub fn draw(tex: &Texture, quad: Rect) {

	let gfx = ctx_get();
	let gfx_mut = ctx_get_mut();
	let renderer = &mut gfx_mut.renderer;
	let t = gfx.state.transform.scale(vec3!(tex.width() as f32 * quad.w, tex.height() as f32 * quad.h, 1.0));
	let color = gfx.state.tint;

	renderer.update_tex(tex);
	renderer.push(QuadVert::new(t.forward(vec2!(0, 1)), vec2!(quad.x, quad.y + quad.h), color));
	renderer.push(QuadVert::new(t.forward(vec2!(1, 1)), vec2!(quad.x + quad.w, quad.y + quad.h), color));
	renderer.push(QuadVert::new(t.forward(vec2!(1, 0)), vec2!(quad.x + quad.w, quad.y), color));
	renderer.push(QuadVert::new(t.forward(vec2!(0, 0)), vec2!(quad.x, quad.y), color));

}

/// render a canvas
pub fn render(c: &Canvas) {
	draw(&c.tex, rect!(0, 0, 1, 1));

}

/// draw text
pub fn text(s: &str) {

	let gfx = ctx_get();
	let font = &gfx.default_font;
	let w = font.grid_size.x * font.tex.width() as f32;
	let h = font.grid_size.y * font.tex.height() as f32;

	let next_line = |st: &str| {

		push();
		translate(vec2!(0, h));
		text(st);
		pop();

	};

	for (i, ch) in s.chars().enumerate() {

		let x = i as f32 * w;

		if let Some(wrap) = gfx.state.text_wrap {

			if x >= wrap as f32 {
				return next_line(&s[i..s.len()]);
			}

		}

		push();
		translate(vec2!(x, 0.0));

		if ch == '\n' {

			pop();

			return next_line(&s[(i + 1) .. s.len()]);

		} else if ch != ' ' {

			let quad = font.map.get(&ch).unwrap_or_else(|| panic!("font does not contain char '{}'", ch));

			draw(&font.tex, *quad);

		}

		pop();

	}

}

/// draw rectangle with size
pub fn rect(size: Vec2) {

	let gfx = ctx_get();

	push();
	scale(size);
	draw(&gfx.empty_tex, rect!(0, 0, 1, 1));
	pop();

}

/// draw line
pub fn line(p1: Vec2, p2: Vec2) {

	let gfx = ctx_get();
	let len = ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt();
	let rot = (p2.y - p1.y).atan2(p2.x - p1.x);

	push();
	translate(p1);
	rotate(rot);
	rect(vec2!(len, gfx.state.line_width));
	pop();

}

/// draw polygon with vertices
pub fn poly(pts: &[Vec2]) {

	for (i, p) in pts.iter().enumerate() {

		if (i == pts.len() - 1) {
			line(*p, pts[0]);
		} else {
			line(*p, pts[i + 1]);
		}

	}

}

/// set global tint
pub fn color(tint: Color) {
	ctx_get_mut().state.tint = tint;
}

/// set line width
pub fn line_width(line_width: u8) {
	ctx_get_mut().state.line_width = line_width;
}

/// set text wrap
pub fn text_wrap(wrap: u32) {
	ctx_get_mut().state.text_wrap = Some(wrap);
}

/// push state
pub fn push() {

	let gfx = ctx_get_mut();
	let stack = &mut gfx.state_stack;

	if (stack.len() < MAX_STATE_STACK) {
		stack.push(gfx.state);
	} else {
		panic!("cannot push anymore");
	}

}

/// pop state
pub fn pop() {

	let mut gfx = ctx_get_mut();
	let stack = &mut gfx.state_stack;

	gfx.state = stack.pop().expect("cannot pop anymore");

}

/// global translate
pub fn translate(pos: Vec2) {

	let state = &mut ctx_get_mut().state;

	state.transform = state.transform.translate(vec3!(pos.x, pos.y, 0.0));

}

/// global rotate
pub fn rotate(rot: f32) {

	let state = &mut ctx_get_mut().state;

	state.transform = state.transform.rotate(rot, vec3!(0, 0, 1));

}

/// global scale
pub fn scale(s: Vec2) {

	let state = &mut ctx_get_mut().state;

	state.transform = state.transform.scale(vec3!(s.x, s.y, 1.0));

}

/// warp a 2d point through current transformed matrix
pub fn warp(pt: Vec2) -> Vec2 {

	let gfx = ctx_get();
	let trans = gfx.state.transform;

	return trans.forward(pt);

}

/// warp multiple points
pub fn multi_warp(pts: &[Vec2]) -> Vec<Vec2> {

	 return pts.iter()
		.map(|&p| warp(p))
		.collect();

}

/// inverse warp a 2d point through current transformed matrix
pub fn inverse_warp(pt: Vec2) -> Vec2 {

	let gfx = ctx_get();
	let trans = gfx.state.transform;

	return trans.inverse().forward(pt);

}

/// get the current transform matrix
pub fn get_matrix() -> Mat4 {
	return ctx_get().state.transform;
}

/// get the current transform matrix
pub fn set_matrix(m: Mat4) {
	ctx_get_mut().state.transform = m;
}

/// set active canvas
pub fn drawon(c: &Canvas) {

	let gfx = ctx_get_mut();
	let (width, height) = window::size();

	assert!(gfx.current_canvas.is_none(), "cannot draw on canvas while another canvas is active");

	flush();
	gfx.projection = Mat4::ortho(0.0, (width as f32), 0.0, (height as f32), -1.0, 1.0);
	gl::set_framebuffer(&*c.handle);
	gfx.current_canvas = Some(c.clone());

}

/// stop active canvas
pub fn stop_drawon(c: &Canvas) {

	let gfx = ctx_get_mut();
	let (width, height) = window::size();

	if let Some(current) = &gfx.current_canvas {

		assert!(current == c, "this is not the active canvas");

		flush();
		gfx.projection = Mat4::ortho(0.0, (width as f32), (height as f32), 0.0, -1.0, 1.0);
		gl::unset_framebuffer(&*c.handle);
		gfx.current_canvas = None;

	} else {
		panic!("no canvas active");
	}

}

/// clear view
pub fn clear() {
	gl::clear(true, true, false);
}

/// save a canvas into a png file
pub fn capture(canvas: &Canvas, fname: &str) {

	let tex = &canvas.tex;
	let buffer = tex.handle.get_data();

	image::save_buffer(
		fname,
		&buffer,
		tex.width(),
		tex.height(),
		image::ColorType::RGBA(8),
	).expect("failed to save png");

}

pub(super) fn begin() {
	clear();
}

pub(super) fn end() {

	let gfx = ctx_get();
	let gfx_mut = ctx_get_mut();

	flush();
	reset();
	gfx_mut.state_stack.clear();

	if gfx.current_canvas.is_some() {
		panic!("unfinished canvas");
	}

}

/// texture
#[derive(PartialEq, Clone)]
pub struct Texture {
	pub(super) handle: Rc<gl::Texture>,
}

impl Texture {

	/// create an empty texture with width and height
	pub fn new(width: u32, height: u32) -> Self {
		return Self {
			handle: Rc::new(gl::Texture::new(width, height)),
		};
	}

	/// create texture with raw data
	pub fn from_bytes(data: &[u8]) -> Self {

		let img = image::load_from_memory(data)
			.expect("failed to load image")
			.to_rgba();

		let width = img.width();
		let height = img.height();
		let pixels = img.into_raw();

		return Self::from_raw(&pixels, width, height);

	}

	/// create texture from pixel data, width and height
	pub fn from_raw(pixels: &[u8], width: u32, height: u32) -> Self {

		let tex = Self::new(width, height);

		tex.handle.data(pixels);

		return tex;

	}

	/// create texture from a file
	pub fn from_file(fname: &str) -> Self {
		return Self::from_bytes(&fs::read_bytes(fname));
	}

	pub fn from_color(c: Color, width: u32, height: u32) -> Self {
		return Self::from_raw(&c.to_rgba(), width, height);
	}

	/// get texture width
	pub fn width(&self) -> u32 {
		return self.handle.width;
	}

	/// get texture height
	pub fn height(&self) -> u32 {
		return self.handle.height;
	}

}

/// bitmap font
#[derive(PartialEq, Clone)]
pub struct Font {

	tex: Texture,
	map: HashMap<char, Rect>,
	grid_size: Vec2,

}

impl Font {

	/// creat a bitmap font from a texture, and grid of characters
	pub fn new(tex: Texture, cols: usize, rows: usize, chars: &str) -> Self {

		let mut map = HashMap::new();
		let grid_size = vec2!(1.0 / cols as f32, 1.0 / rows as f32);

		assert!(tex.width() % cols as u32 == 0, "font size not right");
		assert!(tex.height() % rows as u32 == 0, "font size not right");

		for (i, ch) in chars.chars().enumerate() {

			map.insert(ch, rect!(

				(i % cols) as f32 * grid_size.x,
				(i / cols) as f32 * grid_size.y,
				grid_size.x,
				grid_size.y

			));

		}

		return Self {

			tex: tex,
			map: map,
			grid_size: grid_size,

		}

	}

}

/// offscreen framebuffer
#[derive(PartialEq, Clone)]
pub struct Canvas {

	handle: Rc<gl::Framebuffer>,
	tex: Texture,
	width: u32,
	height: u32,

}

impl Canvas {

	/// create new canvas
	pub fn new(width: u32, height: u32) -> Self {

		let handle = gl::Framebuffer::new();
		let pixels = vec![0.0 as u8; (width * height * 4) as usize];
		let tex = Texture::from_raw(&pixels, width, height);

		handle.attach(&*tex.handle);

		return Self {
			handle: Rc::new(handle),
			tex: tex,
			width: width,
			height: height,
		}

	}

}

pub fn effect(s: &Shader) {

	let gfx_mut = ctx_get_mut();

	flush();
	gfx_mut.current_shader = s.clone();

}

pub fn stop_effect(s: &Shader) {

	let gfx = ctx_get_mut();

	assert!(gfx.current_shader == *s, "this is not the active shader effect");
	flush();
	effect(&ctx_get().default_shader);

}

/// shader effect
#[derive(PartialEq, Clone)]
pub struct Shader {
	program: Rc<gl::Program>,
}

impl Shader {

	pub fn from_code(vert: &str, frag: &str) -> Self {

		let vert = TEMPLATE_2D_VERT.replace("###REPLACE###", vert);
		let frag = TEMPLATE_2D_FRAG.replace("###REPLACE###", frag);

		let program = gl::Program::new(&vert, &frag);

		program
			.attr(0, "vert")
			.attr(1, "uv")
			.attr(2, "color")
			.link();

		return Self {
			program: Rc::new(program),
		};

	}

	pub fn from_code_vert(vert: &str) -> Self {
		return Self::from_code(vert, DEFAULT_2D_FRAG);
	}

	pub fn from_code_frag(frag: &str) -> Self {
		return Self::from_code(DEFAULT_2D_VERT, frag);
	}

	pub fn from_file(vertf: &str, fragf: &str) -> Self {
		return Self::from_code(&fs::read_str(vertf), &fs::read_str(fragf));
	}

	pub fn from_file_vert(vertf: &str) -> Self {
		return Self::from_code(&fs::read_str(vertf), DEFAULT_2D_FRAG);
	}

	pub fn from_file_frag(fragf: &str) -> Self {
		return Self::from_code(DEFAULT_2D_VERT, &fs::read_str(fragf));
	}

	pub fn send_float(&self, name: &str, f: f32) -> &Self {
		self.program.uniform_float(name, f);
		return self;
	}

	pub fn send_vec2(&self, name: &str, v: Vec2) -> &Self {
		self.program.uniform_vec2(name, v);
		return self;
	}

	pub fn send_vec3(&self, name: &str, v: Vec3) -> &Self {
		self.program.uniform_vec3(name, v);
		return self;
	}

	pub fn send_vec4(&self, name: &str, v: Vec4) -> &Self {
		self.program.uniform_vec4(name, v);
		return self;
	}

	pub fn send_mat4(&self, name: &str, v: Mat4) -> &Self {
		self.program.uniform_mat4(name, v.as_arr());
		return self;
	}

	pub fn send_color(&self, name: &str, c: Color) -> &Self {
		self.program.uniform_color(name, c);
		return self;
	}

	pub fn send_rect(&self, name: &str, r: Rect) -> &Self {
		self.program.uniform_rect(name, r);
		return self;
	}

	fn bind(&self) {
		self.program.bind();
	}

}

