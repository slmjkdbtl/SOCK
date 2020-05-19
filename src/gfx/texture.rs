// wengwengweng

use crate::*;
use gfx::*;
use img::*;

#[derive(Clone, PartialEq)]
pub struct Texture {
	gl_tex: gl::Texture2D,
}

impl Texture {

	pub(crate) fn from_gl_tex(gl_tex: gl::Texture2D) -> Self {
		return Self {
			gl_tex: gl_tex,
		};
	}

	pub fn new(ctx: &impl HasGLDevice, w: i32, h: i32) -> Result<Self> {
		return Ok(Self::from_gl_tex(gl::Texture2D::new(&ctx.device(), w, h)?));
	}

	pub fn from_img(ctx: &impl HasGLDevice, img: Image) -> Result<Self> {

		let w = img.width();
		let h = img.height();

		return Self::from_pixels(ctx, w, h, &img.into_raw());

	}

	pub fn from_bytes(ctx: &impl HasGLDevice, data: &[u8]) -> Result<Self> {
		return Self::from_img(ctx, Image::from_bytes(data)?);
	}

	pub fn from_pixels(ctx: &impl HasGLDevice, w: i32, h: i32, pixels: &[u8]) -> Result<Self> {

		let gl_tex = gl::Texture2D::from(&ctx.device(), w, h, &pixels)?;
		return Ok(Self::from_gl_tex(gl_tex));

	}

	pub fn width(&self) -> i32 {
		return self.gl_tex.width();
	}

	pub fn height(&self) -> i32 {
		return self.gl_tex.height();
	}

	pub fn get_pixels(&self) -> Vec<u8> {
		return self.gl_tex.get_data(self.width(), self.height());
	}

	pub fn data(&self, data: &[u8]) {
		self.gl_tex.data(data);
	}

	pub fn sub_data(&self, x: i32, y: i32, w: i32, h: i32, data: &[u8]) {
		self.gl_tex.sub_data(x, y, w, h, data);
	}

	pub fn free(self) {
		self.gl_tex.free();
	}

	pub(crate) fn gl_tex(&self) -> &gl::Texture2D {
		return &self.gl_tex;
	}

}

