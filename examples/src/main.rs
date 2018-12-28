// wengwengweng

#![windows_subsystem = "windows"]
#![allow(unused_parens)]

extern crate dirty;

use dirty::app;
use dirty::gfx;
use dirty::audio;
use dirty::math;

fn main() {

	app::init("yo", 640, 480);
	gfx::init();
	#[cfg(not(target_os = "windows"))]
	audio::init();

	let tex = gfx::Texture::from_byte(&include_bytes!("car.png")[..]);
	let mut index = 0;

	app::run(&mut || {

		if (index < 3) {
			index += 1;
		} else {
			index = 0;
		}

		gfx::clear();
		gfx::draw(&tex, math::vec2(240.0, 240.0), 0.0, math::vec2(2.0, 2.0), math::vec4((index as f32) * 0.25, 0.0, 0.25, 1.0));

	});

}

