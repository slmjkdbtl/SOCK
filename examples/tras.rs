// wengwengweng

use dirty::*;
use math::*;
use rayon::prelude::*;

struct Canvas {
	buf: Vec<Color>,
	width: u32,
	height: u32,
}

impl Canvas {

	pub fn new(w: u32, h: u32) -> Self {
		return Self {
			width: w,
			height: h,
			buf: vec![rgba!(0); w as usize * h as usize],
		};
	}

	pub fn width(&self) -> u32 {
		return self.width;
	}

	pub fn height(&self) -> u32 {
		return self.height;
	}

	pub fn buf(&self) -> &[Color] {
		return &self.buf;
	}

	pub fn as_u8(&self) -> Vec<u8> {

		let mut buf = Vec::with_capacity(self.width as usize * self.height as usize * 4);

		for c in &self.buf {
			buf.push((c.r * 255.0) as u8);
			buf.push((c.g * 255.0) as u8);
			buf.push((c.b * 255.0) as u8);
			buf.push((c.a * 255.0) as u8);
		}

		return buf;

	}

	pub fn clear(&mut self) {
		self.buf = vec![rgba!(0); self.width as usize * self.height as usize];
	}

	pub fn put(&mut self, x: usize, y: usize, c: Color) {

		let i = y * self.width as usize + x;

		self.buf.get_mut(i).map(|i| *i = c);

	}

	pub fn shade(&mut self, f: impl Fn(usize, usize, Color) -> Color + Sync + Send) {

		let w = self.width as usize;

		self.buf.par_iter_mut().enumerate().for_each(|(i, c)| {

			let x = i % w;
			let y = i / w;

			*c = f(x, y, *c);

		});

	}

	pub fn rect(&mut self, p1: Vec2, p2: Vec2, col: Color) {

		let p1 = p1.clamp(vec2!(0), vec2!(self.width, self.height));
		let p2 = p2.clamp(vec2!(0), vec2!(self.width, self.height));

		let x1 = f32::min(p1.x, p2.x) as usize;
		let x2 = f32::max(p1.x, p2.x) as usize;
		let y1 = f32::min(p1.y, p2.y) as usize;
		let y2 = f32::max(p1.y, p2.y) as usize;

		for x in x1..x2 {
			for y in y1..y2 {
				self.put(x, y, col);
			}
		}

	}

	pub fn line(&mut self, p1: Vec2, p2: Vec2, col: Color) {

		let p1 = p1.clamp(vec2!(0), vec2!(self.width, self.height));
		let p2 = p2.clamp(vec2!(0), vec2!(self.width, self.height));

		let mut x0 = p1.x as i32;
		let mut x1 = p2.x as i32;
		let mut y0 = p1.y as i32;
		let mut y1 = p2.y as i32;

		let mut steep = false;

		if (i32::abs(x0 - x1) < i32::abs(y0 - y1)) {
			std::mem::swap(&mut x0, &mut y0);
			std::mem::swap(&mut x1, &mut y1);
			steep = true;
		}

		if (x0 > x1) {
			std::mem::swap(&mut x0, &mut x1);
			std::mem::swap(&mut y0, &mut y1);
		}

		let dx = x1 - x0;
		let dy = y1 - y0;
		let derr = i32::abs(dy) * 2;
		let mut err = 0;
		let mut y = y0;

		for x in x0..x1 {

			if (steep) {
				self.put(y as usize, x as usize, col);
			} else {
				self.put(x as usize, y as usize, col);
			}

			err += derr;

			if (err > dx) {
				y += i32::signum(dy);
				err -= dx * 2;
			}

		}

	}

}

use std::time::Duration;
use std::time::Instant;

struct TimeManager {
	expected_dt: Duration,
	dt: Duration,
	start_time: Instant,
	last_frame_time: Instant,
}

impl TimeManager {

	pub fn new(fps: usize) -> Self {

		let expected_dt = 1.0 / fps as f32;

		return Self {
			expected_dt: Duration::from_millis((expected_dt * 1000.0) as u64),
			dt: Duration::from_millis(0),
			start_time: Instant::now(),
			last_frame_time: Instant::now(),
		};

	}

	pub fn tick(&mut self) {

		let actual_dt = self.last_frame_time.elapsed();

		if actual_dt < self.expected_dt {
			std::thread::sleep(self.expected_dt - actual_dt);
		}

		self.dt = self.last_frame_time.elapsed();
		self.last_frame_time = Instant::now();

	}

	pub fn fps(&self) -> usize {
		return (1.0 / self.dt.as_secs_f32()) as usize;
	}

	pub fn dt(&self) -> Duration {
		return self.dt;
	}

	pub fn time(&self) -> Duration {
		return self.start_time.elapsed();
	}

}

fn main() {

	let mut canvas = Canvas::new(64, 48);
	let mut tm = TimeManager::new(60);

	loop {

		let w = canvas.width() as f32;
		let h = canvas.height() as f32;
		let t = tm.time().as_secs_f32();

		canvas.shade(|x, y, c| {

			let uv = vec2!(x, y) / vec2!(w, h);
			let angle = f32::atan2(uv.y, uv.x);
			let dis = Vec2::dis(uv, vec2!(0.5));

			let time = t * 4.0;

			let c1 = f32::sin(dis * 50.0 + time + angle);
			let c2 = f32::sin(dis * 50.0 + time + angle + (1.0 / 3.0) * 3.14 * 2.0);
			let c3 = f32::sin(dis * 50.0 + time + angle + (2.0 / 3.0) * 3.14 * 2.0);

			return rgba!(c1, c2, c3, 1);

		});

		term::display(canvas.buf(), canvas.width(), canvas.height());
		tm.tick();

	}

}

