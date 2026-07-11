use std::{
    fs,
    path::{Component, Path, PathBuf},
};

use target_lexicon::Triple;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactKind {
    Linker,
    StartupShim,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TargetPackError {
    InvalidManifest,
    TargetMismatch,
    MissingPackRoot,
    AbsoluteArtifactPath(ArtifactKind),
    TraversalArtifactPath(ArtifactKind),
    MissingArtifact(ArtifactKind),
    ArtifactOutsidePack(ArtifactKind),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetPackManifest {
    target: Triple,
    object_format: String,
    executable_format: String,
    linker_path: PathBuf,
    startup_shim_path: PathBuf,
    entry_symbol: String,
    language_entry_symbol: String,
    trap_exit_code: u8,
}

impl TargetPackManifest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        target: Triple,
        object_format: impl Into<String>,
        executable_format: impl Into<String>,
        linker_path: impl Into<PathBuf>,
        startup_shim_path: impl Into<PathBuf>,
        entry_symbol: impl Into<String>,
        language_entry_symbol: impl Into<String>,
        trap_exit_code: u8,
    ) -> Result<Self, TargetPackError> {
        let manifest = Self {
            target,
            object_format: object_format.into(),
            executable_format: executable_format.into(),
            linker_path: linker_path.into(),
            startup_shim_path: startup_shim_path.into(),
            entry_symbol: entry_symbol.into(),
            language_entry_symbol: language_entry_symbol.into(),
            trap_exit_code,
        };
        manifest.validate()?;
        Ok(manifest)
    }

    fn validate(&self) -> Result<(), TargetPackError> {
        if self.object_format.is_empty()
            || self.executable_format.is_empty()
            || self.entry_symbol.is_empty()
            || self.language_entry_symbol.is_empty()
            || self.trap_exit_code == 0
        {
            return Err(TargetPackError::InvalidManifest);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetPack {
    target: Triple,
    object_format: String,
    executable_format: String,
    linker_path: PathBuf,
    startup_shim_path: PathBuf,
    entry_symbol: String,
    language_entry_symbol: String,
    trap_exit_code: u8,
}

impl TargetPack {
    pub fn resolve(
        root: impl AsRef<Path>,
        manifest: TargetPackManifest,
        requested_target: Triple,
    ) -> Result<Self, TargetPackError> {
        manifest.validate()?;
        if manifest.target != requested_target {
            return Err(TargetPackError::TargetMismatch);
        }

        let root = fs::canonicalize(root.as_ref()).map_err(|_| TargetPackError::MissingPackRoot)?;
        if !root.is_dir() {
            return Err(TargetPackError::MissingPackRoot);
        }
        let linker_path = resolve_artifact(&root, &manifest.linker_path, ArtifactKind::Linker)?;
        let startup_shim_path = resolve_artifact(
            &root,
            &manifest.startup_shim_path,
            ArtifactKind::StartupShim,
        )?;

        Ok(Self {
            target: manifest.target,
            object_format: manifest.object_format,
            executable_format: manifest.executable_format,
            linker_path,
            startup_shim_path,
            entry_symbol: manifest.entry_symbol,
            language_entry_symbol: manifest.language_entry_symbol,
            trap_exit_code: manifest.trap_exit_code,
        })
    }

    pub fn target(&self) -> Triple {
        self.target.clone()
    }

    pub fn object_format(&self) -> &str {
        &self.object_format
    }

    pub fn executable_format(&self) -> &str {
        &self.executable_format
    }

    pub fn linker_path(&self) -> &Path {
        &self.linker_path
    }

    pub fn startup_shim_path(&self) -> &Path {
        &self.startup_shim_path
    }

    pub fn entry_symbol(&self) -> &str {
        &self.entry_symbol
    }

    pub fn language_entry_symbol(&self) -> &str {
        &self.language_entry_symbol
    }

    pub fn trap_exit_code(&self) -> u8 {
        self.trap_exit_code
    }
}

fn resolve_artifact(
    root: &Path,
    relative: &Path,
    kind: ArtifactKind,
) -> Result<PathBuf, TargetPackError> {
    if relative.is_absolute() {
        return Err(TargetPackError::AbsoluteArtifactPath(kind));
    }
    if relative.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return Err(TargetPackError::TraversalArtifactPath(kind));
    }

    let path = root.join(relative);
    let canonical = fs::canonicalize(&path).map_err(|_| TargetPackError::MissingArtifact(kind))?;
    if !canonical.starts_with(root) {
        return Err(TargetPackError::ArtifactOutsidePack(kind));
    }
    if !canonical.is_file() {
        return Err(TargetPackError::MissingArtifact(kind));
    }
    Ok(canonical)
}
