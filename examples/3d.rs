// wengwengweng

#![feature(clamp)]

use dirty::*;
use math::*;
use geom::*;
use geom::*;
use input::Key;
use gfx::Camera;
use gfx::shapes;

const SCALE: f32 = 4.0;

#[derive(Clone)]
struct Uniform {
	cam_pos: Vec3,
	fog_color: Color,
	fog_level: f32,
}

impl gfx::CustomUniform for Uniform {
	fn values(&self) -> gfx::UniformValues {
		return hmap![
			"u_cam_pos" => &self.cam_pos,
			"u_fog_color" => &self.fog_color,
			"u_fog_level" => &self.fog_level,
		];
	}
}

struct Game {
	model: gfx::Model,
	cam: gfx::PerspectiveCam,
	move_speed: f32,
	eye_speed: f32,
	shader: gfx::Shader<Uniform>,
	show_ui: bool,
// 	canvas: gfx::Canvas,
	floor: gfx::MeshData,
}

impl State for Game {

	fn init(ctx: &mut Ctx) -> Result<Self> {

		let gfx = &mut ctx.gfx;

		let model = gfx::Model::from_glb(
			gfx,
			include_bytes!("res/btfly.glb"),
		)?;

		let model = gfx::Model::from_obj(
			gfx,
			include_str!("res/truck.obj"),
			Some(include_str!("res/truck.mtl")),
			None,
		)?;

		let floor = meshgen::checkerboard(2.0, 9, 9);

		let cw = (gfx.width() as f32 / SCALE) as i32;
		let ch = (gfx.height() as f32 / SCALE) as i32;

		return Ok(Self {
			model: model,
			cam: gfx::PerspectiveCam {
				fov: f32::to_radians(60.0),
				aspect: gfx.width() as f32 / gfx.height() as f32,
				near: 0.1,
				far: 1024.0,
				pos: vec3!(0, 1, 6),
				dir: vec3!(0, 0, -1),
			},
			move_speed: 12.0,
			eye_speed: 32.0,
			shader: gfx::Shader::from_frag(gfx, include_str!("res/fog.frag"))?,
			show_ui: false,
// 			canvas: gfx::Canvas::new(ctx, cw, ch)?,
			floor: floor,
		});

	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		let win = &mut ctx.window;

		match e {

			Resize(w, h) => {

				let cw = (*w as f32 / SCALE) as i32;
				let ch = (*h as f32 / SCALE) as i32;

// 				self.canvas.resize(ctx, cw, ch)?;
				self.cam.aspect = *w as f32 / *h as f32;

			},

			KeyPress(k) => {
				let mods = win.key_mods();
				match *k {
					Key::Esc => {
						win.toggle_cursor_hidden();
						win.toggle_cursor_locked();
					},
					Key::F => win.toggle_fullscreen(),
					Key::Q if mods.meta => win.quit(),
					Key::L => {
						win.set_cursor_hidden(self.show_ui);
						win.set_cursor_locked(self.show_ui);
						self.show_ui = !self.show_ui;
					}
					_ => {},
				}
			},

			MouseMove(delta) => {

				if win.is_cursor_hidden() {

					let mut rx = self.cam.yaw();
					let mut ry = self.cam.pitch();
					let dead = f32::to_radians(60.0);

					rx += delta.x * self.eye_speed * 0.0001;
					ry += delta.y * self.eye_speed * 0.0001;

					ry = ry.clamp(-dead, dead);

					self.cam.set_angle(rx, ry);

				}

			},

			_ => {},

		}

		return Ok(());

	}

// 	fn ui(&mut self, ctx: &mut Ctx, ui: &mut ui::UI) -> Result<()> {

// 		if self.show_ui {

// 			let top_left = ctx.coord(gfx::Origin::TopLeft);

// 			ui.window(ctx, "debug", top_left + vec2!(120, -120), 240.0, 240.0, |ctx, p| {

// 				let fov = self.cam.fov.to_degrees();

// 				self.cam.fov = p.slider(ctx, "FOV", fov, 45.0, 90.0)?.to_radians();

// 				return Ok(());

// 			})?;

// 		}

// 		return Ok(());

// 	}

	fn update(&mut self, ctx: &mut Ctx) -> Result<()> {

		let win = &mut ctx.window;
		let app = &mut ctx.app;
		let dt = app.dt().as_secs_f32();

		if win.key_down(Key::W) {
			self.cam.pos += self.cam.front() * dt * self.move_speed;
		}

		if win.key_down(Key::S) {
			self.cam.pos += self.cam.back() * dt * self.move_speed;
		}

		if win.key_down(Key::A) {
			self.cam.pos += self.cam.left() * dt * self.move_speed;
		}

		if win.key_down(Key::D) {
			self.cam.pos += self.cam.right() * dt * self.move_speed;
		}

// 		win.set_title(&format!("FPS: {} DCS: {}", ctx.fps(), ctx.draw_calls()));

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		let gfx = &mut ctx.gfx;

		let p = vec3!(0);
		let origin = self.cam.to_screen(gfx, p);

		gfx.use_cam(&self.cam, |gfx| {

			gfx.draw_with(&self.shader, &Uniform {
				cam_pos: self.cam.pos,
				fog_color: rgba!(0, 0, 0, 1),
				fog_level: 3.0,
			}, |gfx| {

				let bbox = self.model.bbox().transform(mat4!());
				let mray = Ray3::new(self.cam.pos, self.cam.dir);

				let c = if col::intersect3d(mray, bbox) {
					rgba!(0, 0, 1, 1)
				} else {
					rgba!(1)
				};

				gfx.draw(&shapes::model(&self.model))?;

				gfx.draw(
					&shapes::Rect3D::from_bbox(bbox)
						.line_width(1.0)
						.color(c)
				)?;

				let ground = Plane::new(vec3!(0, 1, 0), 0.0);

				gfx.draw(&shapes::Raw::from_meshdata(&self.floor))?;

				return Ok(());

			})?;

			return Ok(());

		})?;

		gfx.draw(&shapes::circle(vec2!(0), 2.0))?;

		gfx.draw_t(
			mat4!()
				.t2(origin)
				,
			&shapes::text("car")
				.size(16.0)
				,
		)?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = launcher()
		.cursor_hidden(true)
		.cursor_locked(true)
		.resizable(true)
// 		.hidpi(false)
// 		.fps_cap(None)
// 		.vsync(false)
		.run::<Game>() {
		println!("{}", err);
	}

}

