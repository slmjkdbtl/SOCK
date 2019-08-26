// wengwengweng

use dirty::*;
use dirty::app::*;
use dirty::math::*;
use input::Key;

struct Game {
	model: gfx::Model,
	pixel_effect: gfx::Shader,
	canvas: gfx::Canvas,
	cam: gfx::Camera,
	move_speed: f32,
	eye_speed: f32,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let pixel_effect = gfx::Shader::effect(ctx, include_str!("res/pix.frag"))?;

		pixel_effect.send("size", 6.0);
		pixel_effect.send("dimension", vec2!(ctx.width(), ctx.height()));

		return Ok(Self {
			model: gfx::Model::from_obj(ctx, include_str!("res/cow.obj"))?,
			pixel_effect: pixel_effect,
			canvas: gfx::Canvas::new(ctx, ctx.width(), ctx.height())?,
			cam: gfx::Camera::new(vec3!(0, 0, -12), 0.0, 0.0),
			move_speed: 16.0,
			eye_speed: 0.16,
		});

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			KeyPress(k) => {
				if *k == Key::Esc {
					ctx.quit();
				}
				if *k == Key::F {
					ctx.toggle_fullscreen();
				}
			},

			KeyDown(k) => {

				if *k == Key::W {
					self.cam.set_pos(self.cam.pos() + self.cam.front() * ctx.dt() * self.move_speed);
				}

				if *k == Key::S {
					self.cam.set_pos(self.cam.pos() - self.cam.front() * ctx.dt() * self.move_speed);
				}

				if *k == Key::A {
					self.cam.set_pos(self.cam.pos() + self.cam.front().cross(vec3!(0, 1, 0)).normalize() * ctx.dt() * self.move_speed);
				}

				if *k == Key::D {
					self.cam.set_pos(self.cam.pos() - self.cam.front().cross(vec3!(0, 1, 0)).normalize() * ctx.dt() * self.move_speed);
				}

			},

			MouseMove(delta) => {

				let md: Vec2 = (*delta).into();
				let mut rx = self.cam.yaw();
				let mut ry = self.cam.pitch();
				let dead = 48.0f32.to_radians();

				rx -= md.x * self.eye_speed * ctx.dt();
				ry -= md.y * self.eye_speed * ctx.dt();

				if ry > dead {
					ry = dead;
				}

				if ry < -dead {
					ry = -dead;
				}

				self.cam.set_angle(rx, ry);

			},

			_ => {},

		}

		return Ok(());

	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		use gfx::Transform::*;

		ctx.draw_on(&self.canvas, |ctx| {

			ctx.clear_ex(gfx::Surface::Depth);

			ctx.use_cam(&self.cam, |ctx| {

				ctx.push(&[
					RotateY(ctx.time().into()),
				], |ctx| {
					return ctx.draw(shapes::model(&self.model));
				})?;

				return Ok(());

			})?;

			return Ok(());

		})?;

		ctx.draw_with(&self.pixel_effect, |ctx| {
			return ctx.draw(shapes::canvas(&self.canvas));
		})?;

		ctx.set_title(&format!("FPS: {} DCS: {}", ctx.fps(), ctx.draw_calls()));

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

