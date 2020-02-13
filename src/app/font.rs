// wengwengweng

use std::collections::HashMap;

use super::*;
use super::gfx::*;

const ASCII_CHARS: &str = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;

pub type CharMap = HashMap<char, Quad>;

#[derive(Clone, Debug)]
pub struct FormattedLine {
	text: String,
	width: f32,
	ox: f32,
}

#[derive(Clone)]
pub struct FormattedText<'a> {
	width: f32,
	height: f32,
	lines: Vec<FormattedLine>,
	font: Option<&'a dyn gfx::Font>,
	line_spacing: f32,
	color: Color,
}

#[derive(Clone)]
pub struct Wrap {
	width: f32,
	break_word: bool,
	hyphonate: bool,
}

#[derive(Clone)]
pub struct RenderConf {
	align: gfx::Origin,
	wrap: Option<Wrap>,
	size: Option<f32>,
	line_spacing: f32,
	char_spacing: f32,
	color: Color,
}

impl Default for RenderConf {
	fn default() -> Self {
		return Self {
			align: gfx::Origin::TopLeft,
			wrap: None,
			size: None,
			line_spacing: 0.0,
			char_spacing: 0.0,
			color: rgba!(1),
		};
	}
}

pub trait Font {
	fn get(&self, ch: char) -> Option<(&gfx::Texture, Quad)>;
	fn height(&self) -> f32;
	fn format(&mut self, text: &str);
}

#[derive(Clone, Debug)]
pub struct BitmapFontData {
	pub(super) img: &'static [u8],
	pub(super) gw: u8,
	pub(super) gh: u8,
	pub(super) chars: &'static str,
}

impl BitmapFontData {
	pub const fn new(img: &'static [u8], gw: u8, gh: u8, chars: &'static str) -> Self {
		return Self {
			img: img,
			gw: gw,
			gh: gh,
			chars: chars,
		};
	}
}

#[derive(Clone, PartialEq)]
pub struct BitmapFont {
	tex: Texture,
	map: HashMap<char, Quad>,
	quad_size: Vec2,
	grid_width: u8,
	grid_height: u8,
}

impl BitmapFont {

	pub fn from_data(ctx: &impl GfxCtx, data: BitmapFontData) -> Result<Self> {

		let font_tex = gfx::Texture::from_bytes(ctx, &data.img)?;

		return Ok(Self::from_tex(
			font_tex,
			data.gw,
			data.gh,
			data.chars,
		)?);

	}

	pub fn from_tex(tex: Texture, gw: u8, gh: u8, chars: &'static str) -> Result<Self> {

		let mut map = HashMap::new();
		let tw = tex.width();
		let th = tex.height();
		let quad_size = vec2!(gw as f32 / tw as f32, gh as f32 / th as f32);
		let cols = tw / gw as i32;

		if (tw % gw as i32 != 0 || th % gh as i32 != 0) {
			return Err(format!("bitmap font grid size not correct"));
		}

		for (i, ch) in chars.chars().enumerate() {

			map.insert(ch, quad!(
				(i as i32 % cols) as f32 * quad_size.x,
				(i as i32 / cols) as f32 * quad_size.y,
				quad_size.x,
				quad_size.y
			));

		}

		return Ok(Self {
			tex: tex,
			map: map,
			quad_size: quad_size,
			grid_width: gw,
			grid_height: gh,
		});

	}

	pub fn width(&self) -> i32 {
		return self.grid_width as i32;
	}

}

impl Font for BitmapFont {
	fn get(&self, ch: char) -> Option<(&gfx::Texture, Quad)> {
		return self.map
			.get(&ch)
			.map(|quad| (&self.tex, *quad))
			;
	}
	fn height(&self) -> f32 {
		return self.grid_height as f32;
	}
	fn format(&mut self, text: &str) {}
}

pub struct TruetypeFont {
	font: fontdue::Font,
	size: i32,
	cur_pt: Pt,
	map: HashMap<char, Quad>,
	tex: Texture,
}

impl TruetypeFont {

	pub fn from_bytes(ctx: &impl GfxCtx, b: &[u8], size: i32) -> Result<Self> {

		let font = fontdue::Font::from_bytes(b)?;
		let (max_w, max_h) = (size * 32, size * 32);
		let tex = Texture::new(ctx, max_w, max_h)?;

		if size > 72 {
			return Err(format!("font size cannot exceed 72"));
		}

		return Ok(Self {
			font: font,
			size: size,
			map: HashMap::new(),
			cur_pt: pt!(0, 0),
			tex: tex,
		});

	}

	pub fn cache(&mut self, s: &str) -> Result<()> {

		let (tw, th) = (self.tex.width(), self.tex.height());

		for ch in s.chars() {

			if self.map.get(&ch).is_none() {

				let (metrics, bitmap) = self.font.rasterize(ch, self.size as f32);
				let (w, h) = (metrics.width as i32, metrics.height as i32);

				// TODO: wait for fontdue::Metrics to get fully implemented
				let (bx, by) = (metrics.bearing_x as i32, metrics.bearing_y as i32);
				let (ax, ay) = (metrics.advance_x as i32, metrics.advance_y as i32);

				let mut nbitmap = Vec::with_capacity(bitmap.len() * 4);

				for b in bitmap {
					nbitmap.extend_from_slice(&[255, 255, 255, b]);
				}

				let (mut x, mut y) = self.cur_pt.into();

				if x + w >= tw {
					x = 0;
					y += h;
				}

				if y >= th {
					return Err(format!("reached font texture size limit"));
				}

				self.tex.sub_data(x as i32, y as i32, w as i32, self.size as i32, &nbitmap);

				self.map.insert(ch, quad!(
					x as f32 / tw as f32,
					y as f32 / th as f32,
					w as f32 / tw as f32,
					h as f32 / th as f32,
				));

				x += w;
				self.cur_pt = pt!(x, y);

			}

		}

		return Ok(());

	}

	pub fn cache_asciis(&mut self) -> Result<()> {
		return self.cache(ASCII_CHARS);
	}

	pub fn width(&self, s: &str) -> i32 {
		return s
			.chars()
			.map(|c| self.map.get(&c))
			.flatten()
			.map(|q| (q.w * self.tex.width() as f32) as i32)
			.sum();
	}

	pub fn height(&self) -> i32 {
		return self.size;
	}

}

impl Font for TruetypeFont {
	fn get(&self, ch: char) -> Option<(&gfx::Texture, Quad)> {
		return self.map.get(&ch).map(|quad| (&self.tex, *quad));
	}
	fn height(&self) -> f32 {
		return self.size as f32;
	}
	fn format(&mut self, text: &str) {}
}

// TODO: 3d extruded text

