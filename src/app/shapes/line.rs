// wengwengweng

use super::*;

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

