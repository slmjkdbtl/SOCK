// wengwengweng

use std::path::Path;
use std::rc::Rc;

use crate::*;
use super::*;

#[cfg(feature = "img")]
use crate::img::Image;

/// texture
#[derive(Clone, PartialEq)]
pub struct Texture {
	gl_tex: Rc<gl::Texture2D>,
}

impl Texture {

	pub(super) fn from_gl_tex(gl_tex: gl::Texture2D) -> Self {
		return Self {
			gl_tex: Rc::new(gl_tex),
		};
	}

	pub fn new(ctx: &impl GfxCtx, w: i32, h: i32) -> Result<Self> {
		return Ok(Self::from_gl_tex(gl::Texture2D::new(&ctx.gl_ctx(), w, h)?));
	}

	#[cfg(feature = "img")]
	pub fn from_img(ctx: &impl GfxCtx, img: Image) -> Result<Self> {

		let w = img.width();
		let h = img.height();

		return Self::from_pixels(ctx, w, h, &img.into_raw());

	}

	#[cfg(feature = "img")]
	pub fn from_bytes(ctx: &impl GfxCtx, data: &[u8]) -> Result<Self> {
		return Self::from_img(ctx, Image::from_bytes(data)?);
	}

	pub fn from_pixels(ctx: &impl GfxCtx, w: i32, h: i32, pixels: &[u8]) -> Result<Self> {

		let gl_tex = gl::Texture2D::from(&ctx.gl_ctx(), w, h, &pixels)?;
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

	pub(super) fn data(&self, data: &[u8]) {
		self.gl_tex.data(data);
	}

	pub(super) fn sub_data(&self, x: i32, y: i32, w: i32, h: i32, data: &[u8]) {
		self.gl_tex.sub_data(x, y, w, h, data);
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

	pub(super) fn gl_tex(&self) -> &gl::Texture2D {
		return &self.gl_tex;
	}

}

