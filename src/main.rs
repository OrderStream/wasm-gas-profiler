use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use wasm_gas_profiler::{GasReporter, WasmCostModel, WasmGasAnalyzer};

#[derive(Parser, Debug)]
#[command(
    name = "wasm-gas-profiler",
    author = "OrderStream <healthdecoded77@gmail.com>",
    version = "0.1.0",
    about = "Instruction-level WASM opcode analyzer and CPU fuel profiler for Soroban contracts"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Profiles instruction opcodes, fuel consumption, loops, and storage host calls.
    Profile {
        #[arg(short, long)]
        wasm: Option<PathBuf>,

        #[arg(long)]
        out_json: Option<PathBuf>,

        #[arg(long)]
        out_flamegraph: Option<PathBuf>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Profile {
            wasm,
            out_json,
            out_flamegraph,
        } => {
            let bytes = if let Some(path) = wasm {
                fs::read(&path)?
            } else {
                // Mock Soroban WASM
                let mut mock = Vec::new();
                mock.extend_from_slice(b"\0asm\x01\0\0\0");
                mock
            };

            let analyzer = WasmGasAnalyzer::new(WasmCostModel::default());
            let profile = analyzer.analyze_bytes(&bytes)?;

            let text_report = GasReporter::format_text_report(&profile);
            println!("{}", text_report);

            if let Some(json_path) = out_json {
                let j = serde_json::to_string_pretty(&profile)?;
                fs::write(&json_path, j)?;
                println!("📄 Exported JSON profile to: {}", json_path.display());
            }

            if let Some(flame_path) = out_flamegraph {
                let lines = GasReporter::to_flamegraph_lines(&profile);
                fs::write(&flame_path, lines)?;
                println!("🔥 Exported flamegraph data to: {}", flame_path.display());
            }
        }
    }

    Ok(())
}
