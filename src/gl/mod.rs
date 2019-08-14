// wengwengweng

export!(types);
export!(texture);
export!(program);
export!(vbuf);
export!(ibuf);
export!(fbuf);
export!(attr);
export!(vao);
export!(uniform);
export!(stencil);
export!(renderer);
export!(batched_renderer);

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
type FramebufferID = <GLCtx as Context>::Framebuffer;
type VertexArrayID = <GLCtx as Context>::VertexArray;

// TODO: clean up this mess

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

	pub fn clear(&self, buf: Surface) {
		unsafe {
			self.ctx.clear(buf.into());
		}
	}

	pub fn stencil<F: Fn()>(&self, ops: &[StencilDraw<F>]) {

		self.clear(Surface::Stencil);
		self.enable(Capability::StencilTest);

		unsafe {
			for o in ops {
				self.ctx.stencil_func(o.func.cmp.into(), o.func.rf, o.func.mask);
				self.ctx.stencil_op(o.ops.sfail.into(), o.ops.dpfail.into(), o.ops.dppass.into());
				(o.draw)();
			}
		}

		self.disable(Capability::StencilTest);

	}

	pub fn stencil_op(&self, sfail: StencilOp, dpfail: StencilOp, dppass: StencilOp) {
		unsafe {
			self.ctx.stencil_op(sfail.into(), dpfail.into(), dppass.into());
		}
	}

	pub fn stencil_func(&self, f: Cmp) {
		unsafe {
			self.ctx.stencil_func(f.into(), 1, 0xff);
		}
	}

	pub fn cull_face(&self, face: Face) {
		unsafe {
			self.ctx.cull_face(face.into());
		}
	}

	pub fn front_face(&self, dir: Dir) {
		unsafe {
			self.ctx.front_face(dir.into());
		}
	}

	pub fn viewport(&self, x: u32, y: u32, width: u32, height: u32) {
		unsafe {
			self.ctx.viewport(x as i32, y as i32, width as i32, height as i32);
		}
	}

}

#[cfg(feature="gl3")]
fn draw(ctx: &GLCtx, vao: &VertexArray, ibuf: &IndexBuffer, program: &Program, count: u32, mode: Primitive) {

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
fn draw<V: VertexLayout>(ctx: &GLCtx, vbuf: &VertexBuffer<V>, ibuf: &IndexBuffer, program: &Program, count: u32, mode: Primitive) {

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

