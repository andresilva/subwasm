use clap::{crate_authors, crate_version, AppSettings, Clap};
use std::path::PathBuf;
use subwasmlib::ChainInfo;
use wasm_loader::{OnchainBlock, Source};

/// `subwasm` allows fetching, parsing and calling some methods on WASM runtimes of Substrate based chains.
#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
	/// A level of verbosity, and can be used multiple times
	#[clap(short, long, parse(from_occurrences))]
	pub _verbose: i32,

	/// Less output
	#[clap(short, long)]
	pub quiet: bool,

	#[clap(subcommand)]
	pub subcmd: SubCommand,
}

/// You can find all available commands below.
#[derive(Clap)]
pub enum SubCommand {
	#[clap(version = crate_version!(), author = crate_authors!())]
	Get(GetOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Info(InfoOpts),

	#[clap(version = crate_version!(), author = crate_authors!(), alias("meta"))]
	Metadata(MetaOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Diff(DiffOpts),
}

/// Get/Download the runtime wasm from a running node through rpc
#[derive(Clap)]
pub struct GetOpts {
	/// The node url. Example: ws://localhost:9944 or http://localhost:9933.
	#[clap(default_value = "http://localhost:9933", required_unless_present = "chain", index = 1)]
	pub url: OnchainBlock,

	/// Provide the name of a chain and a random url amongst a list of known nodes will be used.
	/// If you pass a valid --chain, --url will be ignored
	/// --chain local = http://localhost:9933
	#[clap(long, parse(from_str), conflicts_with = "url")]
	pub chain: Option<ChainInfo>,

	/// The optional block where to fetch the runtime. That allows fetching older runtimes but you will need to connect to archive nodes.
	/// Currently, you must pass a block hash. Passing the block numbers is not supported.
	#[clap(short, long)]
	pub block: Option<String>, // TODO: can do better...

	/// You may specifiy the output filename where the runtime will be saved. If not provided, we will figure out an appropriate default name
	/// based on a counter: runtime_NNN.wasm where NNN is incrementing to make sure you do not override previous runtime. If you specify an
	/// existing file as output, it will be overwritten.
	#[clap(short, long, alias("out"), parse(from_os_str))]
	pub output: Option<PathBuf>,
}

/// The `info` command returns summarized information about a runtime.
#[derive(Clap)]
pub struct InfoOpts {
	/// The wasm file to load. It can be a path on your local filesystem such as
	/// /tmp/runtime.wasm or a node url such as http://localhost:9933 or ws://localhost:9944
	#[clap(alias("src"), default_value = "runtime_000.wasm", required_unless_present = "chain", index = 1)]
	pub source: Source,

	/// Provide the name of a chain and a random url amongst a list of known nodes will be used.
	/// If you pass a valid --chain, --url will be ignored
	/// --chain local = http://localhost:9933
	#[clap(long, parse(from_str), conflicts_with = "source")]
	pub chain: Option<ChainInfo>,

	/// Shows the list of modules if you provide `-d`
	#[clap(short, long("details-level"), parse(from_occurrences))]
	pub details_level: i32,
}

/// Returns the metadata as a json object. You may also use the "meta" alias.
#[derive(Clap)]
pub struct MetaOpts {
	/// The wasm file to load. It can be a path on your local filesystem such as
	/// /tmp/runtime.wasm or a node url such as http://localhost:9933 or ws://localhost:9944
	#[clap(alias("src"), default_value = "runtime_000.wasm", index = 1)]
	pub source: Source,
}

/// Compare 2 runtimes
#[derive(Clap)]
pub struct DiffOpts {
	/// The first source
	#[clap(index = 1, required = true)]
	pub a: Source,

	/// The second source
	#[clap(index = 2, required = true)]
	pub b: Source,
}
