// wengwengweng

use dirty::*;
use dirty::addons::res;
use specs::*;

mod trans;
mod vel;
mod sprite;
mod body;
mod move_sys;
mod render_sys;
mod debug_sys;

use trans::*;
use vel::*;
use sprite::*;
use body::*;
use move_sys::*;
use render_sys::*;
use debug_sys::*;

fn main() {

	app::init();
	audio::init();
	window::init("yo", 640, 480);
	res::init();

	res::load_sprites("examples/assets/", &vec!["car"]);
	res::load_sounds("examples/assets/", &vec!["pop", "yo"]);

	let mut world = World::new();

	let mut dispatcher = DispatcherBuilder::new()
		.with(MoveSys, "move", &[])
		.with_thread_local(RenderSys)
		.with_thread_local(DebugSys)
		.build();

	dispatcher.setup(&mut world.res);

	let (width, height) = window::size();

	create_car(&mut world, vec2!(rand!(width), rand!(height)));

	app::run(&mut || {
		dispatcher.dispatch(&mut world.res);
		world.maintain();
	});

}

fn create_car(world: &mut World, pos: Vec2) {

	let s = Sprite::new("car");

	world
		.create_entity()
		.with(Trans::default().pos(pos))
		.with(Vel::default())
		.with(Body::from_verts(&s.get_verts()))
		.with(s)
		.build();

}

