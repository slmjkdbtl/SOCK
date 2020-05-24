// wengwengweng

use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::vec;

use lewton::inside_ogg::OggStreamReader;

use super::*;

pub struct VorbisDecoder<R: Read + Seek> {
	decoder: OggStreamReader<R>,
	cur_packet: Option<vec::IntoIter<i16>>,
	channel_count: ChannelCount,
	sample_rate: u32,
}

impl<R: Read + Seek> VorbisDecoder<R> {

	pub fn new(data: R) -> Result<Self> {

		let mut decoder = OggStreamReader::new(data)
			.map_err(|_| format!("failed to parse vorbis"))?;

		let header = &decoder.ident_hdr;

		let channel_count = match header.audio_channels {
			1 => ChannelCount::One,
			2 => ChannelCount::Two,
			_ => return Err(format!("unsupported channel count: {}", header.audio_channels)),
		};

		let sample_rate = header.audio_sample_rate;

		let data = match decoder.read_dec_packet_itl() {
			Ok(data) => data,
			Err(e) => return Err(format!("failed to read vorbis")),
		};

		return Ok(Self {
			decoder: decoder,
			cur_packet: data.map(|d| d.into_iter()),
			channel_count: channel_count,
			sample_rate: sample_rate,
		});

	}

	fn next_sample(&mut self) -> Option<f32> {

		let cur_packet = match &mut self.cur_packet {
			Some(packet) => packet,
			None => return None,
		};

		if let Some(sample) = cur_packet.next() {
			return Some(utils::i16_to_f32(sample));
		} else {
			self.cur_packet = self.decoder
				.read_dec_packet_itl()
				.ok()
				.flatten()
				.map(|v| v.into_iter());
			return self.next_sample();
		}

	}

}

impl<R: Read + Seek> Source for VorbisDecoder<R> {
	fn sample_rate(&self) -> u32 {
		return self.sample_rate;
	}
}

impl<R: Read + Seek> Iterator for VorbisDecoder<R> {

	type Item = Frame;

	fn next(&mut self) -> Option<Self::Item> {

		let sample = match self.next_sample() {
			Some(sample) => sample,
			None => return None,
		};

		return Some(match self.channel_count {
			ChannelCount::One => Frame::new(sample, sample),
			ChannelCount::Two => Frame::new(sample, self.next_sample().unwrap_or(0.0)),
		});

	}

}

pub fn is_vorbis<R: Read + Seek>(mut reader: R) -> Result<bool> {

	let is_vorbis = OggStreamReader::new(&mut reader).is_ok();

	reader
		.seek(SeekFrom::Start(0))
		.map_err(|_| format!("failed to seek"))?;

	return Ok(is_vorbis)

}

