// wengwengweng

use crate::*;

/// The Main Trait
pub trait State: 'static + Sized {

	fn init(_: &mut Ctx) -> Result<Self>;

	fn event(&mut self, _: &mut Ctx, _: &input::Event) -> Result<()> {
		return Ok(());
	}

	fn update(&mut self, _: &mut Ctx) -> Result<()> {
		return Ok(());
	}

	fn draw(&mut self, _: &mut Ctx) -> Result<()> {
		return Ok(());
	}

}

impl State for () {
	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(());
	}
}

