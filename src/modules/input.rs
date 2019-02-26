// wengwengweng

use std::collections::HashMap;

use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;

use crate::*;
use crate::math::*;

// context
ctx!(INPUT: InputCtx);

struct InputCtx {

	events: sdl2::EventPump,
	key_states: HashMap<Scancode, ButtonState>,
	mouse_states: HashMap<MouseButton, ButtonState>,
	mouse_delta: Vec2,
	mouse_pos: Vec2,

}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ButtonState {
	Up,
	Pressed,
	Down,
	Released,
}

pub(super) fn init(e: sdl2::EventPump) {
	ctx_init(InputCtx {
		events: e,
		key_states: HashMap::new(),
		mouse_states: HashMap::new(),
		mouse_delta: vec2!(),
		mouse_pos: vec2!(),
	});
}

pub(super) fn poll() {

	use sdl2::event::Event;

	let input = ctx_get();
	let input_mut = ctx_get_mut();
	let keyboard_state = input.events.keyboard_state();
	let mouse_state = input.events.mouse_state();
	let rmouse_state = input.events.relative_mouse_state();

	input_mut.mouse_delta = vec2!(rmouse_state.x(), rmouse_state.y());
	input_mut.mouse_pos = vec2!(mouse_state.x(), mouse_state.y());

	for (code, state) in &mut input_mut.key_states {
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

	for (code, state) in &mut input_mut.mouse_states {
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

	for code in keyboard_state.pressed_scancodes() {
		if !input.key_states.contains_key(&code) || input.key_states[&code] == ButtonState::Up {
			input_mut.key_states.insert(code, ButtonState::Pressed);
		}
	}

	for code in mouse_state.pressed_mouse_buttons() {
		if !input.mouse_states.contains_key(&code) || input.mouse_states[&code] == ButtonState::Up {
			input_mut.mouse_states.insert(code, ButtonState::Pressed);
		}
	}

	for event in input_mut.events.poll_iter() {
		match event {
			Event::Quit {..} => {
				app::quit();
			},
			_ => {}
		}
	}

}

/// get list of pressed keys
pub fn pressed_keys() -> Vec<Scancode> {

	let window = ctx_get();
	let states = &window.key_states;

	return states
		.keys()
		.filter(|&k| states[k] == ButtonState::Down )
		.map(|k| *k)
		.collect();

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

/// get mouse position
pub fn mouse_pos() -> Vec2 {
	return ctx_get().mouse_pos;
}

/// get mouse delta position
pub fn mouse_delta() -> Vec2 {
	return ctx_get().mouse_delta;
}

fn check_key_state(code: Scancode, state: ButtonState) -> bool {
	if let Some(s) = ctx_get().key_states.get(&code) {
		return s == &state;
	} else {
		return false;
	}
}

fn check_mouse_state(code: MouseButton, state: ButtonState) -> bool {
	if let Some(s) = ctx_get().mouse_states.get(&code) {
		return s == &state;
	} else {
		return false;
	}
}

