// wengwengweng

use dirty::*;
use dirty::app::*;
use input::Key;

struct Game;

impl app::State for Game {

	fn init(_: &mut app::Ctx) -> Result<Self> {
		return Ok(Self);
	}

	fn event(&mut self, ctx: &mut app::Ctx, e: input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				if k == Key::Esc {
					ctx.quit();
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.push(&gfx::t()
			.translate_3d(vec3!(0, 0, 3))
			.rotate_y(ctx.time())
			.rotate_z(ctx.time())
		, |ctx| {
			return ctx.draw(&shapes::cube());
		})?;

		ctx.draw(&shapes::text("yo"))?;

		return Ok(());

	}

}

fn main() {
	if let Err(err) = app::launcher()
// 		.origin(gfx::Origin::TopLeft)
// 		.quad_origin(gfx::Origin::TopLeft)
		.run::<Game>() {
		println!("{}", err);
	}
}

