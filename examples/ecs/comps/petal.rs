// wengwengweng

use dirty::kit::*;

#[derive(Clone)]
pub struct Petal {

	pub flower: Id,
	pub index: u8

}

impl Petal {

	pub fn new(flower: Id, index: u8) -> Self {

		return Self {

			flower: flower,
			index: index,

		};

	}

}

