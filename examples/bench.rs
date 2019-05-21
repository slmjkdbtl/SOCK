// wengwengweng

use dirty::*;
use window::Key;

const RATE: usize = 96;
const GATE: u32 = 48;

fn main() {

	window::init(Default::default());

	let tex = gfx::Texture::from_bytes(include_bytes!("res/icon.png"));
	let (w, h) = window::size().into();
	let mut started = false;
	let mut done = false;
	let mut count = 0;

	window::run(|| {

		let fps = window::fps();

		if !started {
			if fps >= GATE {
				started = true;
			}
		}

		if !done {

			if fps >= GATE {

				for _ in 0..count {

					g2d::push();
					g2d::translate(vec2!(tex.width(), tex.height()) * -1.0);
					g2d::translate(vec2!(rand!(w), rand!(h)));
					g2d::scale(vec2!(2));
					g2d::draw(&tex, rect!(0, 0, 1, 1));
					g2d::pop();

				}

				count += RATE;

			} else {
				if started {
					done = true;
				}
			}

		} else {

			g2d::translate(vec2!(48));
			g2d::scale(vec2!(6));
			g2d::text(&format!("{}", count));
			println!("{}", count);

		}

		if window::key_pressed(Key::Escape) {
			window::quit();
		}

	});

}

