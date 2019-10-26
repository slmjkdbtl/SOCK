// wengwengweng

use dirty::*;
use dirty::app::*;
use dirty::math::*;
use input::Key;

mod pix;
use pix::*;

struct Game {
	model: gfx::Model,
	pix_effect: PixEffect,
	shader: gfx::Shader3D<()>,
	cam: gfx::PerspectiveCam,
	move_speed: f32,
	eye_speed: f32,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let mut model = gfx::Model::from_obj(ctx, include_str!("res/kart.obj"), None, None)?;

		model.update(|data| {
			for m in data {
				for v in &mut m.vertices {
					v.color = color!(rand!(), rand!(), rand!(), 1);
				}
			}
		});

		let model = gfx::Model::from_glb(ctx, include_bytes!("res/buggy.glb"))?;

		return Ok(Self {
			model: model,
			pix_effect: PixEffect::new(ctx)?,
			cam: gfx::PerspectiveCam::new(60.0, ctx.width() as f32 / ctx.height() as f32, 0.1, 1024.0, vec3!(0, 0, 12), 0.0, 0.0),
			shader: gfx::Shader3D::from_frag(ctx, include_str!("res/normal.frag"))?,
			move_speed: 160.0,
			eye_speed: 0.16,
		});

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			KeyPress(k) => {
				if k == Key::Esc {
					ctx.toggle_cursor_hidden();
					ctx.toggle_cursor_locked()?;
				}
				if k == Key::F {
					ctx.toggle_fullscreen();
				}
			},

			MouseMove(delta) => {

				if ctx.is_cursor_locked() {

					let mut rx = self.cam.yaw();
					let mut ry = self.cam.pitch();
					let dead = 48.0f32.to_radians();

					rx += delta.x * self.eye_speed * ctx.dt();
					ry -= delta.y * self.eye_speed * ctx.dt();

					if ry > dead {
						ry = dead;
					}

					if ry < -dead {
						ry = -dead;
					}

					self.cam.set_angle(rx, ry);

				}

			},

			_ => {},

		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		if ctx.key_down(Key::W) {
			self.cam.set_pos(self.cam.pos() + self.cam.front() * ctx.dt() * self.move_speed);
		}

		if ctx.key_down(Key::S) {
			self.cam.set_pos(self.cam.pos() - self.cam.front() * ctx.dt() * self.move_speed);
		}

		if ctx.key_down(Key::A) {
			self.cam.set_pos(self.cam.pos() - self.cam.front().cross(vec3!(0, 1, 0)).normalize() * ctx.dt() * self.move_speed);
		}

		if ctx.key_down(Key::D) {
			self.cam.set_pos(self.cam.pos() + self.cam.front().cross(vec3!(0, 1, 0)).normalize() * ctx.dt() * self.move_speed);
		}

		let (min, max) = self.model.bound();

		self.pix_effect.render(ctx, |ctx| {

			ctx.clear();
// 			ctx.clear_ex(gfx::Surface::Depth);

			ctx.use_cam(&self.cam, |ctx| {

				ctx.draw(&shapes::rect3d(min, max))?;

// 				ctx.push(&gfx::t()
// 					.rotate_y(ctx.time())
// 				, |ctx| {
					ctx.draw(&shapes::model(&self.model))?;
// 					return Ok(());
// 				})?;

				return Ok(());

			})?;

			return Ok(());

		})?;

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut app::Ctx) -> Result<()> {

// 		self.pix_effect.draw(ctx, &PixUniform {
// 			resolution: vec2!(ctx.width(), ctx.height()),
// 			size: 6.0,
// 		})?;

		let (min, max) = self.model.bound();

		ctx.use_cam(&self.cam, |ctx| {

			ctx.draw_3d_with(&self.shader, &(), |ctx| {
				ctx.draw(&shapes::model(&self.model))?;
				return Ok(());
			})?;

// 			ctx.draw(&shapes::rect3d(min, max))?;

// 				ctx.push(&gfx::t()
// 					.rotate_y(ctx.time())
// 				, |ctx| {
// 				ctx.draw(&shapes::model(&self.model))?;
// 					return Ok(());
// 				})?;

			return Ok(());

		})?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
		.cursor_locked(true)
		.cursor_hidden(true)
		.run::<Game>() {
		println!("{}", err);
	}

}

