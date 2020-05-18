// wengwengweng

use std::rc::Rc;

use crate::*;
use math::*;
use input::*;

pub enum WindowEvent {
	Input(input::Event),
	Frame,
}

pub trait WindowCtx {

	fn run(self, _: impl FnMut(&mut Self, WindowEvent) -> Result<()> + 'static) -> Result<()>;

	fn gl(&self) -> &Rc<gl::Device>;
	fn swap(&self) -> Result<()>;

	fn key_down(&self, _: Key) -> bool;
	fn mouse_down(&self, _: Mouse) -> bool;

	fn width(&self) -> i32;
	fn height(&self) -> i32;

	fn dpi(&self) -> f32;

	fn clip_to_screen(&self, p: Vec2) -> Vec2 {
		return p * vec2!(self.width(), self.height()) * 0.5;
	}

	fn screen_to_clip(&self, p: Vec2) -> Vec2 {
		return p / 0.5 / vec2!(self.width(), self.height());
	}

// 	fn set_fullscreen(&mut self, _: bool);
// 	fn is_fullscreen(&self) -> bool;
// 	fn toggle_fullscreen(&mut self, _: bool) {
// 		self.set_fullscreen(!self.is_fullscreen());
// 	}

// 	fn set_cursor_hidden(&mut self, _: bool);
// 	fn is_cursor_hidden(&self) -> bool;
// 	fn toggle_cursor_hidden(&mut self, _: bool) {
// 		self.set_cursor_hidden(!self.is_cursor_hidden());
// 	}

// 	fn set_cursor_locked(&mut self, _: bool);
// 	fn is_cursor_locked(&self) -> bool;
// 	fn toggle_cursor_locked(&mut self, _: bool) {
// 		self.set_cursor_locked(!self.is_cursor_locked());
// 	}

// 	fn minimize(&self);

// 	fn set_title(&mut self, _: &str);
// 	fn title(&self) -> &str;

	fn mouse_pos(&self) -> Vec2;
	fn set_mouse_pos(&mut self, _: Vec2) -> Result<()>;

}

