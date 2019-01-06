// wengwengweng

//! # DIRTY
//! simple toolkit for creating game-like experiences

#![crate_name = "dirty"]
#![crate_type = "lib"]

#![allow(unused_parens)]
#![allow(dead_code)]

#[macro_use]
mod ctx;
#[macro_use]
mod utils;
pub mod fs;

pub mod app;
pub mod window;
pub mod gfx;
pub mod audio;
pub mod res;
pub mod ecs;
pub mod col;
pub mod math;
pub mod lua;

pub use sdl2::keyboard::Scancode as Key;
pub use sdl2::mouse::MouseButton as Mouse;

