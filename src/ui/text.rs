// wengwengweng

use super::*;

pub struct Text {
	text: String,
}

impl Text {
	pub fn new(text: &str) -> Self {
		return Self {
			text: text.to_string(),
		};
	}
}

impl Widget for Text {

	fn draw(&mut self, gfx: &mut gfx::Gfx, wctx: &WidgetCtx) -> Result<f32> {

		let text = shapes::text(&self.text)
			.size(wctx.theme.font_size)
			.color(wctx.theme.title_color)
			.align(gfx::Origin::TopLeft)
			.format(gfx)
			;

		gfx.draw(&text)?;

		return Ok(text.height());

	}

}

