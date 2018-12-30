// wengwengweng

#![allow(unused_parens)]
#![allow(dead_code)]

mod ctx;
mod utils;

pub mod app;
pub mod gfx;
pub mod audio;
pub mod res;
pub mod col;
pub mod lua;
pub mod math;
pub mod fs;

pub use sdl2::keyboard::Scancode as Key;
pub use sdl2::mouse::MouseButton as Mouse;

