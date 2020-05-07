// wengwengweng

use super::*;
use kit::textedit;

pub struct Input {
	buf: textedit::Input,
	prompt: &'static str,
}

impl Input {
	pub fn new(prompt: &'static str,) -> Self {
		return Self {
			buf: textedit::Input::new(),
			prompt: prompt,
		};
	}
	pub fn text(&self) -> String {
		return self.buf.content().to_string();
	}
}

impl Widget for Input {

	fn draw(&self, ctx: &mut Ctx, pctx: &PanelCtx) -> Result<f32> {

		let mut height = 0.0;

		let ptext = shapes::text(&format!("{}:", self.prompt))
			.size(pctx.theme.font_size)
			.color(pctx.theme.title_color)
			.align(gfx::Origin::TopLeft)
			.format(ctx)
			;

		height += ptext.height() + pctx.theme.margin * 0.8;

		ctx.draw(&ptext)?;

		let itext = shapes::text(self.buf.content())
			.size(pctx.theme.font_size)
			.color(pctx.theme.title_color)
			.align(gfx::Origin::TopLeft)
			.format(ctx)
			;

		ctx.draw(
			&shapes::rect(vec2!(0, -height), vec2!(pctx.width - 4.0, -height - itext.height() - 12.0) + vec2!(6, -6))
				.stroke(pctx.theme.border_color)
				.line_width(2.0)
				.fill(pctx.theme.bar_color)
		)?;

		height += itext.height() + 12.0 + pctx.theme.margin;

		return Ok(height);

	}

}

