use std::{fs, path::PathBuf, process::ExitCode};

use clap::{Args, Parser, Subcommand};
use compiler::driver::{
    SourceDriverOptions, compile_manifest_to_executable_for_target,
    compile_source_to_test_executable, discover_manifest_tests,
};
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
    #[command(about = "Run Neu test functions")]
    Test(TestArgs),
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

#[derive(Debug, Args)]
struct TestArgs {
    #[arg(value_name = "PROJECT_OR_MANIFEST")]
    project: Option<PathBuf>,
    #[arg(long)]
    list: bool,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("neu: {error}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<(), String> {
    let Cli { command } = Cli::parse();
    match command {
        Command::Build(args) => {
            println!("{}", build(args)?.display());
            Ok(())
        }
        Command::Test(args) => test(args),
    }
}

fn test(args: TestArgs) -> Result<(), String> {
    let manifest_path = resolve_manifest_path(args.project)?;
    let project = discover_manifest_tests(&manifest_path).map_err(|error| format!("{error:?}"))?;
    if args.list {
        for test in project.tests() {
            println!("{}", test.symbol());
        }
        return Ok(());
    }
    let root = manifest_path
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."));
    let output_root = root
        .join("target")
        .join(format!("neu-tests-{}", std::process::id()));
    fs::create_dir_all(&output_root).map_err(|error| error.to_string())?;
    let mut failures = 0usize;
    for (index, test) in project.tests().iter().enumerate() {
        let output = output_root.join(format!("test-{index}"));
        let executable = compile_source_to_test_executable(
            project.source(),
            SourceDriverOptions::new(
                project.source_file(),
                project.module().clone(),
                project.package().clone(),
                &output,
            ),
            test.declaration(),
        )
        .map_err(|error| format!("{error:?}"))?;
        let result = std::process::Command::new(executable)
            .output()
            .map_err(|error| error.to_string())?;
        if !result.status.success() {
            failures += 1;
            let detail = String::from_utf8_lossy(&result.stderr).trim().to_owned();
            let detail = if detail.is_empty() {
                "native runtime trap".to_owned()
            } else {
                detail
            };
            eprintln!("test {}: failed: {}", test.symbol(), detail);
        }
    }
    let _ = fs::remove_dir_all(&output_root);
    if failures != 0 {
        return Err(format!("{failures} test(s) failed"));
    }
    Ok(())
}

fn resolve_manifest_path(project: Option<PathBuf>) -> Result<PathBuf, String> {
    match project {
        Some(path) if path.is_file() => Ok(path),
        Some(path) if path.is_dir() => compiler::manifest::ProjectManifest::discover(path)
            .map_err(|error| error.detail().to_owned()),
        Some(path) => Err(format!("project path does not exist: {}", path.display())),
        None => compiler::manifest::ProjectManifest::discover(
            std::env::current_dir().map_err(|error| error.to_string())?,
        )
        .map_err(|error| error.detail().to_owned()),
    }
}

fn build(args: BuildArgs) -> Result<PathBuf, String> {
    let manifest_path = resolve_manifest_path(args.project)?;
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
        .map(|value| value.parse::<Triple>().map_err(|error| error.to_string()))
        .transpose()?
        .unwrap_or_else(Triple::host);
    compile_manifest_to_executable_for_target(&manifest_path, target, &output)
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
