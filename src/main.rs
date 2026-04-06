mod cli;
mod mpeg;

use crate::cli::Cli;
use crate::mpeg::Frame;
use clap::Parser;
use deku::{DekuContainerRead, DekuContainerWrite};
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom, Write};
use std::path::Path;

fn main() {
	let cli = Cli::parse();
	work(&cli.in_file, &cli.out_dir);
}

fn work(in_file: &Path, out_dir: &Path) {
	let in_file = File::open(in_file).expect("Couldn't open file!");
	let file_length = in_file
		.metadata()
		.expect("Couldn't query file metadata!")
		.len();
	let mut reader = BufReader::new(in_file);

	let mut frame_buffer: Vec<Frame> = Vec::new();
	let mut frame_index = 1;
	let mut frames_start: u64 = 0;

	loop {
		let pos = reader
			.stream_position()
			.expect("The reader got disoriented!");

		if pos == file_length - 3 {
			break;
		}

		let mut got_useful_frame = false;
		match Frame::from_reader((&mut reader, 0)) {
			Ok((_, frame)) => 'frame: {
				if !frame.is_useful_frame() {
					break 'frame;
				}

				if frame_buffer.is_empty() {
					frames_start = pos;
				}

				frame_buffer.push(frame);
				got_useful_frame = true;
			}
			Err(e) => {
				eprintln!("Frame construction error: {e:?}");
			}
		}

		if !got_useful_frame {
			match frame_buffer.len() {
				// frame buffer is empty, continue
				0 => (),

				// frame buffer has only one frame, discard it
				1 => frame_buffer.clear(),

				// frame buffer has more than one frame. this could be interesting
				_ => {
					let out_file_path = out_dir.join(format!("track{frame_index}.mp3"));
					let mut out_file = File::create(&out_file_path).expect("Couldn't create new file!");

					for frame in &frame_buffer {
						let frame_bytes = frame.to_bytes().expect("Couldn't get frame bytes!");
						_ = out_file
							.write(&frame_bytes)
							.expect("Couldn't write frame to file!");
					}

					let new_pos = reader
						.stream_position()
						.expect("The reader got disoriented!");

					eprintln!(
						"Wrote {} frames (0x{frames_start:08X}-0x{new_pos:08X}) to {}",
						frame_buffer.len(),
						out_file_path.display()
					);

					frame_buffer.clear();
					frame_index += 1;
				}
			}

			// Move the reader back to its initial position, advanced by one
			_ = reader.seek(SeekFrom::Start(pos + 1));
		}
	}

	eprintln!("Reached EOF");
}
