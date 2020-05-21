// wengwengweng

use dirty::*;
use gfx::shapes;
use input::Key;

struct Game;

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {
		return Ok(Self);
	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => d.window.quit(),
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&mut self, d: &mut Ctx) -> Result<()> {

		use gfx::Vertex;

		d.gfx.draw(&shapes::raw(&[
			Vertex {
				pos: vec3!(0, 72, 0),
				color: rgba!(1, 0, 0, 1),
				normal: vec3!(0, 0, 1),
				uv: vec2!(0),
			},
			Vertex {
				pos: vec3!(-96, -72, 0),
				color: rgba!(0, 1, 0, 1),
				normal: vec3!(0, 0, 1),
				uv: vec2!(0),
			},
			Vertex {
				pos: vec3!(96, -72, 0),
				color: rgba!(0, 0, 1, 1),
				normal: vec3!(0, 0, 1),
				uv: vec2!(0),
			},
		], &[0, 1, 2]))?;

		return Ok(());

	}

}

fn main() {
	if let Err(e) = launcher()
		.title("raw")
		.canvas_root(CanvasRoot::Element("#container"))
		.run::<Game>() {
		elog!("{}", e);
	}
}

