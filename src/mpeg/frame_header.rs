use crate::mpeg::bitrates::{
	BITRATES_V1_L1, BITRATES_V1_L2, BITRATES_V1_L3, BITRATES_V2_L1, BITRATES_V2_L2_L3,
};
use crate::mpeg::enums::{AudioVersion, Bitrate, ChannelMode, Emphasis, Layer, Protection};
use crate::mpeg::samplerates::{SAMPLE_RATES_V1, SAMPLE_RATES_V2, SAMPLE_RATES_V2_5};
use deku::{DekuRead, DekuWrite};

const SYNC_WORD: u16 = 0b11111111111;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct FrameHeader {
	#[deku(endian = "big", bits = 11)]
	pub frame_sync: u16,
	pub version: AudioVersion,
	pub layer: Layer,
	pub protection: Protection,
	#[deku(bits = 4)]
	pub bitrate_index: u8,
	#[deku(bits = 2)]
	pub sample_rate_index: u8,
	#[deku(bits = 1)]
	pub padding_bit: bool,
	#[deku(bits = 1)]
	pub private: u8,
	pub channel_mode: ChannelMode,
	#[deku(bits = 2)]
	pub mode_extension: u8,
	#[deku(bits = 1)]
	pub copyright: bool,
	#[deku(bits = 1)]
	pub original: bool,
	pub emphasis: Emphasis,
}

impl FrameHeader {
	pub fn has_sync_word(&self) -> bool {
		self.frame_sync & SYNC_WORD == SYNC_WORD
	}

	/// The bitrate in kbps. Returns `Free` for arbitrary bitrate or `Bad` for invalid values.
	pub fn bitrate(&self) -> Option<Bitrate> {
		if self.bitrate_index == 0 {
			return Some(Bitrate::Free);
		} else if self.bitrate_index == 0b1111 {
			return Some(Bitrate::Bad);
		}

		let array_index = (self.bitrate_index - 1) as usize;
		match (&self.version, &self.layer) {
			(AudioVersion::Mpeg1, Layer::Layer1) => Some(BITRATES_V1_L1[array_index]),
			(AudioVersion::Mpeg1, Layer::Layer2) => Some(BITRATES_V1_L2[array_index]),
			(AudioVersion::Mpeg1, Layer::Layer3) => Some(BITRATES_V1_L3[array_index]),
			(AudioVersion::Mpeg2 | AudioVersion::Mpeg2_5, Layer::Layer1) => Some(BITRATES_V2_L1[array_index]),
			(AudioVersion::Mpeg2 | AudioVersion::Mpeg2_5, Layer::Layer2 | Layer::Layer3) => {
				Some(BITRATES_V2_L2_L3[array_index])
			}
			(_, _) => None,
		}
	}

	/// The sample rate in Hz. Returns `None` for invalid configurations.
	pub fn sample_rate(&self) -> Option<u16> {
		if self.sample_rate_index == 0b11 {
			return None;
		}

		let array_index = self.sample_rate_index as usize;
		match self.version {
			AudioVersion::Mpeg2_5 => Some(SAMPLE_RATES_V2_5[array_index]),
			AudioVersion::Mpeg2 => Some(SAMPLE_RATES_V2[array_index]),
			AudioVersion::Mpeg1 => Some(SAMPLE_RATES_V1[array_index]),
			AudioVersion::Reserved => None,
		}
	}

	fn padding_size(&self) -> u8 {
		match (self.padding_bit, &self.layer) {
			(false, _) | (_, Layer::Reserved) => 0,
			(true, Layer::Layer1) => 4,
			(true, Layer::Layer2 | Layer::Layer3) => 1,
		}
	}

	pub fn frame_length(&self) -> Option<usize> {
		let padding = self.padding_size() as usize;

		match (&self.layer, self.bitrate(), self.sample_rate()) {
			(Layer::Layer1, Some(Bitrate::Kbps(kbps)), Some(sample_rate)) => {
				Some((12 * (kbps as usize * 1000) / sample_rate as usize + padding) * 4)
			}
			(Layer::Layer2 | Layer::Layer3, Some(Bitrate::Kbps(kbps)), Some(sample_rate)) => {
				Some(144 * (kbps as usize * 1000) / sample_rate as usize + padding)
			}
			_ => None,
		}
	}
}
