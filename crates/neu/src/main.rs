use std::{path::PathBuf, process::ExitCode, str::FromStr};

use clap::{Args, Parser, Subcommand};
use compiler::driver::compile_manifest_to_executable;
use target_lexicon::Triple;

#[derive(Debug, Parser)]
#[command(name = "neu", version, about = "Neu compiler tooling")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(about = "Build a Neu project")]
    Build(BuildArgs),
}

#[derive(Debug, Args)]
struct BuildArgs {
    #[arg(value_name = "PROJECT_OR_MANIFEST")]
    project: Option<PathBuf>,
    #[arg(long, value_name = "PATH")]
    output: Option<PathBuf>,
    #[arg(long, value_name = "TARGET")]
    target: Option<String>,
}

fn main() -> ExitCode {
    match run() {
        Ok(output) => {
            println!("{}", output.display());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("neu: {error}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<PathBuf, String> {
    let Cli { command } = Cli::parse();
    match command {
        Command::Build(args) => build(args),
    }
}

fn build(args: BuildArgs) -> Result<PathBuf, String> {
    let manifest_path = match args.project {
        Some(path) if path.is_file() => path,
        Some(path) if path.is_dir() => compiler::manifest::ProjectManifest::discover(path)
            .map_err(|error| error.detail().to_owned())?,
        Some(path) => return Err(format!("project path does not exist: {}", path.display())),
        None => compiler::manifest::ProjectManifest::discover(
            std::env::current_dir().map_err(|error| error.to_string())?,
        )
        .map_err(|error| error.detail().to_owned())?,
    };
    let (manifest, root) = compiler::manifest::ProjectManifest::load(&manifest_path)
        .map_err(|error| error.detail().to_owned())?;
    let output = args.output.unwrap_or_else(|| {
        root.join("target")
            .join(safe_executable_name(manifest.name()))
    });
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let target = args
        .target
        .map(|target| Triple::from_str(&target).map_err(|error| error.to_string()))
        .transpose()?
        .unwrap_or_else(Triple::host);
    let target_packs = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("target-packs");
    compile_manifest_to_executable(&manifest_path, target_packs, target, &output)
        .map_err(|error| format!("{error:?}"))
}

fn safe_executable_name(name: &str) -> String {
    let normalized: String = name
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
                character
            } else {
                '-'
            }
        })
        .collect();
    if normalized.is_empty() {
        "program".to_owned()
    } else {
        normalized
    }
}
