mod cli;
mod mpeg;

use crate::cli::Cli;
use crate::mpeg::Frame;
use clap::Parser;
use deku::{DekuContainerRead, DekuContainerWrite, DekuError};
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom, Write};
use std::path::Path;
use std::time::Instant;

fn main() {
	let cli = Cli::parse();
	work(&cli.in_file, &cli.out_dir, cli.frame_limit);
}

fn work(in_file: &Path, out_dir: &Path, frame_limit: usize) {
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
				match e {
					// explicitly ignore Incomplete errors.
					// these happen sometimes because of how deku works but they mean nothing.
					DekuError::Incomplete(_) => (),
					_ => eprintln!("Frame construction error at 0x{pos:08X}: {e:?}"),
				}
			}
		}

		if !got_useful_frame {
			if (1..frame_limit).contains(&frame_buffer.len()) {
				// this is not a number of frames we're interested in, discard them
				frame_buffer.clear();
			} else if frame_buffer.len() >= frame_limit {
				let out_file_path = out_dir.join(format!("track{frame_index}.mp3"));
				let mut out_file = File::create(&out_file_path).expect("Couldn't create new file!");

				let mut written_bytes_total = 0;

				for frame in &frame_buffer {
					let frame_bytes = frame.to_bytes().expect("Couldn't get frame bytes!");
					let written_bytes = out_file
						.write(&frame_bytes)
						.expect("Couldn't write frame to file!");

					written_bytes_total += written_bytes;
				}

				let new_pos = reader
					.stream_position()
					.expect("The reader got disoriented!");

				eprintln!(
					"Wrote {} frames ({written_bytes_total} bytes, 0x{frames_start:08X}-0x{new_pos:08X}) to {}",
					frame_buffer.len(),
					out_file_path.file_name().unwrap().display()
				);

				frame_buffer.clear();
				frame_index += 1;
			}

			// Move the reader back to its initial position, advanced by one
			_ = reader.seek(SeekFrom::Start(pos + 1));
		}
	}

	eprintln!("Reached EOF");
}
