// wengwengweng

use dirty::*;
use window::Key;

const RATE: usize = 128;
const GATE: u16 = 54;

struct Game {

	tex: gfx::Texture,
	count: usize,
	started: bool,
	done: bool,

}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		return Ok(Self {
			tex: gfx::Texture::from_bytes(ctx, include_bytes!("res/icon.png"))?,
			count: 0,
			done: false,
			started: false,
		});
	}

	fn run(&mut self, ctx: &mut app::Ctx, dt: f32) -> Result<()> {

		let w = 640;
		let h = 480;

		if window::key_pressed(ctx, Key::F) {
			window::toggle_fullscreen(ctx);
		}

		if window::key_pressed(ctx, Key::Escape) {
			app::quit(ctx);
		}

		if self.started {

			if !self.done {

				for _ in 0..self.count {

					gfx::push(ctx);
					gfx::translate(ctx, vec2!(rand!(0, w), rand!(0, h)));
					gfx::draw(ctx, gfx::sprite(&self.tex))?;
					gfx::pop(ctx)?;

				}

			} else {

				gfx::push(ctx);
				gfx::translate(ctx, vec2!(24));
				gfx::draw(ctx, gfx::text(&format!("{}", self.count)))?;
				gfx::pop(ctx)?;

			}

		} else {

			gfx::push(ctx);
			gfx::translate(ctx, vec2!(24));
			gfx::draw(ctx, gfx::text("waiting..."))?;
			gfx::pop(ctx)?;

		}

		window::set_title(ctx, &format!("FPS: {} DCS: {} OBJS: {}", app::fps(ctx), gfx::draw_calls(ctx), self.count));

		if !self.started {
			if app::fps(ctx) >= 60 {
				self.started = true;
			}
		} else {
			if !self.done {
				self.count += RATE;
				if app::fps(ctx) <= GATE {
					println!("{}", self.count);
					self.done = true;
				}
			}
		}

		return Ok(());

	}

}

fn main() {

	let result = app::App::new()
		.run::<Game>();

	if let Err(err) = result {
		println!("{}", err);
	}

}

