// wengwengweng

//! 3D Rendering

use std::rc::Rc;

use crate::*;
use crate::math::*;
use crate::gfx::*;
use crate::backends::gl;

const MAX_STATE_STACK: usize = 64;

include!("../res/resources.rs");

// context
ctx!(G3D: G3dCtx);

struct G3dCtx {

	projection: Mat4,
	state: State,
	state_stack: Vec<State>,
	default_shader: Shader,
	current_shader: Shader,
	empty_tex: Texture,
	renderer: Renderer,

}

pub(super) fn init() {

	let (width, height) = window::size();
	let projection = math::perspective(45f32.to_radians(), width as f32 / height as f32, 0.1, 100.0);

	let renderer = Renderer::new(CubeMesh);
	let default_shader = Shader::from_code(DEFAULT_3D_VERT, DEFAULT_3D_FRAG);

	ctx_init(G3dCtx {

		projection: projection,
		state_stack: Vec::with_capacity(MAX_STATE_STACK),
		state: State::default(),
		default_shader: default_shader.clone(),
		current_shader: default_shader.clone(),
		empty_tex: Texture::from_color(color!(1), 1, 1),
		renderer: renderer,

	});

}

struct CubeMesh;

impl Mesh for CubeMesh {

	type Vertex = CubeVert;
	const COUNT: usize = 8;

	fn push(&self, queue: &mut Vec<f32>) {

		Self::Vertex::new(vec3!(-0.5, -0.5, 0.5), color!(1, 0, 0, 1)).push(queue);
		Self::Vertex::new(vec3!(0.5, -0.5, 0.5), color!(0, 1, 0, 1)).push(queue);
		Self::Vertex::new(vec3!(0.5, 0.5, 0.5), color!(0, 0, 1, 1)).push(queue);
		Self::Vertex::new(vec3!(-0.5, 0.5, 0.5), color!(1, 1, 1, 1)).push(queue);
		Self::Vertex::new(vec3!(-0.5, -0.5, -0.5), color!(1, 0, 0, 1)).push(queue);
		Self::Vertex::new(vec3!(0.5, -0.5, -0.5), color!(0, 1, 0, 1)).push(queue);
		Self::Vertex::new(vec3!(0.5, 0.5, -0.5), color!(0, 0, 1, 1)).push(queue);
		Self::Vertex::new(vec3!(-0.5, 0.5, -0.5), color!(1, 1, 1, 1)).push(queue);

	}

	fn index() -> Vec<u32> {
		return vec![
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

struct CubeVert {

	pos: Vec3,
	color: Color,

}

impl CubeVert {
	fn new(pos: Vec3, color: Color) -> Self {
		return Self { pos, color };
	}
}

impl VertexLayout for CubeVert {

	const STRIDE: usize = 7;

	fn push(&self, queue: &mut Vec<f32>){

		queue.push(self.pos.x);
		queue.push(self.pos.y);
		queue.push(self.pos.z);
		queue.push(self.color.r);
		queue.push(self.color.g);
		queue.push(self.color.b);
		queue.push(self.color.a);

	}

	fn attr() -> Vec<gl::VertexAttr> {

		return vec![
			gl::VertexAttr::new(3, 3, 0),
			gl::VertexAttr::new(4, 4, 3),
		];

	}

}

/// check if g3d is initiated
pub fn enabled() -> bool {
	return ctx_ok();
}

#[derive(Clone, Copy)]
struct State {
	transform: Mat4,
}

impl Default for State {
	fn default() -> Self {
		return Self {
			transform: Mat4::identity(),
		}
	}
}

/// reset global transforms
pub fn reset() {
	ctx_get_mut().state = State::default();
}

pub(super) fn clear_stack() {
	ctx_get_mut().state_stack.clear();
}

/// push state
pub fn push() {

	let g3d = ctx_get_mut();
	let stack = &mut g3d.state_stack;

	if (stack.len() < MAX_STATE_STACK) {
		stack.push(g3d.state);
	} else {
		panic!("cannot push anymore");
	}

}

/// pop state
pub fn pop() {

	let mut g3d = ctx_get_mut();
	let stack = &mut g3d.state_stack;

	g3d.state = stack.pop().expect("cannot pop anymore");

}

/// global translate
pub fn translate(pos: Vec3) {

	let state = &mut ctx_get_mut().state;

	state.transform = state.transform.translate(pos);

}

/// global rotate
pub fn rotate(x: f32, y: f32, z: f32) {

	let state = &mut ctx_get_mut().state;

	if x != 0.0 {
		state.transform = state.transform.rotate(x, Dir::X);
	}

	if y != 0.0 {
		state.transform = state.transform.rotate(y, Dir::Y);
	}

	if z != 0.0 {
		state.transform = state.transform.rotate(z, Dir::Z);
	}

}

/// global scale
pub fn scale(s: Vec3) {

	let state = &mut ctx_get_mut().state;

	state.transform = state.transform.scale(s);

}

/// draw a cube
pub fn cube() {

	let gfx = ctx_get();
	let model = gfx.state.transform;
	let view = Mat4::identity();
	let projection = gfx.projection;

	gfx.current_shader.send_mat4("model", model);
	gfx.current_shader.send_mat4("view", view);
	gfx.current_shader.send_mat4("projection", projection);
	gfx.renderer.draw(&gfx.empty_tex.handle, &gfx.current_shader.program);

}

/// shader effect
#[derive(PartialEq, Clone)]
pub struct Shader {
	program: Rc<gl::Program>,
}

impl Shader {

	pub fn from_code(vert: &str, frag: &str) -> Self {

		let vert = TEMPLATE_3D_VERT.replace("###REPLACE###", vert);
		let frag = TEMPLATE_3D_FRAG.replace("###REPLACE###", frag);
		let program = gl::Program::new(&vert, &frag);

		program
			.attr(3, "vert")
			.attr(4, "color")
			.link();

		return Self {
			program: Rc::new(program),
		};

	}

	pub fn from_code_vert(vert: &str) -> Self {
		return Self::from_code(vert, DEFAULT_3D_FRAG);
	}

	pub fn from_code_frag(frag: &str) -> Self {
		return Self::from_code(DEFAULT_3D_VERT, frag);
	}

	pub fn from_file(vertf: &str, fragf: &str) -> Self {
		return Self::from_code(&fs::read_str(vertf), &fs::read_str(fragf));
	}

	pub fn from_file_vert(vertf: &str) -> Self {
		return Self::from_code(&fs::read_str(vertf), DEFAULT_3D_FRAG);
	}

	pub fn from_file_frag(fragf: &str) -> Self {
		return Self::from_code(DEFAULT_3D_VERT, &fs::read_str(fragf));
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

}

