// wengwengweng

use std::thread;
use std::time::Instant;
use std::time::Duration;

use crate::*;

pub struct Ctx {

	pub(crate) window: window::Ctx,
	pub(crate) gfx: gfx::Ctx,

	quit: bool,
	dt: f32,
	time: f32,
	fps_cap: u32,

}

#[derive(Clone, Debug)]
pub struct Conf {
	pub width: u32,
	pub height: u32,
	pub title: String,
	pub hidpi: bool,
	pub resizable: bool,
	pub fullscreen: bool,
	pub always_on_top: bool,
	pub borderless: bool,
	pub transparent: bool,
	pub vsync: bool,
	pub hide_title: bool,
	pub hide_titlebar_buttons: bool,
	pub fullsize_content: bool,
	pub titlebar_transparent: bool,
	pub cursor_hidden: bool,
	pub cursor_locked: bool,
}

impl Conf {

	pub fn basic(title: &str, width: u32, height: u32) -> Self {
		return Self {
			title: String::from(title),
			width: width,
			height: height,
			..Default::default()
		};
	}

}

impl Default for Conf {

	fn default() -> Self {
		return Self {
			width: 640,
			height: 480,
			title: String::new(),
			hidpi: true,
			resizable: false,
			fullscreen: false,
			always_on_top: false,
			borderless: false,
			transparent: false,
			vsync: false,
			fullsize_content: false,
			hide_title: false,
			hide_titlebar_buttons: false,
			titlebar_transparent: false,
			cursor_hidden: false,
			cursor_locked: false,
		};
	}

}

pub trait State {
	fn init(&mut self, ctx: &mut Ctx) {}
	fn run(&mut self, ctx: &mut Ctx) {}
}

pub fn run<S: State>(mut s: S) -> Result<()> {

	let window = window::Ctx::new(&Conf::default())?;
	let gfx = gfx::Ctx::new(&window);

	let mut ctx = Ctx {

		window: window,
		gfx: gfx,

		quit: false,
		dt: 0.0,
		time: 0.0,
		fps_cap: 60,

	};

	s.init(&mut ctx);

	loop {

		let start_time = Instant::now();

		if !ctx.window.poll()? {
			break;
		}

		gfx::clear(&mut ctx);
		s.run(&mut ctx);
		ctx.window.swap()?;

		if ctx.quit {
			break;
		}

		let actual_dt = start_time.elapsed();
		let actual_dt = actual_dt.as_millis() as f32;
		let expected_dt = 1000.0 / ctx.fps_cap as f32;

		if expected_dt > actual_dt {
			ctx.dt = expected_dt as f32 / 1000.0;
			thread::sleep(Duration::from_millis((expected_dt - actual_dt) as u64));
		} else {
			ctx.dt = actual_dt as f32 / 1000.0;
		}

		ctx.time += ctx.dt;

	}

	return Ok(());

}

pub fn quit(ctx: &mut Ctx) {
	ctx.quit = true;
}

pub fn dt(ctx: &Ctx) -> f32 {
	return ctx.dt;
}

pub fn time(ctx: &Ctx) -> f32 {
	return ctx.time;
}

