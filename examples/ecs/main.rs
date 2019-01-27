// wengwengweng

use dirty::*;
use dirty::math::*;
use dirty::ecs::*;
use dirty::addons::res;

mod comps;
mod systems;

use comps::*;
use systems::*;

fn main() {

	app::init();
	audio::init();
	window::init("yo", 640, 480);
	res::init();

// 	app::set_debug(true);
	window::scale(window::Scale::X4);
	res::load_sprites("examples/assets/", &vec!["core", "petal", "pixel"]);
	res::load_sounds("examples/assets/", &vec!["pop", "yo"]);

	let mut world = World::new();

	create_flower(&mut world, Player::One);
	create_flower(&mut world, Player::Two);

	world.run(AnimSys);
	world.run(TransformSys);
	world.run(ShootSys);
	world.run(PowderUpdateSys);
	world.run(FlowerInputSys);
	world.run(PetalFollowSys);
	world.run(RenderSys);
	world.run(DebugSys);

	app::run(|| {
		world.update();
	});

}

fn flower(p: Player, pos: Vec2) -> Entity {

	let trans = Trans::new(pos, 0.0, vec2!(1));
	let mut sprite = Sprite::new("core");
	let vel = Vel::default();
	let body = Body::new(&sprite.get_verts());
	let flower = Flower::new(p);

	sprite.color = flower.color;

	return entity![trans, sprite, vel, body, flower];

}

fn petal(flower: Id, index: u8) -> Entity {

	let trans = Trans::default();
	let mut sprite = Sprite::new("petal");
	let vel = Vel::default();
	let body = Body::new(&sprite.get_verts());
	let petal = Petal::new(flower, index);

	sprite.origin = vec2!(0.5, 1);

	return entity![trans, sprite, vel, body, petal];

}

fn rand_in_view(margin: u32) -> Vec2 {

	let (width, height) = window::size();

	return vec2!(rand!(margin, width - margin), rand!(margin, height - margin));

}

fn create_flower(s: &mut World, player: Player) {

	let f = s.add(flower(player, rand_in_view(24)));

	for i in 0..4 {
		s.add(petal(f, i));
	}

}

