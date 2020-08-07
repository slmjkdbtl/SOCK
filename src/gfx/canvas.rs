// wengwengweng

use super::*;

/// Off-screen Rendering Canvas
#[derive(Clone)]
pub struct Canvas {
	gl: Rc<glow::Context>,
	fbo: Rc<FramebufferHandle>,
	rbo: Rc<RenderbufferHandle>,
	tex: Texture,
	width: i32,
	height: i32,
}

impl Canvas {

	/// create a new canvas with default conf
	pub fn new(ctx: &Gfx, w: i32, h: i32) -> Result<Self> {
		return Self::new_with_conf(ctx, w, h, TextureConf::default());
	}

	/// create a new canvas
	pub fn new_with_conf(ctx: &Gfx, w: i32, h: i32, conf: TextureConf) -> Result<Self> {

		let dpi = ctx.dpi();
		let tw = (w as f32 * dpi) as i32;
		let th = (h as f32 * dpi) as i32;

		unsafe {

			let gl = ctx.gl().clone();
			let fbo = FramebufferHandle::new(gl.clone())?;

			let pixels = vec![0.0 as u8; (tw * th * 4) as usize];
			let tex = Texture::from_raw_with_conf(ctx, tw, th, &pixels, conf)?;

			let rbo = RenderbufferHandle::new(gl.clone())?;

			gl.bind_renderbuffer(glow::RENDERBUFFER, Some(rbo.id()));

			gl.renderbuffer_storage(
				glow::RENDERBUFFER,
				glow::DEPTH_STENCIL,
				tw as i32,
				th as i32,
			);

			gl.bind_renderbuffer(glow::RENDERBUFFER, None);

			let fbuf = Self {
				fbo: Rc::new(fbo),
				rbo: Rc::new(rbo),
				gl: gl,
				tex: tex,
				width: w,
				height: h,
			};

			fbuf.bind();

			fbuf.gl.clear(Surface::Color.as_glow());
			fbuf.gl.clear(Surface::Depth.as_glow());
			fbuf.gl.clear(Surface::Stencil.as_glow());

			fbuf.gl.framebuffer_texture_2d(
				glow::FRAMEBUFFER,
				glow::COLOR_ATTACHMENT0,
				glow::TEXTURE_2D,
				Some(fbuf.tex.id()),
				0,
			);

			fbuf.gl.framebuffer_renderbuffer(
				glow::FRAMEBUFFER,
				glow::DEPTH_STENCIL_ATTACHMENT,
				glow::RENDERBUFFER,
				Some(fbuf.rbo.id()),
			);

			if fbuf.gl.check_framebuffer_status(glow::FRAMEBUFFER) != glow::FRAMEBUFFER_COMPLETE {
				return Err(format!("failed to create framebuffer"));
			}

			fbuf.unbind();

			return Ok(fbuf);

		}

	}

	pub(super) fn bind(&self) {
		unsafe {
			self.gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.fbo.id()));
		}
	}

	pub(super) fn unbind(&self) {
		unsafe {
			self.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
		}
	}

	/// get canvas width
	pub fn width(&self) -> i32 {
		return self.width;
	}

	/// get canvas height
	pub fn height(&self) -> i32 {
		return self.height;
	}

	/// get canvas texture
	pub fn tex(&self) -> &Texture {
		return &self.tex;
	}

	/// capture content to an [`Image`](../img/struct.Image.html)
	pub fn capture(&self) -> Result<img::Image> {
		return Ok(self.tex.capture()?.flip_v());
	}

}

impl PartialEq for Canvas {
	fn eq(&self, other: &Self) -> bool {
		return self.fbo == other.fbo && self.rbo == other.rbo;
	}
}

