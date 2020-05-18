// wengwengweng

use std::path::Path;

use crate::*;
use gfx::*;
use window::WindowCtx;

#[derive(Clone, PartialEq)]
pub struct Canvas {
	gl_fbuf: gl::Framebuffer,
	tex: Texture,
	width: i32,
	height: i32,
}

impl Canvas {

	pub fn new(ctx: &Ctx, w: i32, h: i32) -> Result<Self> {

		let dpi = ctx.window.dpi();
		let tw = (w as f32 * dpi) as i32;
		let th = (h as f32 * dpi) as i32;
		let fbuf = gl::Framebuffer::new(&ctx.device(), tw, th)?;
		let tex = Texture::from_gl_tex(fbuf.tex().clone());

		return Ok(Self {
			gl_fbuf: fbuf,
			tex: tex,
			width: w,
			height: h,
		});

	}

	// TODO: give original size
	pub fn width(&self) -> i32 {
		return self.width;
	}

	pub fn height(&self) -> i32 {
		return self.height;
	}

	pub fn tex(&self) -> &Texture {
		return &self.tex;
	}

	pub fn capture(&self, path: impl AsRef<Path>) -> Result<()> {

		return img::Image::from_raw(
			self.tex.width(),
			self.tex.height(),
			self.tex.get_pixels()
		)?
			.flip_v()
			.save(path);

	}

	pub fn resize(&mut self, ctx: &Ctx, w: i32, h: i32) -> Result<()> {

		let new = Self::new(ctx, w, h)?;
		let old = std::mem::replace(self, new);

		old.free();

		return Ok(());

	}

	pub fn free(self) {
		self.tex.free();
		self.gl_fbuf.free();
	}

	pub(crate) fn gl_fbuf(&self) -> &gl::Framebuffer {
		return &self.gl_fbuf;
	}

}

