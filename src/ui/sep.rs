// wengwengweng

use super::*;

pub struct Sep;

impl Widget for Sep {

	fn draw(&mut self, ctx: &mut Ctx, wctx: &WidgetCtx) -> Result<f32> {

		let p = wctx.theme.padding;

		ctx.draw(
			&shapes::line(vec2!(0, -p * 0.5), vec2!(wctx.width, -p * 0.5))
				.color(wctx.theme.border_color)
				.width(2.0)
		)?;

		return Ok(p);

	}

}

