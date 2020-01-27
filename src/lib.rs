// wengwengweng

//! # DIRTY
//! Toolkit for Games & Interactive Experiences

#![feature(clamp)]
#![feature(const_fn)]
#![feature(try_blocks)]
#![feature(box_syntax)]
#![feature(trait_alias)]
#![feature(bool_to_option)]
#![feature(fixed_size_array)]
#![feature(type_alias_impl_trait)]

#![allow(unused_parens)]
#![deny(clippy::implicit_return)]

#[macro_use]
pub mod helpers;

pub mod task;

#[cfg(feature = "app")]
pub mod math;
#[cfg(feature = "app")]
pub mod gl;
#[cfg(feature = "app")]
pub mod app;

#[cfg(feature = "fs")]
pub mod fs;

#[cfg(feature = "img")]
pub mod img;

#[cfg(feature = "audio")]
pub mod audio;

#[cfg(feature = "physics")]
pub mod physics;

#[cfg(feature = "synth")]
pub mod synth;

#[cfg(feature = "http")]
pub mod http;

#[cfg(all(feature = "term", desktop))]
pub mod term;

#[cfg(all(feature = "serial", desktop))]
pub mod serial;

#[cfg(feature = "ase")]
pub mod ase;

#[cfg(feature = "texpack")]
pub mod texpack;

pub mod codec;

pub type Error = String;
pub type Result<T> = std::result::Result<T, Error>;

