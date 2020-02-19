// wengwengweng

use dirty::*;
use input::Key;

struct Game;

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(Self);
	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => ctx.quit(),
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		use gfx::Vertex2D;

		ctx.draw(&shapes::raw_2d(&[
			Vertex2D {
				pos: vec3!(0, 72, 0),
				color: rgba!(1, 0, 0, 1),
				uv: vec2!(0),
			},
			Vertex2D {
				pos: vec3!(-96, -72, 0),
				color: rgba!(0, 1, 0, 1),
				uv: vec2!(0),
			},
			Vertex2D {
				pos: vec3!(96, -72, 0),
				color: rgba!(0, 0, 1, 1),
				uv: vec2!(0),
			},
		], &[0, 1, 2]))?;

		return Ok(());

	}

}

fn main() -> Result<()> {
	return run::<Game>();
}

