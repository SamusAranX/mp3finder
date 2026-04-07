use clap::Parser;
use const_format::formatcp;
use std::path::PathBuf;

const GIT_HASH: &str = env!("GIT_HASH");
const GIT_BRANCH: &str = env!("GIT_BRANCH");
const GIT_VERSION: &str = env!("GIT_VERSION");
const BUILD_DATE: &str = env!("BUILD_DATE");

const CLAP_VERSION: &str = formatcp!("{GIT_VERSION} [{GIT_BRANCH}, {GIT_HASH}, {BUILD_DATE}]");

#[derive(Parser, Debug, Clone)]
#[command(version = CLAP_VERSION, about = "Extracts common MPEG Audio files from binary blobs")]
#[allow(clippy::doc_markdown)]
pub(crate) struct Cli {
	/// The input file
	#[arg()]
	pub in_file: PathBuf,
	/// The output dir
	#[arg()]
	pub out_dir: PathBuf,

	/// The minimum amount of frames to consider a valid file. Runs with fewer than frame_limit frames will be discarded.
	/// Consider that one frame can be anywhere from ~100 to ~1000 bytes long.
	#[arg(short = 'f', long, default_value_t = 16)]
	pub frame_limit: usize,
}
