![icon](icon.png)

## DIRTY
personal toolkit for creating game-like experiences

### usage
add to `Cargo.toml`
```toml
[dependencies.dirty]
git = "https://github.com/slmjkdbtl/DIRTY"
```
not on crates.io yet due to duplicate crate name

### modules
- **app** Lifecycles, Time and Errors
- **window** Window & Input
- **gfx** General Rendering
- **g2d** 2D Rendering
- **g3d** 3D Rendering (*)
- **audio** Sound Loading & playback
- **math** Common Math Functions & Structs
- **fs** Common File Related Functions
- **ecs** Simple ECS
- **res** Resource Loading
- **pref** Save & Load User Data
- **net** Simple Networking Wrapper (*)
- **col** Common Simple Collision Detection Functions
- **ui** Simple UI (*)

(* means not ready for use yet)

### example
```rust
use dirty::*;

fn main() {

	// init
	app::init();
	window::init("yo", 640, 480);
	audio::init();

	// main loop
	app::run(|| {

		// transform
		g2d::push();
		g2d::translate(vec2!(220, 120));
		g2d::scale(vec2!(12));
		g2d::color(color!(0, 1, 1, 1));

		// draw text
		g2d::text("yo");
		g2d::pop();

		// input
		if window::key_pressed(Key::Escape) {
			app::quit();
		}

	});

}
```
with `ecs`:

```rust
use dirty::*;
use dirty::ecs::*;
use dirty::ecs::derive::*;

#[derive(Comp, Clone)]
struct Pos {
	x: f32,
	y: f32,
}

struct MoveSys;

impl System for MoveSys {

	fn filter(&self) -> Filter {
		return filter![Pos];
	}

	fn each(&mut self, e: &mut Entity) {

		let mut pos = e.get::<Pos>();

		pos.x = pos.x + 1.0;
		println!("pos: ({}, {})", pos.x, pos.y);
		e.set::<Pos>(pos);

	}

}

fn thing(x: f32, y: f32) -> Entity {
	return entity![Pos { x, y }];
}

fn main() {

	// init modules
	app::init();

	// create new world
	let mut world = World::new();

	// add entities
	world.add(thing(0.0, 0.0));

	// run systems
	world.run(MoveSys);

	// loop
	for _ in 0..3 {
		world.update();
	}

}
```
more under `examples/`

### notes & caveats

- `app::init()` also overrides default panic behavior (also display messages to screen if `window` is initiated)
- module contexts are handled internally
- currently using OpenGL 2.1 for better compatibility, but might change to gfx-hal in the future
- on Windows, `audio::init()` must be called before `window::init()`, for some reason it crashes if tried to get audio device after SDL initialization
- ui style is a tribute to [MEKA](http://www.smspower.org/meka/), thanks for keep making awesome tools

