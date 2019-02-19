// wengwengweng

use dirty::ecs::*;

#[derive(Clone)]
pub struct Powder {

	pub flower: Id,
	pub speed: f32,
	pub dir: f32,

}

impl Powder {

	pub fn new(flower: Id, dir: f32) -> Self {

		return Self {

			flower: flower,
			speed: 12.0,
			dir: dir,

		};

	}

}

