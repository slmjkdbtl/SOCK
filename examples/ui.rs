// wengwengweng

use dirty::*;
use input::*;

struct Game {
	ui: ui::UI,
}

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			ui: ui::UI::new(),
		});
	}

	fn event(&mut self, d: &mut Ctx, e: &Event) -> Result<()> {

		use Event::*;

		self.ui.event(d, &e);

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

		let top_left = d.gfx.coord(gfx::Origin::TopLeft);

		self.ui.window(d, "test", top_left + vec2!(64, -64), 240.0, 360.0, |ctx, p| {

			p.text(ctx, "yo")?;
			p.input(ctx, "name")?;
			p.slider::<i32>(ctx, "height", 170, 0, 300)?;
			p.select(ctx, "gender", &["unknown", "male", "female"], 1)?;
			p.checkbox(ctx, "dead", false)?;
			p.sep(ctx)?;
			p.button(ctx, "explode")?;

			return Ok(());

		})?;

		return Ok(());

	}

}

fn main() {
	if let Err(e) = launcher()
		.run::<Game>() {
		elog!("{}", e);
	}
}

