// wengwengweng

use dirty::*;
use specs::*;

use crate::sprite::*;

pub struct AnimSys;

impl<'a> System<'a> for AnimSys {

	type SystemData = (
		WriteStorage<'a, Sprite>
	);

	fn run(&mut self, (mut sprite): Self::SystemData) {

		for (s) in (&mut sprite).join() {

			if let Some(anim) = s.current_anim {

				if s.timer >= s.speed {
					s.timer = 0.0;
					s.tick();
				} else {
					s.timer += app::dt();
				}

			}

		}

	}

}


