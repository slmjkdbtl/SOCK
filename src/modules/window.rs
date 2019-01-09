// wengwengweng

//! Window & Events

use std::collections::HashMap;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;
use sdl2::video::{Window, FullscreenType, SwapInterval};

use crate::*;

// context
ctx!(WINDOW: WindowCtx);

struct WindowCtx {

	sdl_ctx: sdl2::Sdl,
	window: Window,
	#[allow(dead_code)]
	gl_ctx: sdl2::video::GLContext,
	events: sdl2::EventPump,
	key_states: HashMap<Scancode, ButtonState>,
	mouse_states: HashMap<MouseButton, ButtonState>,
	size: (u32, u32),

}

/// start window with title, width, and height
pub fn init(title: &str, width: u32, height: u32) {

	if !app::enabled() {
		panic!("can't init window without app");
	}

	let sdl_ctx = sdl2::init().expect("failed to init SDL context");
	let video = sdl_ctx.video().expect("failed to init SDL video subsystem");
	let gl_attr = video.gl_attr();

	gl_attr.set_context_profile(sdl2::video::GLProfile::Compatibility);
	gl_attr.set_context_version(2, 1);

	let window = video.window(title, width, height)
		.opengl()
		.resizable()
		.build()
		.expect("failed to create window");

	let gl_ctx = window.gl_create_context().expect("failed to create OpenGL context");

	gl::load_with(|name| {
		video.gl_get_proc_address(name) as *const std::os::raw::c_void
	});

	video.gl_set_swap_interval(SwapInterval::VSync).expect("vsync failed");

	ctx_init(WindowCtx {

		events: sdl_ctx.event_pump().expect("failed to create event pump"),
		window: window,
		gl_ctx: gl_ctx,
		sdl_ctx: sdl_ctx,
		key_states: HashMap::new(),
		mouse_states: HashMap::new(),
		size: (width, height),

	});

	gfx::init();

}

/// check if window is initiated
pub fn enabled() -> bool {
	return ctx_is_ok();
}

pub(crate) fn poll_events() {

	let window = ctx_get();
	let window_mut = ctx_get_mut();
	let keyboard_state = window.events.keyboard_state();
	let mouse_state = window.events.mouse_state();

	for (code, state) in &mut window_mut.key_states {
		match state {
			ButtonState::Pressed => {
				*state = ButtonState::Down;
			},
			ButtonState::Released => {
				*state = ButtonState::Up;
			},
			ButtonState::Down => {
				if !keyboard_state.is_scancode_pressed(*code) {
					*state = ButtonState::Released;
				}
			},
			_ => {}
		}
	}

	for (code, state) in &mut window_mut.mouse_states {
		match state {
			ButtonState::Pressed => {
				*state = ButtonState::Down;
			},
			ButtonState::Released => {
				*state = ButtonState::Up;
			},
			ButtonState::Down => {
				if !mouse_state.is_mouse_button_pressed(*code) {
					*state = ButtonState::Released;
				}
			},
			_ => {}
		}
	}

	for event in window_mut.events.poll_iter() {

		match event {

			Event::Quit {..} => {
				app::quit();
			},

			Event::KeyDown { repeat: false, .. } => {
				for code in keyboard_state.pressed_scancodes() {
					if !window.key_states.contains_key(&code) || window.key_states[&code] == ButtonState::Up {
						window_mut.key_states.insert(code, ButtonState::Pressed);
					}
				}
			},

			Event::MouseButtonDown { .. } => {
				for code in mouse_state.pressed_mouse_buttons() {
					if !window.mouse_states.contains_key(&code) || window.mouse_states[&code] == ButtonState::Up {
						window_mut.mouse_states.insert(code, ButtonState::Pressed);
					}
				}
			},

			_ => {}

		}

	}

}

#[derive(Debug, PartialEq)]
enum ButtonState {
	Up,
	Pressed,
	Down,
	Released,
}

/// set window fullscreen state
pub fn set_fullscreen(b: bool) {

	let app_mut = ctx_get_mut();

	if b {
		app_mut.window.set_fullscreen(FullscreenType::Desktop).expect("fullscreen failed");
	} else {
		app_mut.window.set_fullscreen(FullscreenType::Off).expect("fullscreen failed");
	}

}

/// get window fullscreen state
pub fn get_fullscreen() -> bool {
	return ctx_get().window.fullscreen_state() == FullscreenType::Desktop;
}

/// show cursor
pub fn show_cursor() {
	ctx_get_mut().sdl_ctx.mouse().show_cursor(true);
}

/// hide cursor
pub fn hide_cursor() {
	ctx_get_mut().sdl_ctx.mouse().show_cursor(false);
}

/// set mouse relative state
pub fn set_relative(b: bool) {
	ctx_get_mut().sdl_ctx.mouse().set_relative_mouse_mode(b);
}

/// get mouse relative state
pub fn get_relative() -> bool {
	return ctx_get().sdl_ctx.mouse().relative_mouse_mode();
}

/// get window size
pub fn size() -> (u32, u32) {
	return ctx_get().size;
}

/// check if a key was pressed this frame
pub fn key_pressed(k: Scancode) -> bool {
	return check_key_state(k, ButtonState::Pressed);
}

/// check if a key is holding down
pub fn key_down(k: Scancode) -> bool {
	return check_key_state(k, ButtonState::Down);
}

/// check if a key was released this frame
pub fn key_released(k: Scancode) -> bool {
	return check_key_state(k, ButtonState::Released);
}

/// check if a mouse button was pressed this frame
pub fn mouse_pressed(b: MouseButton) -> bool {
	return check_mouse_state(b, ButtonState::Pressed);
}

/// check if a mouse button is holding down
pub fn mouse_down(b: MouseButton) -> bool {
	return check_mouse_state(b, ButtonState::Down);
}

/// check if a mouse button was released this frame
pub fn mouse_released(b: MouseButton) -> bool {
	return check_mouse_state(b, ButtonState::Released);
}

pub(crate) fn swap() {
	ctx_get().window.gl_swap_window();
}

fn check_key_state(code: Scancode, state: ButtonState) -> bool {

	match ctx_get().key_states.get(&code) {
		Some(s) => {
			return *s == state;
		}
		None => {
			return false;
		}
	}

}

fn check_mouse_state(code: MouseButton, state: ButtonState) -> bool {

	match ctx_get().mouse_states.get(&code) {
		Some(s) => {
			return *s == state;
		}
		None => {
			return false;
		}
	}

}

