use deku::{DekuRead, DekuWrite};

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 2)]
pub enum AudioVersion {
	#[deku(id = 0b00)]
	Mpeg2_5,
	#[deku(id = 0b01)]
	Reserved,
	#[deku(id = 0b10)]
	Mpeg2,
	#[deku(id = 0b11)]
	Mpeg1,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 2)]
pub enum Layer {
	#[deku(id = 0b00)]
	Reserved,
	#[deku(id = 0b01)]
	Layer3,
	#[deku(id = 0b10)]
	Layer2,
	#[deku(id = 0b11)]
	Layer1,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 1)]
pub enum Protection {
	#[deku(id = 0b00)]
	Protected,
	#[deku(id = 0b01)]
	NotProtected,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Bitrate {
	Free,
	Bad,
	Kbps(u16),
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 2)]
pub enum ChannelMode {
	#[deku(id = 0b00)]
	Stereo,
	#[deku(id = 0b01)]
	JointStereo,
	#[deku(id = 0b10)]
	DualMono,
	#[deku(id = 0b11)]
	Mono,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 2)]
pub enum Emphasis {
	#[deku(id = 0b00)]
	None,
	#[deku(id = 0b01)]
	FiftyFifteenMicroseconds,
	#[deku(id = 0b10)]
	Reserved,
	#[deku(id = 0b11)]
	CCITJ17,
}