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
pub(crate) struct Cli {
	/// The input file
	#[arg()]
	pub in_file: PathBuf,
	/// The output dir
	#[arg()]
	pub out_dir: PathBuf,
}