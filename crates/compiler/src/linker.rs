use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use crate::target_pack::TargetPack;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LinkInvocationError {
    MissingObject,
    LinkerUnavailable,
    LinkerFailed(Option<i32>),
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
pub struct LinkInvocation {
    program: PathBuf,
    arguments: Vec<OsString>,
    language_entry_symbol: String,
    output: PathBuf,
}

impl LinkInvocation {
    pub fn new(
        pack: &TargetPack,
        object: impl AsRef<Path>,
        output: impl AsRef<Path>,
    ) -> Result<Self, LinkInvocationError> {
        let object = object.as_ref();
        if !fs::metadata(object).is_ok_and(|metadata| metadata.is_file()) {
            return Err(LinkInvocationError::MissingObject);
        }

        let output = output.as_ref().to_owned();
        let arguments = vec![
            OsString::from("-o"),
            output.as_os_str().to_owned(),
            OsString::from("-e"),
            OsString::from(pack.entry_symbol()),
            pack.startup_shim_path().as_os_str().to_owned(),
            object.as_os_str().to_owned(),
        ];
        Ok(Self {
            program: pack.linker_path().to_owned(),
            arguments,
            language_entry_symbol: pack.language_entry_symbol().to_owned(),
            output,
        })
    }

    pub fn program(&self) -> &Path {
        &self.program
    }

    pub fn arguments(&self) -> &[OsString] {
        &self.arguments
    }

    pub fn language_entry_symbol(&self) -> &str {
        &self.language_entry_symbol
    }

    pub fn execute(&self) -> Result<(), LinkInvocationError> {
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

    pub fn run(&self) -> Result<ExecutableRunOutcome, ExecutableRunError> {
        let status = Command::new(&self.output)
            .status()
            .map_err(|_| ExecutableRunError::Unavailable)?;
        Ok(status
            .code()
            .map_or(ExecutableRunOutcome::Signaled, ExecutableRunOutcome::Exited))
    }
}
