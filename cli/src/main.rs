mod opts;

use clap::Clap;
use opts::*;
use rand::seq::SliceRandom;
use std::path::PathBuf;
use subwasmlib::*;
use wasm_loader::*;
use wasm_testbed::*;

/// Main entry point of the `subwasm` cli.
fn main() -> color_eyre::Result<()> {
	let opts: Opts = Opts::parse();

	match opts.subcmd {
		SubCommand::Get(get_opts) => {
			let urls = match get_opts.chain.as_ref() {
				"polkadot" => vec![
					"wss://rpc.polkadot.io",
					"wss://polkadot.api.onfinality.io/public-ws",
					"wss://polkadot.elara.patract.io",
				],
				"kusama" => vec!["wss://kusama-rpc.polkadot.io"],
				"westend" => vec!["wss://westend-rpc.polkadot.io"],
				"rococo" => vec!["wss://rococo-rpc.polkadot.io"],
				"wococo" => vec!["wss://wococo-rpc.polkadot.io"],
				_ => vec![get_opts.url.as_ref()],
			};

			let url = urls.choose(&mut rand::thread_rng()).expect("Picking a node");
			println!("Getting runtime from a node at {:?}", url);
			download_runtime(url, get_opts.block, get_opts.output)?;
		}

		SubCommand::Info(info_opts) => {
			let src = Source::File(PathBuf::from(&info_opts.input));
			let runtime = WasmTestBed::new(&src)
				.map_err(|e| {
					eprintln!("{}", e);
					if let WasmTestbedError::Decoding(data) = e {
						print_magic_and_version(&data);
					}
					const REPO: &str = env!("CARGO_PKG_REPOSITORY");
					const NAME: &str = env!("CARGO_PKG_NAME");
					const VERSION: &str = env!("CARGO_PKG_VERSION");
					println!("🗣️ If you think it should have worked, please open an issue at {}/issues", REPO);
					println!("and attach your runtime and mention using {} v{}", NAME, VERSION);
					panic!("Could not load runtime");
				})
				.unwrap();

			match info_opts.details_level {
				0 => {
					// println!(
					// 	"Version {:?} {} supported.",
					// 	runtime.metadata_version(),
					// 	if runtime.is_supported() { "is" } else { "is NOT" }
					// );
					// display_infos(runtime.runtime_metadata_prefixed())?;
					print_runtime_infos(src);
				}
				_ => {
					display_modules_list(runtime.runtime_metadata_prefixed())?;
				}
			}
		}

		SubCommand::Metadata(meta_opts) => {
			let src = Source::File(PathBuf::from(&meta_opts.input));
			let runtime = WasmTestBed::new(&src).expect("Loading runtime to testbed");
			display_raw_metadata(runtime.metadata())?;
		}

		SubCommand::Diff(diff_opts) => {
			diff(diff_opts.a, diff_opts.b);
		}
	};

	Ok(())
}
