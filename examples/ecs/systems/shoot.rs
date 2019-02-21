// wengwengweng

use dirty::*;
use dirty::math::*;
use dirty::kit::*;
use crate::comps::*;
use crate::resources::*;

fn powder(flower: Id, pos: Vec2, dir: f32) -> Entity {

	let sprite = Sprite::new("pixel");
	let trans = Trans::new(pos, 0.0, vec2!(1));
	let vel = Vel::new(vec2!(), 0.0, vec2!(1));
	let powder = Powder::new(flower, dir);

	return entity![sprite, trans, powder, vel];

}

pub fn shoot(pool: &mut Pool) {

	let mut bullets = vec![];

	for id in pool.pick(&comps![Flower, Trans]) {

		let e = pool.get_mut(id).unwrap();
		let mut f = e.get::<Flower>();
		let t = e.get::<Trans>();

		if f.energy >= f.rate {
			bullets.push(powder(id, t.pos + Vec2::from_angle(t.rot) * 8, t.rot));
			f.energy = 0;
		}

		e.set::<Flower>(f);

	}

	for b in bullets {
		pool.push(b);
	}

}

