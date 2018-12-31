// wengwengweng

#![windows_subsystem = "windows"]

use dirty::*;
use dirty::math::*;

fn main() {

	app::init("yo", 640, 480);

	let tex = gfx::make_tex(&include_bytes!("./car.png")[..]);
	let mut index = 0;

	res::load_sprites(".", vec!["car"]);

	app::run(&mut || {

		let (width, height) = app::size();

		if index < 3 {
			index += 1;
		} else {
			index = 0;
		}

		gfx::clear();

		gfx::push();
		gfx::translate(vec2!(108));
		gfx::scale(vec2!(2));
		gfx::translate(vec2!(64));
		gfx::rotate(app::time().to_radians());
		gfx::translate(vec2!(-64));
		gfx::draw(&tex, rect!((index as f32) * 0.25, 0, 0.25, 1));
		gfx::text("yo♪");
		gfx::pop();

		gfx::color(color!(1, 1, 0, 1));
		gfx::line_width(2);
		let p1 = vec2!(rand() * width as f32, rand() * height as f32);
		let p2 = vec2!(rand() * width as f32, rand() * height as f32);
		gfx::line(p1, p2);

		if app::key_pressed(Key::F) {
			app::set_fullscreen(!app::get_fullscreen())
		}

		if app::key_pressed(Key::Escape) {
			app::quit();
		}

	});

}

