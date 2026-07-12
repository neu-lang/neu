use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use object::{Object, ObjectSymbol};

use crate::bootstrap::{BootstrapOutcome, map_main_result};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LinkInvocationError {
    MissingObject,
    LinkerUnavailable,
    LinkerFailed(Option<i32>),
    MissingRuntimeSymbol(String),
    MissingOutput,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutableRunError {
    Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutableRunOutcome {
    Exited(i32),
    Signaled,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExecutableSmokeError {
    Unavailable,
    Signaled,
    UnexpectedExit { expected: u8, actual: i32 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemLinkInvocation {
    program: PathBuf,
    arguments: Vec<OsString>,
    entry_symbol: String,
    output: PathBuf,
}

impl SystemLinkInvocation {
    pub fn new(
        object: impl AsRef<Path>,
        output: impl AsRef<Path>,
    ) -> Result<Self, LinkInvocationError> {
        let object = object.as_ref();
        if !fs::metadata(object).is_ok_and(|metadata| metadata.is_file()) {
            return Err(LinkInvocationError::MissingObject);
        }
        let output = output.as_ref().to_owned();
        Ok(Self {
            program: std::env::var_os("NEU_LINKER")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("cc")),
            arguments: vec![
                OsString::from("-o"),
                output.as_os_str().to_owned(),
                object.as_os_str().to_owned(),
            ],
            entry_symbol: "main".to_owned(),
            output,
        })
    }

    pub fn program(&self) -> &Path {
        &self.program
    }

    pub fn arguments(&self) -> &[OsString] {
        &self.arguments
    }

    pub fn entry_symbol(&self) -> &str {
        &self.entry_symbol
    }

    pub fn execute(&self) -> Result<(), LinkInvocationError> {
        self.validate_runtime_symbols()?;
        let status = Command::new(&self.program)
            .args(&self.arguments)
            .status()
            .map_err(|_| LinkInvocationError::LinkerUnavailable)?;
        if status.success() {
            fs::metadata(&self.output)
                .is_ok_and(|metadata| metadata.is_file())
                .then_some(())
                .ok_or(LinkInvocationError::MissingOutput)
        } else {
            Err(LinkInvocationError::LinkerFailed(status.code()))
        }
    }

    fn validate_runtime_symbols(&self) -> Result<(), LinkInvocationError> {
        let object = self
            .arguments
            .last()
            .expect("link plan always contains the object path");
        let bytes = fs::read(object).map_err(|_| LinkInvocationError::MissingObject)?;
        let file = object::File::parse(bytes.as_slice())
            .map_err(|_| LinkInvocationError::LinkerFailed(None))?;
        if let Some(symbol) = file.symbols().find_map(|symbol| {
            let name = symbol.name().ok()?;
            let normalized = name.strip_prefix('_').unwrap_or(name);
            (symbol.is_undefined() && normalized.starts_with("neu_runtime_"))
                .then_some(normalized.to_owned())
        }) {
            return Err(LinkInvocationError::MissingRuntimeSymbol(symbol));
        }
        Ok(())
    }

    pub fn run(&self) -> Result<ExecutableRunOutcome, ExecutableRunError> {
        let status = Command::new(&self.output)
            .status()
            .map_err(|_| ExecutableRunError::Unavailable)?;
        Ok(status
            .code()
            .map_or(ExecutableRunOutcome::Signaled, ExecutableRunOutcome::Exited))
    }

    pub fn verify_main_result(
        &self,
        main_result: i64,
        failure_status: u8,
    ) -> Result<(), ExecutableSmokeError> {
        let expected = match map_main_result(main_result, failure_status) {
            BootstrapOutcome::Exit(status) | BootstrapOutcome::Failure { status, .. } => status,
        };
        match self.run() {
            Err(ExecutableRunError::Unavailable) => Err(ExecutableSmokeError::Unavailable),
            Ok(ExecutableRunOutcome::Signaled) => Err(ExecutableSmokeError::Signaled),
            Ok(ExecutableRunOutcome::Exited(actual)) if actual == i32::from(expected) => Ok(()),
            Ok(ExecutableRunOutcome::Exited(actual)) => {
                Err(ExecutableSmokeError::UnexpectedExit { expected, actual })
            }
        }
    }
}
