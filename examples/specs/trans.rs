// wengwengweng

use dirty::*;
use dirty::math::*;
use specs::*;
use specs_derive::*;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Trans {

	pub pos: Vec2,
	pub rot: f32,
	pub scale: Vec2,

}

impl Trans {

	pub fn new(pos: Vec2, rot: f32, scale: Vec2) -> Self {
		return Self {
			pos: pos,
			rot: rot,
			scale: scale,
		}
	}

	pub fn pos(self, pos: Vec2) -> Self {
		return Self {
			pos: pos,
			..self
		}
	}

	pub fn scale(self, scale: Vec2) -> Self {
		return Self {
			scale: scale,
			..self
		}
	}

	pub fn rot(self, rot: f32) -> Self {
		return Self {
			rot: rot,
			..self
		}
	}

}

impl Default for Trans {

	fn default() -> Self {
		return Self {
			pos: vec2!(),
			rot: 0.0,
			scale: vec2!(1),
		};
	}

}
