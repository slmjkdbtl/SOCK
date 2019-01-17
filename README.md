# DIRTY
simple toolkit for creating game-like experiences

### usage
add to `Cargo.toml`
```toml
[dependencies.dirty]
git = "https://github.com/slmjkdbtl/DIRTY"
```
not on crates.io yet due to duplicate crate name

### example
```rust
use dirty::*;

fn main() {

	// initialize modules
	app::init();
	window::init("yo", 640, 480);
	audio::init();
	res::init();

	// load resources
	res::load_sprite("car", include_bytes!("car.png"));
	res::load_sound("pop", include_bytes!("pop.ogg"));
	res::load_sound("yo", include_bytes!("yo.ogg"));

	// play music repeatedly
	let music = audio::track(&res::sound("yo").fadein(1200).repeat());

	// main loop
	app::run(&mut || {

		// transforms
		gfx::push();
		gfx::translate(vec2!(120, 120));
		gfx::scale(vec2!(4));
		gfx::color(color!(0, 0, 1, 1));

		// draw text
		gfx::text("yo");
		gfx::pop();

		gfx::push();
		gfx::translate(vec2!(240, 240));
		gfx::color(color!());

		// draw texture
		gfx::draw(&res::sprite("car").tex, rect!(0, 0, 1, 1));
		gfx::pop();

		// input
		if window::key_released(Key::Space) {
			// play audio with effect
			audio::play(&res::sound("pop").speed(math::rand() * 2));
		}

		if window::key_pressed(Key::F) {
			window::set_fullscreen(!window::get_fullscreen())
		}

		if window::key_pressed(Key::Escape) {
			app::quit();
		}

	});

}
```
more under `examples/`

### notes & caveats

- api is not very idiomadic
- upon initialization each module has its hidden `static` state that dies when program ends
- `app::init()` also overrides the panic behavior (also display messages to screen if `window` is initiated)
- currently using OpenGL 2.1 for better compatibility, might change to gfx-rs in the future
- there's no proper custom error handling now (e.g. if you use some of the file io wrapper functions in the `fs` module an error will directly go in the error screen)
- on Windows, `audio::init()` must be called before `window::init()`, for some reason it crashes if tried to get audio device after SDL initialization

