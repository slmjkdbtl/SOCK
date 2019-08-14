// wengwengweng

use dirty::*;
use dirty::app::*;
use input::Key;

struct Game {
	mask: gfx::Tex2D,
	tex: gfx::Tex2D,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		return Ok(Self {
			mask: gfx::Tex2D::from_bytes(ctx, include_bytes!("res/blob.png"))?,
			tex: gfx::Tex2D::from_bytes(ctx, include_bytes!("res/gradient.png"))?,
		});

	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.scale(vec2!(2));

		ctx.draw_masked(|ctx| {
			ctx.draw(shapes::sprite(&self.mask))?;
			return Ok(());
		}, |ctx| {
			ctx.push();
			ctx.translate(vec2!(0, (ctx.time() * 6.0).sin() * 24.0));
			ctx.draw(shapes::sprite(&self.tex))?;
			ctx.pop()?;
			return Ok(());
		})?;

		ctx.set_title(&format!("FPS: {} DCS: {}", ctx.fps(), ctx.draw_calls()));

		if ctx.key_pressed(Key::F) {
			ctx.toggle_fullscreen();
		}

		if ctx.key_pressed(Key::Escape) {
			ctx.quit();
		}

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
		.run::<Game>() {
		println!("{}", err);
	}

}


