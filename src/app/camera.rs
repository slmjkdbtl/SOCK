// wengwengweng

use crate::*;
use super::*;

/// general functionalities of a camera
pub trait Camera {
	fn projection(&self) -> Mat4;
	fn lookat(&self) -> Mat4;
	fn pos(&self) -> Vec3;
}

/// 3d perspective camera
#[derive(Clone)]
pub struct PerspectiveCam {
	front: Vec3,
	pos: Vec3,
	yaw: f32,
	pitch: f32,
	fov: f32,
	aspect: f32,
	near: f32,
	far: f32,
}

impl PerspectiveCam {

	/// create new perspective camera
	pub fn new(fov: f32, aspect: f32, near: f32, far: f32, pos: Vec3, yaw: f32, pitch: f32) -> Self {

		let mut c = Self {
			pos: vec3!(),
			front: vec3!(),
			yaw: 0.0,
			pitch: 0.0,
			fov: fov,
			aspect: aspect,
			near: near,
			far: far,
		};

		c.set_pos(pos);
		c.set_angle(yaw, pitch);

		return c;

	}

	/// set cam pos
	pub fn set_pos(&mut self, pos: Vec3) {
		self.pos = pos;
	}

	/// set cam facing direction
	pub fn set_front(&mut self, front: Vec3) {
		// TODO: calculate yaw & pitch from front
		self.front = front;
	}

	/// set cam angle
	pub fn set_angle(&mut self, yaw: f32, pitch: f32) {

		self.yaw = yaw;
		self.pitch = pitch;

		self.front = vec3!(
			self.pitch.cos() * (self.yaw - 90f32.to_radians()).cos(),
			self.pitch.sin(),
			self.pitch.cos() * (self.yaw - 90f32.to_radians()).sin(),
		).normalize();

	}

	/// get cam facing direction
	pub fn front(&self) -> Vec3 {
		return self.front;
	}

	/// get cam yaw
	pub fn yaw(&self) -> f32 {
		return self.yaw;
	}

	/// get cam pitch
	pub fn pitch(&self) -> f32 {
		return self.pitch;
	}

}

impl Camera for PerspectiveCam {
	fn projection(&self) -> Mat4 {
		return perspective(self.fov.to_radians(), self.aspect, self.near, self.far);
	}
	fn lookat(&self) -> Mat4 {
		return lookat(self.pos, self.pos + self.front, vec3!(0, 1, 0));
	}
	fn pos(&self) -> Vec3 {
		return self.pos;
	}
}

/// orthographics camera
#[derive(Clone)]
pub struct OrthoCam {
	front: Vec3,
	pos: Vec3,
	yaw: f32,
	pitch: f32,
	width: f32,
	height: f32,
	near: f32,
	far: f32,
}

impl OrthoCam {

	/// create new orthographic camera
	pub fn new(width: f32, height: f32, near: f32, far: f32, pos: Vec3, yaw: f32, pitch: f32) -> Self {

		let mut c = Self {
			pos: vec3!(),
			front: vec3!(),
			yaw: 0.0,
			pitch: 0.0,
			width: width,
			height: height,
			near: near,
			far: far,
		};

		c.set_pos(pos);
		c.set_angle(yaw, pitch);

		return c;

	}

	/// set cam pos
	pub fn set_pos(&mut self, pos: Vec3) {
		self.pos = pos;
	}

	/// set cam facing direction
	pub fn set_front(&mut self, front: Vec3) {
		self.front = front;
	}

	/// set cam angle
	pub fn set_angle(&mut self, yaw: f32, pitch: f32) {

		self.yaw = yaw;
		self.pitch = pitch;

		self.front = vec3!(
			self.pitch.cos() * (self.yaw + 90f32.to_radians()).cos(),
			self.pitch.sin(),
			self.pitch.cos() * (self.yaw + 90f32.to_radians()).sin(),
		).normalize();

	}

	/// get cam facing direction
	pub fn front(&self) -> Vec3 {
		return self.front;
	}

	/// get cam pitch
	pub fn yaw(&self) -> f32 {
		return self.yaw;
	}

	/// get cam yaw
	pub fn pitch(&self) -> f32 {
		return self.pitch;
	}

}

impl Camera for OrthoCam {
	fn projection(&self) -> Mat4 {
		return ortho(-self.width / 2.0, self.width / 2.0, self.height / 2.0, -self.height / 2.0, self.near, self.far);
	}
	fn lookat(&self) -> Mat4 {
		return lookat(self.pos, self.pos + self.front, vec3!(0, 1, 0));
	}
	fn pos(&self) -> Vec3 {
		return self.pos;
	}
}

fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {

	let tx = -(right + left) / (right - left);
	let ty = -(top + bottom) / (top - bottom);
	let tz = -(far + near) / (far - near);

	return mat4!(
		2.0 / (right - left), 0.0, 0.0, 0.0,
		0.0, 2.0 / (top - bottom), 0.0, 0.0,
		0.0, 0.0, 2.0 / (near - far), 0.0,
		tx, ty, tz, 1.0,
	);

}

fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4 {

	let f = 1.0 / (fov / 2.0).tan();

	return mat4!(
		-f / aspect, 0.0, 0.0, 0.0,
		0.0, f, 0.0, 0.0,
		0.0, 0.0, (far + near) / (far - near), 1.0,
		0.0, 0.0, -(2.0 * far * near) / (far - near), 0.0,
	);

}

fn lookat(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {

	let z = (center - eye).normalize();
	let x = up.cross(z).normalize();
	let y = z.cross(x);

	return mat4!(
		x.x, y.x, z.x, 0.0,
		x.y, y.y, z.y, 0.0,
		x.z, y.z, z.z, 0.0,
		-x.dot(eye), -y.dot(eye), -z.dot(eye), 1.0,
	);

}

