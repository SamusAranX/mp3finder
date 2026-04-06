use crate::mpeg::frame_header::FrameHeader;
use crate::mpeg::{AudioVersion, Layer, Protection};
use deku::{DekuRead, DekuWrite};

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct Frame {
	pub header: FrameHeader,
	#[deku(count = "header.frame_length().unwrap_or(0).saturating_sub(4)")]
	pub data: Vec<u8>,
}

impl Frame {
	pub fn is_useful_frame(&self) -> bool {
		if !self.header.has_sync_word() {
			return false;
		}

		if self.header.version == AudioVersion::Mpeg1
			&& self.header.layer == Layer::Layer1
			&& self.header.protection == Protection::NotProtected
		{
			// the first two bytes of the header are 0xFFFF, this is not what we're looking for
			return false;
		}

		// match self.header.frame_length() {
		// 	None => false,
		// 	Some(len) => len >= 1024,
		// }

		if self.header.frame_length().is_none() {
			return false;
		}

		true
	}
}
