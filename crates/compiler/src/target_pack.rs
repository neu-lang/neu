use std::{
    fs,
    path::{Component, Path, PathBuf},
};

use object::{Object, ObjectSection, ObjectSymbol};
use serde::Deserialize;
use target_lexicon::Triple;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SerializedTargetPackManifest {
    target: SerializedTarget,
    linker: SerializedArtifact,
    startup_shim: SerializedArtifact,
    entry: SerializedEntry,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SerializedTarget {
    triple: String,
    object_format: String,
    executable_format: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SerializedArtifact {
    path: PathBuf,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SerializedEntry {
    platform_symbol: String,
    language_symbol: String,
    trap_exit_code: u8,
}

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
    InvalidStartupShim,
    StartupShimFormatMismatch,
    MissingStartupEntrySymbol,
    MissingLanguageEntryRelocation,
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
    pub fn from_toml(input: &str) -> Result<Self, TargetPackError> {
        let serialized = toml::from_str::<SerializedTargetPackManifest>(input)
            .map_err(|_| TargetPackError::InvalidManifest)?;
        let target = serialized
            .target
            .triple
            .parse::<Triple>()
            .map_err(|_| TargetPackError::InvalidManifest)?;
        Self::new(
            target,
            serialized.target.object_format,
            serialized.target.executable_format,
            serialized.linker.path,
            serialized.startup_shim.path,
            serialized.entry.platform_symbol,
            serialized.entry.language_symbol,
            serialized.entry.trap_exit_code,
        )
    }

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
            || binary_format(&self.object_format).is_none()
            || binary_format(&self.executable_format).is_none()
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
    pub fn resolve_toml(
        root: impl AsRef<Path>,
        input: &str,
        requested_target: Triple,
    ) -> Result<Self, TargetPackError> {
        Self::resolve(
            root,
            TargetPackManifest::from_toml(input)?,
            requested_target,
        )
    }

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
        validate_startup_shim(
            &startup_shim_path,
            &manifest.object_format,
            &manifest.entry_symbol,
            &manifest.language_entry_symbol,
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

fn binary_format(name: &str) -> Option<object::BinaryFormat> {
    match name.to_ascii_lowercase().as_str() {
        "coff" => Some(object::BinaryFormat::Coff),
        "elf" => Some(object::BinaryFormat::Elf),
        "macho" | "mach-o" => Some(object::BinaryFormat::MachO),
        "pe" => Some(object::BinaryFormat::Pe),
        "wasm" => Some(object::BinaryFormat::Wasm),
        "xcoff" => Some(object::BinaryFormat::Xcoff),
        _ => None,
    }
}

fn validate_startup_shim(
    path: &Path,
    object_format: &str,
    entry_symbol: &str,
    language_entry_symbol: &str,
) -> Result<(), TargetPackError> {
    let bytes = fs::read(path).map_err(|_| TargetPackError::InvalidStartupShim)?;
    let object =
        object::File::parse(bytes.as_slice()).map_err(|_| TargetPackError::InvalidStartupShim)?;
    let expected_format = binary_format(object_format).ok_or(TargetPackError::InvalidManifest)?;
    if object.format() != expected_format {
        return Err(TargetPackError::StartupShimFormatMismatch);
    }
    if !object.symbols().any(|symbol| {
        symbol
            .name()
            .is_ok_and(|name| symbol_name_matches(object.format(), name, entry_symbol))
    }) {
        return Err(TargetPackError::MissingStartupEntrySymbol);
    }
    let has_language_entry_relocation = object.sections().any(|section| {
        section.relocations().any(|(_, relocation)| {
            matches!(relocation.target(), object::RelocationTarget::Symbol(index)
            if object.symbol_by_index(index).is_ok_and(|symbol| {
                    symbol
                        .name()
                        .is_ok_and(|name| {
                            symbol_name_matches(object.format(), name, language_entry_symbol)
                        })
                }))
        })
    });
    if !has_language_entry_relocation {
        return Err(TargetPackError::MissingLanguageEntryRelocation);
    }
    Ok(())
}

fn symbol_name_matches(format: object::BinaryFormat, actual: &str, expected: &str) -> bool {
    actual == expected
        || (format == object::BinaryFormat::MachO && actual == format!("_{expected}"))
}
