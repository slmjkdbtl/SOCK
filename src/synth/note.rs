// wengwengweng

use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Note {
	life: f32,
	afterlife: f32,
	released: bool,
	dead: bool,
	amp: f32,
	envelope: Envelope,
}

impl Note {

	pub fn new(e: Envelope) -> Self {
		return Self {
			life: 0.0,
			afterlife: 0.0,
			released: false,
			dead: false,
			amp: 0.0,
			envelope: e,
		};
	}

	pub fn update(&mut self, dt: f32) {

		if !self.released {
			self.life += dt;
		} else {
			self.afterlife += dt;
		}

		let e = self.envelope;

		if !self.released {
			if self.life < e.attack {
				self.amp = self.life / e.attack;
			} else if self.life >= e.attack && self.life < e.attack + e.decay {
				self.amp = 1.0 - (self.life - e.attack) / e.decay * (1.0 - e.sustain);
			} else {
				self.amp = e.sustain;
			}
		} else {
			if e.release == 0.0 {
				self.amp = 0.0;
			} else {
				self.amp = e.sustain - (self.afterlife / e.release) * e.sustain;
			}
		}

		if self.released {
			if self.afterlife > e.release {
				self.amp = 0.0;
				self.dead = true;
			}
		}

	}

	pub fn release(&mut self) {
		self.released = true;
	}

	pub fn dead(&self) -> bool {
		return self.dead;
	}

	pub fn amp(&self) -> f32 {
		return self.amp;
	}

}

