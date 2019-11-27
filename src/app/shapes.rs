// wengwengweng

//! Common Drawing Primitives

use std::f32::consts::PI;

use super::*;
use gfx::Drawable;
use gl::VertexLayout;

#[derive(Clone)]
pub struct Sprite<'a> {
	tex: &'a gfx::Texture,
	quad: Quad,
	offset: Option<Vec2>,
	flip: gfx::Flip,
	color: Color,
}

impl<'a> Sprite<'a> {
	pub fn new(tex: &'a gfx::Texture) -> Self {
		return Self {
			tex: tex,
			quad: quad!(0, 0, 1, 1),
			color: rgba!(1),
			offset: None,
			flip: gfx::Flip::None,
		};
	}
	pub fn quad(mut self, quad: Quad) -> Self {
		self.quad = quad;
		return self;
	}
	pub fn color(mut self, color: Color) -> Self {
		self.color = color;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		self.color.a = a;
		return self;
	}
	pub fn offset(mut self, offset: Vec2) -> Self {
		self.offset = Some(offset);
		return self;
	}
	pub fn flip(mut self, flip: gfx::Flip) -> Self {
		self.flip = flip;
		return self;
	}
}

pub fn sprite<'a>(tex: &'a gfx::Texture) -> Sprite<'a> {
	return Sprite::new(tex);
}

impl<'a> Drawable for Sprite<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let scale = vec2!(self.tex.width(), self.tex.height()) * vec2!(self.quad.w, self.quad.h);
		let offset = self.offset.unwrap_or(ctx.conf.origin.as_pt());

		// TODO: extremely slow
		let t = ctx.transform
			.s2(scale)
			.t2(offset * -0.5)
			;

		let shape = gfx::QuadShape {
			transform: t.as_mat4(),
			quad: self.quad,
			color: self.color,
			flip: self.flip,
		};

		ctx.renderer_2d.push_shape(
			gl::Primitive::Triangle,
			shape,
			&ctx.cur_pipeline_2d,
			&gfx::Uniform2D {
				proj: ctx.proj_2d,
				tex: self.tex.clone(),
				custom: ctx.cur_custom_uniform_2d.clone(),
			}
		)?;

		return Ok(());

	}

}

#[derive(Clone, Copy, Debug)]
pub enum FillMode {
	Stretch,
	Tiled,
}

// TODO: support other origins
#[derive(Clone)]
pub struct TexFill<'a> {
	p1: Vec2,
	p2: Vec2,
	tex: &'a gfx::Texture,
	mode: FillMode,
	color: Color,
}

impl<'a> TexFill<'a> {
	pub fn new(p1: Vec2, p2: Vec2, tex: &'a gfx::Texture) -> Self {
		let lx = f32::min(p1.x, p2.x);
		let mx = f32::max(p1.x, p2.x);
		let ly = f32::min(p1.y, p2.y);
		let my = f32::max(p1.y, p2.y);
		return Self {
			p1: vec2!(lx, ly),
			p2: vec2!(mx, my),
			tex: tex,
			mode: FillMode::Tiled,
			color: rgba!(1),
		};
	}
	pub fn color(mut self, color: Color) -> Self {
		self.color = color;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		self.color.a = a;
		return self;
	}
	pub fn mode(mut self, m: FillMode) -> Self {
		self.mode = m;
		return self;
	}
}

pub fn texfill<'a>(p1: Vec2, p2: Vec2, tex: &'a gfx::Texture) -> TexFill<'a> {
	return TexFill::new(p1, p2, tex);
}

impl<'a> gfx::Drawable for TexFill<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let tw = self.tex.width() as f32;
		let th = self.tex.height() as f32;
		let pw = self.p2.x - self.p1.x;
		let ph = self.p2.y - self.p1.y;

		ctx.push(&gfx::t().t2(self.p1), |ctx| {

			match self.mode {

				FillMode::Stretch => {},
				FillMode::Tiled => {

					let cw = pw / tw;
					let ch = ph / th;

					for i in 0..f32::ceil(cw) as i32 {

						for j in 0..f32::ceil(ch) as i32 {

							let i = i as f32;
							let j = j as f32;

							let qw = if (cw) - i < 1.0 {
								(cw) - i
							} else {
								1.0
							};

							let qh = if (ch) - j < 1.0 {
								(ch) - j
							} else {
								1.0
							};

							ctx.draw_t(
								&gfx::t()
									.t2(vec2!(i * tw, j * th))
									,
								&sprite(&self.tex)
									.color(self.color)
									.offset(vec2!(-1, -1))
									.quad(quad!(0, 0, qw, qh))
									,
							)?;

						}

					}

				},

			}

			return Ok(());

		})?;

		return Ok(());

	}

}

#[derive(Clone)]
pub struct Text<'a> {
	content: &'a str,
	font: Option<&'a dyn gfx::Font>,
	color: Color,
	align: Option<gfx::Origin>,
	wrap: Option<f32>,
	line_height: f32,
}

impl<'a> Text<'a> {
	pub fn new(s: &'a str) -> Self {
		return Self {
			content: s,
			font: None,
			align: None,
			color: rgba!(1),
			wrap: None,
			line_height: 0.0,
		};
	}
	pub fn font(mut self, f: &'a dyn gfx::Font) -> Self {
		self.font = Some(f);
		return self;
	}
	pub fn color(mut self, color: Color) -> Self {
		self.color = color;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		self.color.a = a;
		return self;
	}
	pub fn align(mut self, o: gfx::Origin) -> Self {
		self.align = Some(o);
		return self;
	}
	pub fn wrap(mut self, wrap: f32) -> Self {
		self.wrap = Some(wrap);
		return self;
	}
	pub fn line_height(mut self, h: f32) -> Self {
		self.line_height = h;
		return self;
	}
}

pub struct RenderedLine {
	text: String,
	width: f32,
}

pub struct RenderedText<'a> {
	width: f32,
	height: f32,
	lines: Vec<RenderedLine>,
	align: gfx::Origin,
	font: Option<&'a dyn gfx::Font>,
	line_height: f32,
	color: Color,
}

impl<'a> RenderedText<'a> {

	pub fn width(&self) -> f32 {
		return self.width;
	}

	pub fn height(&self) -> f32 {
		return self.height;
	}

	pub fn cursor_pos(&self, ctx: &Ctx, cpos: i32) -> Option<Vec2> {

		if cpos == 0 {
			return Some(vec2!());
		}

		let font = self.font.unwrap_or(&ctx.default_font);
		let offset = (self.align.as_pt() + vec2!(1)) * 0.5;
		let gh = font.height() + self.line_height;
		let mut tl = 0;

		for (y, line) in self.lines.iter().enumerate() {

			tl += line.text.len() as i32;

			if cpos > tl {
				continue;
			} else {

				let mut x = 0.0;
				let ox = (self.width - line.width) * offset.x;
				let ccpos = cpos - tl + line.text.len() as i32 - 1;

				for (i, ch) in line.text.chars().enumerate() {

					if let Some((tex, quad)) = font.get(ch) {

						let gw = tex.width() as f32 * quad.w;
						x += gw;

						if i as i32 == ccpos {
							return Some(vec2!(x + ox, y as f32 * gh));
						}

					}
				}

			}

		}

		return None;

	}

}

impl<'a> Text<'a> {

	pub fn render(&self, ctx: &Ctx) -> RenderedText<'a> {

		// TODO: \n for new line?
		let font = self.font.unwrap_or(&ctx.default_font);
		let gh = font.height() + self.line_height;
		let mut lines = vec![];

		let (pw, ph) = {

			let (mut pw, mut ph) = (0.0, 0.0);
			let mut l = String::new();

			ph += gh;

			for ch in self.content.chars() {

				if let Some((tex, quad)) = font.get(ch) {

					let gw = tex.width() as f32 * quad.w;

					if let Some(wrap) = self.wrap {

						if pw + gw > wrap {

							lines.push(RenderedLine {
								text: std::mem::replace(&mut l, String::new()),
								width: pw,
							});

							pw = 0.0;
							ph += gh;
							pw += gw;
							l.push(ch);

						} else {
							pw += gw;
							l.push(ch);
						}

					} else {

						l.push(ch);
						pw += gw;

					}

				}

			}

			lines.push(RenderedLine {
				text: l,
				width: pw,
			});

			if let Some(wrap) = self.wrap {
				(wrap, ph)
			} else {
				(pw, ph)
			}

		};

		return RenderedText {
			width: pw,
			height: ph,
			lines: lines,
			align: self.align.unwrap_or(ctx.conf.origin),
			font: self.font,
			line_height: self.line_height,
			color: self.color,
		};

	}

}

pub fn text<'a>(s: &'a str) -> Text<'a> {
	return Text::new(s);
}

impl<'a> Drawable for RenderedText<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let dfont = ctx.default_font.clone();
		let font = self.font.unwrap_or(&dfont);
		let gh = font.height() + self.line_height;

		let offset = (self.align.as_pt() + vec2!(1)) * 0.5;
		let offset_pos = -offset * vec2!(self.width, self.height);

		ctx.push(&gfx::t()
			.t2(offset_pos)
		, |ctx| {

			for (y, line) in self.lines.iter().enumerate() {

				let mut x = 0.0;
				let ox = (self.width - line.width) * offset.x;

				for ch in line.text.chars() {

					if let Some((tex, quad)) = font.get(ch) {

						let gw = tex.width() as f32 * quad.w;

						ctx.draw_t(&gfx::t()
							.t2(vec2!(x + ox, y as f32 * gh))
						, &shapes::sprite(&tex)
							.offset(vec2!(-1))
							.quad(quad)
							.color(self.color)
						)?;

						x += gw;

					}

				}

			}

			return Ok(());

		})?;

		return Ok(());

	}

}

impl<'a> Drawable for Text<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw(&self.render(ctx))?;

		return Ok(());

	}

}

#[derive(Debug, Clone, Copy, PartialEq)]
struct LineDash {
	len: f32,
	interval: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineJoin {
	None,
	Round,
	Bevel,
	Miter,
}

#[derive(Debug, Clone, Copy)]
pub enum LineCap {
	Square,
	Butt,
	Round,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Stroke {
	width: f32,
	join: LineJoin,
	dash: Option<LineDash>,
	color: Color,
}

#[derive(Clone)]
pub struct Polygon {
	pts: Vec<Vec2>,
	fill: Option<Color>,
	stroke: Option<Stroke>,
	radius: Option<f32>,
}

impl Polygon {
	pub fn from_pts(pts: &[Vec2]) -> Self {
		return Self {
			pts: pts.to_vec(),
			fill: Some(rgba!()),
			stroke: None,
			radius: None,
		};
	}
	pub fn fill(mut self, c: Color) -> Self {
		self.fill = Some(c);
		return self;
	}
	pub fn no_fill(mut self) -> Self {
		self.fill = None;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		if let Some(fill) = &mut self.fill {
			fill.a = a;
		}
		if let Some(stroke) = &mut self.stroke {
			stroke.color.a = a;
		}
		return self;
	}
	pub fn stroke(mut self, c: Color) -> Self {
		self.stroke = Some(Stroke {
			width: 1.0,
			join: LineJoin::None,
			dash: None,
			color: c,
		});
		return self
	}
	pub fn line_join(mut self, j: LineJoin) -> Self {
		if let Some(stroke) = &mut self.stroke {
			stroke.join = j;
		}
		return self;
	}
	pub fn line_width(mut self, w: f32) -> Self {
		if let Some(stroke) = &mut self.stroke {
			stroke.width = w;
		}
		return self;
	}
	pub fn radius(mut self, r: f32) -> Self {
		self.radius = Some(r);
		return self
	}
}

pub fn polygon(pts: &[Vec2]) -> Polygon {
	return Polygon::from_pts(pts);
}

impl Drawable for Polygon {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		if self.pts.len() < 3 {
			return Ok(());
		}

		use std::borrow::Cow;

		let pts = if let Some(radius) = self.radius {
			Cow::Owned(rounded_poly_verts(&self.pts, radius, None))
		} else {
			Cow::Borrowed(&self.pts)
		};

		if let Some(color) = self.fill {

			let mut verts = Vec::with_capacity(pts.len() * gfx::Vertex2D::STRIDE);
			let mut indices = Vec::with_capacity((pts.len() - 2) * 3);

			for (i, p) in pts.iter().enumerate() {

				gfx::Vertex2D {
					pos: ctx.transform * vec3!(p.x, p.y, 0.0),
					uv: vec2!(0),
					color: color,
				}.push(&mut verts);

				if i >= 2 {
					indices.extend_from_slice(&[0, (i as u32 - 1), i as u32]);
				}

			}

			ctx.renderer_2d.push(
				gl::Primitive::Triangle,
				&verts,
				&indices,
				&ctx.cur_pipeline_2d,
				&gfx::Uniform2D {
					proj: ctx.proj_2d,
					tex: ctx.empty_tex.clone(),
					custom: ctx.cur_custom_uniform_2d.clone(),
				}
			)?;

		}

		if let Some(stroke) = &self.stroke {

			// TODO: line join
			for i in 0..pts.len() {

				let p1 = pts[i];
				let p2 = pts[(i + 1) % pts.len()];

				use LineJoin::*;

				match stroke.join {
					None => {
						ctx.draw(&line(p1, p2).width(stroke.width).color(stroke.color))?;
					},
					Bevel => {
						// TODO
						ctx.draw(&line(p1, p2).width(stroke.width).color(stroke.color))?;
					},
					Miter => {
						// TODO
						ctx.draw(&line(p1, p2).width(stroke.width).color(stroke.color))?;
					},
					Round => {
						ctx.draw(&line(p1, p2).width(stroke.width).color(stroke.color).cap(LineCap::Round))?;
					},
				}

			}

		}

		return Ok(());

	}

}

#[derive(Clone)]
pub struct Gradient {
	p1: Vec2,
	p2: Vec2,
	steps: Vec<(Color, f32)>,
	width: f32,
}

impl Gradient {
	pub fn from(p1: Vec2, p2: Vec2, steps: &[(Color, f32)]) -> Gradient {
		return Self {
			p1: p1,
			p2: p2,
			steps: steps.to_vec(),
			width: 1.0,
		};
	}
	pub fn width(mut self, w: f32) -> Self {
		self.width = w;
		return self;
	}
}

pub fn gradient(p1: Vec2, p2: Vec2, steps: &[(Color, f32)]) -> Gradient {
	return Gradient::from(p1, p2, steps);
}

impl Drawable for Gradient {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		if self.steps.len() < 2 {
			return Err(Error::Gfx("need at least 2 points to draw a gradient".into()));
		}

		use gfx::Vertex2D;

		let rot = (self.p2.y - self.p1.y).atan2(self.p2.x - self.p1.x);
		let mut verts = Vec::with_capacity(4 + 2 * (self.steps.len() - 2) * gfx::Vertex2D::STRIDE);

		let matrix = ctx.transform
			.t2((self.p1 + self.p2) * 0.5)
			.r(rot - 90f32.to_radians())
			.as_mat4();

		let w = self.width;
		let h = Vec2::dis(self.p1, self.p2);

		let mut last_pos = None;

		for s in &self.steps {

			if (last_pos.is_none()) {
				if (s.1 != 0.0) {
					return Err(Error::Gfx("gradient step should start at 0.0".into()));
				}
			}

			last_pos = Some(s.1);

			Vertex2D {
				pos: matrix * vec3!(-w / 2.0, -h / 2.0 + h * s.1, 0.0),
				uv: vec2!(0),
				color: s.0,
			}.push(&mut verts);

			Vertex2D {
				pos: matrix * vec3!(w / 2.0, -h / 2.0 + h * s.1, 0.0),
				uv: vec2!(0),
				color: s.0,
			}.push(&mut verts);

		}

		if (last_pos != Some(1.0)) {
			return Err(Error::Gfx("gradient step should end at 1.0".into()));
		}

		let indices = [
			0, 1, 2,
			1, 2, 3,
		];

		let indices: Vec<u32> = indices
			.iter()
			.cycle()
			.take((self.steps.len() - 1) * indices.len())
			.enumerate()
			.map(|(i, vertex)| vertex + i as u32 / 6 * 2 )
			.collect();

		ctx.renderer_2d.push(
			gl::Primitive::Triangle,
			&verts,
			&indices,
			&ctx.cur_pipeline_2d,
			&gfx::Uniform2D {
				proj: ctx.proj_2d,
				tex: ctx.empty_tex.clone(),
				custom: ctx.cur_custom_uniform_2d.clone(),
			}
		)?;

		return Ok(());

	}

}

#[derive(Clone)]
pub struct Rect {
	p1: Vec2,
	p2: Vec2,
	radius: Option<f32>,
	fill: Option<Color>,
	stroke: Option<Stroke>,
}

impl Rect {
	pub fn from_pts(p1: Vec2, p2: Vec2) -> Self {
		return Self {
			p1: p1,
			p2: p2,
			radius: None,
			stroke: None,
			fill: Some(rgba!(1)),
		};
	}
	pub fn from_size(w: f32, h: f32) -> Self {
		return Self::from_pts(vec2!(w, h) * -0.5, vec2!(w, h) * 0.5);
	}
	pub fn radius(mut self, r: f32) -> Self {
		self.radius = Some(r);
		return self
	}
	pub fn fill(mut self, c: Color) -> Self {
		self.fill = Some(c);
		return self;
	}
	pub fn no_fill(mut self) -> Self {
		self.fill = None;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		if let Some(fill) = &mut self.fill {
			fill.a = a;
		}
		if let Some(stroke) = &mut self.stroke {
			stroke.color.a = a;
		}
		return self;
	}
	pub fn stroke(mut self, c: Color) -> Self {
		self.stroke = Some(Stroke {
			width: 1.0,
			join: LineJoin::None,
			dash: None,
			color: c,
		});
		return self
	}
	pub fn line_join(mut self, j: LineJoin) -> Self {
		if let Some(stroke) = &mut self.stroke {
			stroke.join = j;
		}
		return self;
	}
	pub fn line_width(mut self, w: f32) -> Self {
		if let Some(stroke) = &mut self.stroke {
			stroke.width = w;
		}
		return self;
	}
}

pub fn rect(p1: Vec2, p2: Vec2) -> Rect {
	return Rect::from_pts(p1, p2);
}

impl Drawable for Rect {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let pts = vec![
			self.p1,
			vec2!(self.p2.x, self.p1.y),
			self.p2,
			vec2!(self.p1.x, self.p2.y),
		];

		let poly = Polygon {
			pts: pts.to_vec(),
			fill: self.fill,
			stroke: self.stroke.clone(),
			radius: self.radius,
		};

		ctx.draw(&poly)?;

		return Ok(());

	}

}

#[derive(Clone)]
pub struct Line {
	p1: Vec2,
	p2: Vec2,
	width: f32,
	color: Color,
	cap: LineCap,
	dash: Option<LineDash>,
}

impl Line {
	pub fn from(p1: Vec2, p2: Vec2) -> Self {
		return Self {
			p1: p1,
			p2: p2,
			width: 1.0,
			color: rgba!(1),
			cap: LineCap::Butt,
			dash: None,
		};
	}
	pub fn width(mut self, w: f32) -> Self {
		self.width = w;
		return self;
	}
	pub fn color(mut self, color: Color) -> Self {
		self.color = color;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		self.color.a = a;
		return self;
	}
	pub fn cap(mut self, c: LineCap) -> Self {
		self.cap = c;
		return self;
	}
	pub fn dashed(mut self, len: f32, interval: f32) -> Self {
		self.dash = Some(LineDash {
			len: len,
			interval: interval,
		});
		return self;
	}
}

pub fn line(p1: Vec2, p2: Vec2) -> Line {
	return Line::from(p1, p2);
}

impl Drawable for Line {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		if let Some(dash) = self.dash {

			let diff = self.p2 - self.p1;
			let nd = diff.normalize();
			let len = diff.mag();
			let mut l = 0.0;
			let mut nxt_p1 = self.p1;

			loop {

				let p1 = nxt_p1;
				let mut p2 = nxt_p1 + nd * dash.len;

				l += dash.len;

				if l >= len {
					p2 = self.p2;
				}

				ctx.draw(&Line {
					p1: p1,
					p2: p2,
					width: self.width,
					color: self.color,
					cap: self.cap,
					dash: None,
				})?;

				nxt_p1 = p2 + nd * dash.interval;
				l += dash.interval;

				if l >= len {
					break;
				}

			}

		} else {

			let len = (self.p2 - self.p1).mag();
			let rot = (self.p2.y - self.p1.y).atan2(self.p2.x - self.p1.x);

			ctx.push(&gfx::t()

				.t2((self.p1 + self.p2) * 0.5)
				.r(rot)

			, |ctx| {

				let w = len;
				let h = self.width;

				ctx.draw(&Rect::from_size(w, h).fill(self.color))?;

				if let LineCap::Round = self.cap {
					ctx.draw(&circle(vec2!(-w / 2.0, 0), h / 2.0).fill(self.color))?;
					ctx.draw(&circle(vec2!(w / 2.0, 0), h / 2.0).fill(self.color))?;
				}

				return Ok(());

			})?;

		}

		return Ok(());

	}

}

impl splines::interpolate::Linear<f32> for Vec2 {
	fn outer_mul(self, t: f32) -> Self {
		return self * t;
	}
	fn outer_div(self, t: f32) -> Self {
		return self / t;
	}
}

impl splines::Interpolate<f32> for Vec2 {
	fn lerp(a: Self, b: Self, t: f32) -> Self {
		return a * (1. - t) + b * t;
	}

	fn cubic_hermite(x: (Self, f32), a: (Self, f32), b: (Self, f32), y: (Self, f32), t: f32) -> Self {
		return splines::interpolate::cubic_hermite_def(x, a, b, y, t);
	}

	fn quadratic_bezier(a: Self, u: Self, b: Self, t: f32) -> Self {
		return splines::interpolate::quadratic_bezier_def(a, u, b, t);
	}

	fn cubic_bezier(a: Self, u: Self, v: Self, b: Self, t: f32) -> Self {
		return splines::interpolate::cubic_bezier_def(a, u, v, b, t);
	}
}

pub use splines::Interpolation as Interp;

// TODO
#[derive(Clone)]
pub struct Spline {
	dt: f32,
	spline: splines::Spline<f32, Vec2>,
}

impl Spline {

	pub fn from_pts(pts: &[(f32, Vec2)]) -> Self {

		use splines::Key;

		let keys = pts
			.iter()
			.map(|(t, p)| Key::new(*t, *p, Interp::Cosine))
			.collect();

		let spline = splines::Spline::from_vec(keys);

		return Self {
			dt: 0.1,
			spline: spline,
		};

	}

}

impl Drawable for Spline {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let mut step = 0.0;
		let mut samples = vec![];

		while step <= 1.0 {
			if let Some(sample) = self.spline.sample(step) {
				samples.push(sample);
			}
			step += self.dt;
		}

		for i in 0..samples.len() - 1 {
			ctx.draw(&line(samples[i], samples[i + 1]))?;
		}

		return Ok(());

	}

}

#[derive(Debug, Clone, Copy)]
pub enum PointMode {
	Rect,
	Circle,
}

#[derive(Clone)]
pub struct Points<'a> {
	pts: &'a[Vec2],
	size: f32,
	mode: PointMode,
	color: Color,
}

impl<'a> Points<'a> {
	pub fn from(pts: &'a[Vec2]) -> Self {
		return Self {
			pts: pts,
			size: 1.0,
			color: rgba!(1),
			mode: PointMode::Rect,
		};
	}
	pub fn size(mut self, s: f32) -> Self {
		self.size = s;
		return self;
	}
	pub fn color(mut self, color: Color) -> Self {
		self.color = color;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		self.color.a = a;
		return self;
	}
	pub fn mode(mut self, m: PointMode) -> Self {
		self.mode = m;
		return self;
	}
}

pub fn points<'a>(pts: &'a[Vec2]) -> Points<'a> {
	return Points::from(pts);
}

impl<'a> Drawable for Points<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		for pt in self.pts {
			match self.mode {
				PointMode::Circle => {
					ctx.draw(&Circle::new(*pt, self.size).fill(self.color))?;
				},
				PointMode::Rect => {
					ctx.draw(&Rect::from_pts(*pt - vec2!(self.size) * 0.5, *pt + vec2!(self.size) * 0.5).fill(self.color))?;
				},
			}
		}

		return Ok(());

	}

}

#[derive(Clone)]
pub struct Circle {
	center: Vec2,
	radius: f32,
	segments: Option<u32>,
	stroke: Option<Stroke>,
	fill: Option<Color>,
	range: (f32, f32),
}

impl Circle {
	pub fn new(center: Vec2, radius: f32) -> Self {
		return Self {
			center: center,
			radius: radius,
			segments: None,
			stroke: None,
			fill: Some(rgba!(1)),
			range: (0.0, 2.0 * PI),
		};
	}
	pub fn fill(mut self, c: Color) -> Self {
		self.fill = Some(c);
		return self;
	}
	pub fn no_fill(mut self) -> Self {
		self.fill = None;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		if let Some(fill) = &mut self.fill {
			fill.a = a;
		}
		if let Some(stroke) = &mut self.stroke {
			stroke.color.a = a;
		}
		return self;
	}
	pub fn stroke(mut self, c: Color) -> Self {
		self.stroke = Some(Stroke {
			width: 1.0,
			join: LineJoin::None,
			dash: None,
			color: c,
		});
		return self;
	}
	pub fn line_join(mut self, j: LineJoin) -> Self {
		if let Some(stroke) = &mut self.stroke {
			stroke.join = j;
		}
		return self;
	}
	pub fn line_width(mut self, w: f32) -> Self {
		if let Some(stroke) = &mut self.stroke {
			stroke.width = w;
		}
		return self;
	}
	pub fn segments(mut self, s: u32) -> Self {
		self.segments = Some(s);
		return self
	}
	pub fn range(mut self, p1: f32, p2: f32) -> Self {
		self.range = (p1, p2);
		return self
	}
}

pub fn circle(center: Vec2, radius: f32) -> Circle {
	return Circle::new(center, radius);
}

// TODO: is this correct?
fn circle_segments(radius: f32) -> u32 {
	return (radius.sqrt() * 6.0) as u32;
}

fn normalize_angle(angle: f32) -> f32 {
	if angle < 0.0 {
		return PI * 2.0 + angle;
	} else {
		return angle;
	}
}

fn rounded_poly_verts(verts: &[Vec2], radius: f32, segments: Option<u32>) -> Vec<Vec2> {

	let segments = segments.unwrap_or(circle_segments(radius));
	let segments = segments as usize;
	let mut nv = Vec::with_capacity(segments);
	let len = verts.len();

	for i in 0..len {

		// TODO: fix weirdness
		let prev = verts.get(i - 1).map(|p| *p).unwrap_or(verts[len - 1]);
		let p = verts[i];
		let next = verts.get(i + 1).map(|p| *p).unwrap_or(verts[0]);
		let angle = normalize_angle(p.angle(prev) - p.angle(next));
		let dis = radius / f32::tan(angle / 2.0);

		let p1 = p + (prev - p) * (dis / (prev - p).mag());
		let p2 = p + (next - p) * (dis / (next - p).mag());

		let center = p + (p1 - p) + (p2 - p);

		let start_angle = center.angle(p1);
		let end_angle = start_angle + angle;

		let arc = arc_verts(radius, start_angle, end_angle, None)
			.iter()
			.map(|p| *p + center)
			.collect::<Vec<Vec2>>()
			;

		nv.extend_from_slice(&arc);

	}

	return nv;

}

fn arc_verts(radius: f32, start: f32, end: f32, segments: Option<u32>) -> Vec<Vec2> {

	let (start, end) = if end < start {
		(end, start)
	} else {
		(start, end)
	};

	let segments = segments.unwrap_or(f32::ceil(circle_segments(radius) as f32 * (end - start) / (PI * 2.0)) as u32);
	let segments = segments as usize;
	let step = (end - start) / segments as f32;
	let mut verts = Vec::with_capacity(segments);

	for i in 0..=segments {

		let angle = start + i as f32 * step;
		verts.push(Vec2::from_angle(angle) * radius);

	}

	return verts;

}

impl Drawable for Circle {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		if self.radius < 0.0 {
			return Ok(());
		}

		let p1 = self.range.0.max(0.0).min(PI * 2.0);
		let p2 = self.range.1.max(0.0).min(PI * 2.0);

		let mut pts = arc_verts(self.radius, p1, p2, self.segments);

		if p1 != 0.0 || p2 != PI * 2.0 {
			pts.push(self.center);
		}

		ctx.push(&gfx::t()
			.t2(self.center)
		, |ctx| {

			let poly = Polygon {
				pts: pts,
				fill: self.fill,
				stroke: self.stroke.clone(),
				radius: None,
			};

			ctx.draw(&poly)?;

			return Ok(());

		})?;

		return Ok(());

	}

}

#[derive(Clone)]
pub struct Canvas<'a> {
	canvas: &'a gfx::Canvas,
	color: Color,
}

pub fn canvas<'a>(c: &'a gfx::Canvas) -> Canvas<'a> {
	return Canvas::new(c);
}

impl<'a> Canvas<'a> {
	pub fn new(c: &'a gfx::Canvas) -> Self {
		return Self {
			canvas: c,
			color: rgba!(1),
		};
	}
	pub fn color(mut self, color: Color) -> Self {
		self.color = color;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		self.color.a = a;
		return self;
	}
}

impl<'a> Drawable for Canvas<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.push(&gfx::t()
			.s2(vec2!(1.0 / ctx.dpi() as f32))
		, |ctx| {
			return ctx.draw(&sprite(&self.canvas.tex()).color(self.color));
		})?;

		return Ok(());

	}

}

#[derive(Clone)]
pub struct Checkerboard {
	size: f32,
	c1: Color,
	c2: Color,
	w: f32,
	h: f32,
}

impl Checkerboard {
	pub fn new(w: f32, h: f32, s: f32) -> Self {
		return Self {
			size: s,
			c1: rgba!(0.5, 0.5, 0.5, 1),
			c2: rgba!(0.75, 0.75, 0.75, 1),
			w: w,
			h: h,
		};
	}
	pub fn color(mut self, c1: Color, c2: Color) -> Self {
		self.c1 = c1;
		self.c2 = c2;
		return self;
	}
}

pub fn checkerboard(w: f32, h: f32, s: f32) -> Checkerboard {
	return Checkerboard::new(w, h, s);
}

impl gfx::Drawable for Checkerboard {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let col = f32::ceil(self.w / self.size) as i32;
		let row = f32::ceil(self.h / self.size) as i32;

		for i in 0..col {

			for j in 0..row {

				let c = if (i + j) % 2 == 0 {
					rgba!(0.5, 0.5, 0.5, 1)
				} else {
					rgba!(0.75, 0.75, 0.75, 1)
				};

				ctx.draw(
					&rect(
						vec2!(i as f32 * self.size, j as f32 * self.size),
						vec2!((i + 1) as f32 * self.size, (j + 1) as f32 * self.size),
					)
						.fill(c)
				)?;

			}

		}

		return Ok(());

	}

}

#[derive(Clone)]
pub struct Model<'a> {
	mesh: &'a gfx::Model,
	color: Color,
	bound: bool,
	wireframe: bool,
}

pub fn model<'a>(m: &'a gfx::Model) -> Model<'a> {
	return Model::new(m);
}

impl<'a> Model<'a> {
	pub fn new(m: &'a gfx::Model) -> Self {
		return Self {
			mesh: m,
			color: rgba!(1),
			bound: false,
			wireframe: false,
		};
	}
	pub fn color(mut self, color: Color) -> Self {
		self.color = color;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		self.color.a = a;
		return self;
	}
	pub fn bound(mut self) -> Self {
		self.bound = true;
		return self;
	}
	pub fn wireframe(mut self, b: bool) -> Self {
		self.wireframe = b;
		return self;
	}
}

impl<'a> Drawable for Model<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw_calls += 1;

		let tex = self.mesh.texture().unwrap_or(&ctx.empty_tex);

		let prim = if self.wireframe {
			gl::Primitive::Line
		} else {
			gl::Primitive::Triangle
		};

		for m in self.mesh.meshes() {
			m.gl_mesh().draw(
				prim,
				&ctx.cur_pipeline_3d,
				&gfx::Uniform3D {
					proj: ctx.proj_3d,
					view: ctx.view_3d,
					model: ctx.transform,
					// TODO: ?
					color: self.color * m.data().mtl.diffuse,
					tex: tex.clone(),
					custom: ctx.cur_custom_uniform_3d.clone(),
				},
			);
		}

		if self.bound {
			let (min, max) = self.mesh.bound();
			ctx.draw(&rect3d(min, max))?;
		}

		return Ok(());

	}

}

#[derive(Clone)]
pub struct Skybox<'a> {
	skybox: &'a gfx::Skybox,
	color: Color,
}

impl<'a> Skybox<'a> {
	pub fn new(s: &'a gfx::Skybox) -> Self {
		return Self {
			skybox: s,
			color: rgba!(1),
		};
	}
}

pub fn skybox<'a>(s: &'a gfx::Skybox) -> Skybox<'a> {
	return Skybox::new(s);
}

impl<'a> Drawable for Skybox<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw_calls += 1;

		ctx.gl.disable(gl::Capability::DepthTest);

		ctx.cubemap_renderer.draw(
			gl::Primitive::Triangle,
			&ctx.pipeline_cubemap,
			&gfx::UniformCubemap {
				proj: ctx.proj_3d,
				view: ctx.view_3d.remove_translation(),
				color: self.color,
				tex: self.skybox.texture().clone(),
			},
		);

		ctx.gl.enable(gl::Capability::DepthTest);

		return Ok(());

	}

}

#[derive(Clone)]
pub struct Cube;

impl Cube {
	pub fn new() -> Self {
		return Self;
	}
}

pub fn cube() -> Cube {
	return Cube::new();
}

impl Drawable for Cube {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw_calls += 1;

		ctx.cube_renderer.draw(
			gl::Primitive::Triangle,
			&ctx.cur_pipeline_3d,
			&gfx::Uniform3D {
				proj: ctx.proj_3d,
				view: ctx.view_3d,
				model: ctx.transform,
				color: rgba!(),
				tex: ctx.empty_tex.clone(),
				custom: ctx.cur_custom_uniform_3d.clone(),
			},
		);

		return Ok(());

	}

}

#[derive(Clone)]
pub struct Line3D {
	p1: Vec3,
	p2: Vec3,
	color: Color,
	width: f32,
}

pub fn line3d(p1: Vec3, p2: Vec3) -> Line3D {
	return Line3D::from(p1, p2);
}

impl Line3D {
	pub fn from(p1: Vec3, p2: Vec3) -> Self {
		return Self {
			p1: p1,
			p2: p2,
			color: rgba!(),
			width: 1.0,
		};
	}
	pub fn color(mut self, c: Color) -> Self {
		self.color = c;
		return self;
	}
}

impl Drawable for Line3D {

	// TODO: deal with out of bound
	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let mut verts = Vec::with_capacity(2 * gfx::Vertex3D::STRIDE);

		gfx::Vertex3D {
			pos: self.p1,
			normal: vec3!(0),
			color: self.color,
			uv: vec2!(0),
		}.push(&mut verts);

		gfx::Vertex3D {
			pos: self.p2,
			normal: vec3!(0),
			color: self.color,
			uv: vec2!(0),
		}.push(&mut verts);

		ctx.renderer_3d.push(
			gl::Primitive::Line,
			&verts,
			&[0, 1],
			&ctx.cur_pipeline_3d,
			&gfx::Uniform3D {
				proj: ctx.proj_3d,
				view: ctx.view_3d,
				model: ctx.transform,
				color: rgba!(),
				tex: ctx.empty_tex.clone(),
				custom: ctx.cur_custom_uniform_3d.clone(),
			},
		)?;

// 		let p1 = ctx.to_sc(self.p1);
// 		let p2 = ctx.to_sc(self.p2);

// 		ctx.draw(
// 			&line(p1, p2)
// 				.color(self.color)
// 		)?;

		return Ok(());

	}

}

#[derive(Clone)]
pub struct Rect3D {
	p1: Vec3,
	p2: Vec3,
	color: Color,
}

pub fn rect3d(p1: Vec3, p2: Vec3) -> Rect3D {
	return Rect3D::from_pts(p1, p2);
}

impl Rect3D {
	pub fn from_pts(p1: Vec3, p2: Vec3) -> Self {
		return Self {
			p1: p1,
			p2: p2,
			color: rgba!(),
		};
	}
	pub fn color(mut self, c: Color) -> Self {
		self.color = c;
		return self;
	}
}

impl Drawable for Rect3D {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let p1 = vec3!(self.p1.x, self.p2.y, self.p1.z);
		let p2 = vec3!(self.p2.x, self.p2.y, self.p1.z);
		let p3 = vec3!(self.p2.x, self.p1.y, self.p1.z);
		let p4 = self.p1;

		let p5 = vec3!(self.p1.x, self.p2.y, self.p2.z);
		let p6 = self.p2;
		let p7 = vec3!(self.p2.x, self.p1.y, self.p2.z);
		let p8 = vec3!(self.p1.x, self.p1.y, self.p2.z);

		ctx.draw(&line3d(p1, p2).color(self.color))?;
		ctx.draw(&line3d(p2, p3).color(self.color))?;
		ctx.draw(&line3d(p3, p4).color(self.color))?;
		ctx.draw(&line3d(p4, p1).color(self.color))?;

		ctx.draw(&line3d(p5, p6).color(self.color))?;
		ctx.draw(&line3d(p6, p7).color(self.color))?;
		ctx.draw(&line3d(p7, p8).color(self.color))?;
		ctx.draw(&line3d(p8, p5).color(self.color))?;

		ctx.draw(&line3d(p1, p5).color(self.color))?;
		ctx.draw(&line3d(p2, p6).color(self.color))?;
		ctx.draw(&line3d(p3, p7).color(self.color))?;
		ctx.draw(&line3d(p4, p8).color(self.color))?;

		return Ok(());

	}

}

#[derive(Clone)]
pub struct Circle3D {
	pt: Vec3,
	radius: f32,
	color: Color,
}

pub fn circle3d(p: Vec3, r: f32) -> Circle3D {
	return Circle3D::new(p, r);
}

impl Circle3D {
	pub fn new(p: Vec3, r: f32) -> Self {
		return Self {
			pt: p,
			radius: r,
			color: rgba!(),
		};
	}
	pub fn color(mut self, c: Color) -> Self {
		self.color = c;
		return self;
	}
}

impl Drawable for Circle3D {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let spt = ctx.to_sc(self.pt);

		ctx.draw(
			&circle(spt, self.radius)
				.fill(self.color)
		)?;

		return Ok(());

	}

}

#[derive(Clone)]
pub struct Sprite3D<'a> {
	tex: &'a gfx::Texture,
	quad: Quad,
	offset: Vec2,
	flip: gfx::Flip,
	color: Color,
}

pub fn sprite3d<'a>(tex: &'a gfx::Texture) -> Sprite3D<'a> {
	return Sprite3D::new(tex);
}

// TODO: up side down?
// TODO: clean
impl<'a> Sprite3D<'a> {
	pub fn new(tex: &'a gfx::Texture) -> Self {
		return Self {
			tex: tex,
			quad: quad!(0, 0, 1, 1),
			color: rgba!(1),
			offset: vec2!(0),
			flip: gfx::Flip::None,
		};
	}
	pub fn quad(mut self, quad: Quad) -> Self {
		self.quad = quad;
		return self;
	}
	pub fn color(mut self, color: Color) -> Self {
		self.color = color;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		self.color.a = a;
		return self;
	}
	pub fn offset(mut self, offset: Vec2) -> Self {
		self.offset = offset;
		return self;
	}
	pub fn flip(mut self, flip: gfx::Flip) -> Self {
		self.flip = flip;
		return self;
	}
}

impl<'a> Drawable for Sprite3D<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let scale = vec2!(self.tex.width(), self.tex.height()) * vec2!(self.quad.w, self.quad.h);
		let offset = self.offset * -0.5;

		ctx.push(&gfx::t()
			.s3(vec3!(scale.x, scale.y, 1.0))
			.t3(vec3!(offset.x, offset.y, 0.0))
		, |ctx| {

			let shape = gfx::Quad3DShape {
				transform: ctx.transform.as_mat4(),
				quad: self.quad,
				color: self.color,
				flip: self.flip,
			};

			ctx.renderer_3d.push_shape(
				gl::Primitive::Triangle,
				shape,
				&ctx.cur_pipeline_3d,
				&gfx::Uniform3D {
					proj: ctx.proj_3d,
					view: ctx.view_3d,
					model: ctx.transform,
					color: rgba!(),
					tex: self.tex.clone(),
					custom: ctx.cur_custom_uniform_3d.clone(),
				},
			)?;

			return Ok(());

		})?;

		return Ok(());

	}

}

