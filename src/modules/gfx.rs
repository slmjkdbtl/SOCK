// wengwengweng

//! Rendering

use std::collections::HashMap;

use crate::*;
use crate::math::mat::Mat4;
use crate::backends::gl;

const MAX_DRAWS: usize = 65536;
const VERT_STRIDE: usize = 8;
const MAX_VERTICES: usize = MAX_DRAWS * VERT_STRIDE * 4;
const INDEX_ARRAY: [u32; 6] = [0, 1, 3, 1, 2, 3];
const MAX_INDICES: usize = MAX_DRAWS * 6;
const MAX_STATE_STACK: usize = 64;
const DEFAULT_FONT: &[u8] = include_bytes!("../res/CP437.png");
const DEFAULT_VERT_SHADER: &str = include_str!("../shaders/quad.vert");
const DEFAULT_FRAG_SHADER: &str = include_str!("../shaders/quad.frag");

// context
ctx!(GFX: GfxCtx);

struct GfxCtx {

	ibuf: gl::IndexBuffer,
	vbuf: gl::VertexBuffer,
	program: gl::Program,
	empty_tex: Texture,
	projection: Mat4,
	state: State,
	state_stack: Vec<State>,
	default_font: Font,
	current_tex: Option<&'static Texture>,
	vertex_queue: Vec<f32>,

}

#[derive(Clone, Copy)]
struct State {
	transform: Mat4,
	tint: Color,
	line_width: u8,
}

impl Default for State {
	fn default() -> Self {
		return Self {
			transform: Mat4::identity(),
			tint: color!(),
			line_width: 1,
		}
	}
}

pub(crate) fn init() {

	let indices: Vec<u32> = INDEX_ARRAY
		.iter()
		.cycle()
		.take(MAX_INDICES)
		.enumerate()
		.map(|(i, vertex)| vertex + i as u32 / 6 * 4)
		.collect();

	let vbuf = gl::VertexBuffer::new(MAX_VERTICES, VERT_STRIDE, gl::BufferUsage::Dynamic);

	vbuf
		.attr(0, 2, 0)
		.attr(1, 2, 2)
		.attr(2, 4, 4);

	let ibuf = gl::IndexBuffer::new(MAX_INDICES, gl::BufferUsage::Static);

	ibuf
		.data(&indices, 0);

	let program = gl::Program::new(
		DEFAULT_VERT_SHADER,
		DEFAULT_FRAG_SHADER,
	);

	program
		.attr(0, "pos")
		.attr(1, "uv")
		.attr(2, "color")
		.link();

	let default_font = Font::new(
		Texture::from_bytes(DEFAULT_FONT),
		32,
		8,
		r##" ☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼ !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■"##,
	);

	let (width, height) = window::size();
	let projection = Mat4::ortho(0.0, (width as f32), (height as f32), 0.0, -1.0, 1.0);

	ctx_init(GfxCtx {

		vbuf: vbuf,
		ibuf: ibuf,
		program: program,
		empty_tex: Texture::from_raw(&[255, 255, 255, 255], 1, 1),
		projection: projection,
		state_stack: Vec::with_capacity(MAX_STATE_STACK),
		state: State::default(),
		default_font: default_font,
		current_tex: None,
		vertex_queue: Vec::with_capacity(MAX_VERTICES),

	});

	gl::enable_blend();
	clear();
	window::swap();

}

/// check if gfx is initiated
pub fn enabled() -> bool {
	return ctx_is_ok();
}

/// reset global transforms and style states
pub fn reset() {

	let gfx_mut = ctx_get_mut();

	gfx_mut.state_stack.clear();
	gfx_mut.state = State::default();

}

pub(crate) fn flush() {

	let gfx = ctx_get();
	let gfx_mut = ctx_get_mut();

	if gfx.vertex_queue.is_empty() {
		return;
	}

	if let Some(tex) = &gfx.current_tex {

		gfx.program.uniform_mat4("projection", gfx.projection.as_arr());
		gfx.vbuf.data(&gfx.vertex_queue, 0);
		gl::draw(&gfx.vbuf, &gfx.ibuf, &gfx.program, &tex.handle, gfx.vertex_queue.len() / 4);
		gfx_mut.vertex_queue.clear();
		gfx_mut.current_tex = None;

	}

}

/// draw a texture with visible quad area
pub fn draw(tex: &'static Texture, quad: Rect) {

	let gfx = ctx_get();
	let gfx_mut = ctx_get_mut();
	let queue = &mut gfx_mut.vertex_queue;

	let wrapped_tex = Some(tex);

	if gfx.current_tex != wrapped_tex {
		flush();
		gfx_mut.current_tex = wrapped_tex;
	}

	let mut push_vertex = |pos: Vec2, uv: Vec2, color: Color| {

		if queue.len() >= MAX_VERTICES {
			queue.clear();
			panic!("reached maximum draw count");
		}

		queue.push(pos.x);
		queue.push(pos.y);
		queue.push(uv.x);
		queue.push(uv.y);
		queue.push(color.r);
		queue.push(color.g);
		queue.push(color.b);
		queue.push(color.a);

	};

	let t = gfx.state.transform.scale(vec2!(tex.width as f32 * quad.w, tex.height as f32 * quad.h));
	let color = gfx.state.tint;

	push_vertex(t.forward(vec2!(0, 1)), vec2!(quad.x, quad.y + quad.h), color);
	push_vertex(t.forward(vec2!(1, 1)), vec2!(quad.x + quad.w, quad.y + quad.h), color);
	push_vertex(t.forward(vec2!(1, 0)), vec2!(quad.x + quad.w, quad.y), color);
	push_vertex(t.forward(vec2!(0, 0)), vec2!(quad.x, quad.y), color);

}

/// draw text
pub fn text(s: &str) {

	let gfx = ctx_get();
	let font = &gfx.default_font;

	for (i, ch) in s.chars().enumerate() {

		push();
		translate(vec2!(i as f32 * font.grid_size.x * font.tex.width as f32, 0));

		if ch != ' ' {
			draw(&font.tex, *font.map.get(&ch).unwrap_or_else(|| panic!("does not have char '{}'", ch)));
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

	state.transform = state.transform.translate(pos);

}

/// global rotate
pub fn rotate(rot: f32) {

	let state = &mut ctx_get_mut().state;

	state.transform = state.transform.rotate(rot);

}

/// global scale
pub fn scale(s: Vec2) {

	let state = &mut ctx_get_mut().state;

	state.transform = state.transform.scale(s);


}

/// warp a 2d point through current transformed matrix
pub fn warp(pt: Vec2) -> Vec2 {

	let gfx = ctx_get();
	let trans = gfx.state.transform;

	return trans.forward(pt);

}

/// inverse warp a 2d point through current transformed matrix
pub fn inverse_warp(pt: Vec2) -> Vec2 {

	let gfx = ctx_get();
	let trans = gfx.state.transform;

	return trans.inverse().forward(pt);

}

/// clear view
pub fn clear() {
	gl::clear(Some(color!(0, 0, 0, 1)), None, None);
}

/// texture
#[derive(PartialEq)]
pub struct Texture {

	handle: gl::Texture,
	/// width
	pub width: u32,
	/// height
	pub height: u32,

}

impl Texture {

	/// create an empty texture with width and height
	pub fn new(width: u32, height: u32) -> Self {

		return Self {

			handle: gl::Texture::new(width, height),
			width: width,
			height: height,

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

}

/// bitmap font
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

		assert_eq!(tex.width % cols as u32, 0, "font size not right");
		assert_eq!(tex.height % rows as u32, 0, "font size not right");

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

