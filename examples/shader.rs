// wengwengweng

use dirty::*;

fn main() {

	// init modules
	app::init();
	window::init("shader", 640, 480);

	let (w, h) = window::size();

	let canvas = gfx::Canvas::new(320, 320);
	let shader = g2d::Shader::from_code_frag(include_str!("assets/test.frag"));

	// main loop
	app::run(|| {

		g2d::color(color!(0, 1, 1, 1));
		g2d::rect(vec2!(w, h));

		g2d::set_shader(&shader);
		g2d::translate(vec2!(160, 80));
		g2d::render(&canvas);


	});

}

