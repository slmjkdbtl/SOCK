// wengwengweng

//! Sound Loading & playback

use std::io::Cursor;
use std::time::Duration;

use rodio::Source;
use rodio::Decoder;
use rodio::Sink;
use rodio::source::Buffered;

use crate::Result;

#[derive(Clone)]
pub struct Sound {
	buffer: Buffered<Decoder<Cursor<Vec<u8>>>>,
	effect: Effect,
}

#[derive(Clone, Copy)]
struct Effect {
	speed: f32,
	volume: f32,
	repeat: bool,
	fadein: f32,
}

pub struct Track {
	sink: Sink,
}

fn get_device() -> Result<rodio::Device> {
	return rodio::default_output_device()
		.ok_or(format!("failed to get audio output device"));
}

pub fn play(s: &Sound) -> Result<()> {
	return s.play();
}

impl Default for Effect {
	fn default() -> Self {
		return Self {
			speed: 1.0,
			volume: 1.0,
			repeat: false,
			fadein: 0.0,
		};
	}
}

impl Sound {

	pub fn from_bytes(data: &[u8]) -> Result<Self> {

		let cursor = Cursor::new(data.to_owned());
		let source = Decoder::new(cursor)
			.map_err(|_| format!("failed to parse sound from file"))?;

		return Ok(Self {
			buffer: source.buffered(),
			effect: Effect::default(),
		});

	}

	pub fn play(&self) -> Result<()> {
		return Ok(rodio::play_raw(&get_device()?, self.apply().convert_samples()));
	}

	pub fn as_track(&self) -> Result<Track> {
		return Track::from_sound(&self);
	}

	pub fn speed(&self, s: f32) -> Self {
		assert!(s > 0.0 && s <= 2.0, "invalid speed");
		return Self {
			effect: Effect {
				speed: s,
				.. self.effect
			},
			buffer: self.buffer.clone(),
		}
	}

	pub fn volume(&self, v: f32) -> Self {
		assert!(v >= 0.0 && v <= 2.0, "invalid volume");
		return Self {
			effect: Effect {
				volume: v,
				.. self.effect
			},
			buffer: self.buffer.clone(),
		}
	}

	pub fn repeat(&self) -> Self {
		return Self {
			effect: Effect {
				repeat: true,
				.. self.effect
			},
			buffer: self.buffer.clone(),
		}
	}

	pub fn fadein(&self, time: f32) -> Self {
		return Self {
			effect: Effect {
				fadein: time,
				.. self.effect
			},
			buffer: self.buffer.clone(),
		}
	}

	fn apply(&self) -> Box<dyn Source<Item = i16> + Send> {

		type S = dyn Source<Item = i16> + Send;

		let s = box self.buffer.clone();
		let effect = self.effect;

		let s: Box<S> = if effect.speed != 0.0 {
			box s.speed(effect.speed)
		} else {
			s
		};

		let s: Box<S> = if effect.volume != 0.0 {
			box s.amplify(effect.volume)
		} else {
			s
		};

		let s: Box<S> = if effect.fadein != 0.0 {
			box s.fade_in(Duration::from_secs_f32(effect.fadein))
		} else {
			s
		};

		let s: Box<S> = if effect.repeat {
			box s.repeat_infinite()
		} else {
			s
		};

		return s;

	}

}

impl Track {

	pub fn from_bytes(data: &[u8]) -> Result<Self> {
		return Self::from_sound(&Sound::from_bytes(data)?);
	}

	pub fn from_sound(sound: &Sound) -> Result<Self> {

		let device = get_device()?;
		let sink = Sink::new(&device);

		sink.append(sound.apply());
		sink.pause();

		return Ok(Self {
			sink,
		});

	}

	pub fn is_playing(&self) -> bool {
		return !self.sink.is_paused();
	}

	pub fn pause(&self) {
		self.sink.pause();
	}

	pub fn play(&self) {
		self.sink.play();
	}

}

