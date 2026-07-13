use std::{
    collections::BTreeSet,
    fs,
    path::{Component, Path, PathBuf},
};

use serde::Deserialize;

use crate::{module::ModuleName, module::VirtualSource};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ManifestDiagnosticKind {
    Io,
    MalformedJson,
    DuplicateField,
    UnknownField,
    MissingField,
    InvalidFieldType,
    InvalidName,
    InvalidPath,
    AbsolutePath,
    PathTraversal,
    InvalidGlob,
    EmptyGlob,
    EntrypointNotSelected,
    EntrypointRequired,
    SymlinkEscape,
    InvalidDependency,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManifestDiagnostic {
    kind: ManifestDiagnosticKind,
    detail: String,
}

impl ManifestDiagnostic {
    fn new(kind: ManifestDiagnosticKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> ManifestDiagnosticKind {
        self.kind
    }
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawManifest {
    name: String,
    #[serde(default)]
    description: Option<String>,
    entrypoint: Option<String>,
    srcs: Vec<String>,
    #[serde(default)]
    dependencies: Vec<RawDependency>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawDependency {
    url: String,
    #[serde(default)]
    r#type: Option<String>,
    tag: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyDescriptor {
    url: String,
    kind: String,
    tag: String,
}

impl DependencyDescriptor {
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn kind(&self) -> &str {
        &self.kind
    }
    pub fn tag(&self) -> &str {
        &self.tag
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectManifest {
    name: String,
    description: Option<String>,
    entrypoint: Option<PathBuf>,
    srcs: Vec<String>,
    dependencies: Vec<DependencyDescriptor>,
}

impl ProjectManifest {
    pub fn parse(text: &str) -> Result<Self, ManifestDiagnostic> {
        let raw: RawManifest = serde_json::from_str(text).map_err(|error| {
            let message = error.to_string();
            let kind = if message.contains("unknown field") {
                ManifestDiagnosticKind::UnknownField
            } else if message.contains("duplicate field") {
                ManifestDiagnosticKind::DuplicateField
            } else if message.contains("missing field") {
                ManifestDiagnosticKind::MissingField
            } else if message.contains("invalid type") {
                ManifestDiagnosticKind::InvalidFieldType
            } else {
                ManifestDiagnosticKind::MalformedJson
            };
            ManifestDiagnostic::new(kind, message)
        })?;
        ModuleName::parse(&raw.name).map_err(|_| {
            ManifestDiagnostic::new(ManifestDiagnosticKind::InvalidName, raw.name.clone())
        })?;
        let entrypoint = raw
            .entrypoint
            .map(|entrypoint| {
                let entrypoint = validate_relative_path(Path::new(&entrypoint))?;
                if entrypoint
                    .extension()
                    .and_then(|extension| extension.to_str())
                    != Some("neu")
                {
                    return Err(ManifestDiagnostic::new(
                        ManifestDiagnosticKind::InvalidPath,
                        "entrypoint must have the .neu extension",
                    ));
                }
                Ok(entrypoint)
            })
            .transpose()?;
        if raw.srcs.is_empty() {
            return Err(ManifestDiagnostic::new(
                ManifestDiagnosticKind::MissingField,
                "srcs must contain at least one pattern",
            ));
        }
        for pattern in &raw.srcs {
            validate_pattern(pattern)?;
        }
        let dependencies = raw
            .dependencies
            .into_iter()
            .map(validate_dependency)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            name: raw.name,
            description: raw.description,
            entrypoint,
            srcs: raw.srcs,
            dependencies,
        })
    }

    pub fn load(path: impl AsRef<Path>) -> Result<(Self, PathBuf), ManifestDiagnostic> {
        let path = path.as_ref().to_owned();
        let text = fs::read_to_string(&path).map_err(|_| {
            ManifestDiagnostic::new(ManifestDiagnosticKind::Io, path.display().to_string())
        })?;
        let manifest = Self::parse(&text)?;
        let root = path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();
        Ok((manifest, root))
    }

    pub fn discover(start: impl AsRef<Path>) -> Result<PathBuf, ManifestDiagnostic> {
        let start = start.as_ref();
        let mut directory = if start.is_file() {
            start
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf()
        } else {
            start.to_path_buf()
        };
        loop {
            let candidate = directory.join("neu.json");
            if candidate.is_file() {
                return Ok(candidate);
            }
            if !directory.pop() {
                return Err(ManifestDiagnostic::new(
                    ManifestDiagnosticKind::Io,
                    "neu.json was not found",
                ));
            }
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub fn is_library(&self) -> bool {
        self.entrypoint.is_none()
    }
    pub fn entrypoint(&self) -> Option<&Path> {
        self.entrypoint.as_deref()
    }
    pub fn require_entrypoint(&self) -> Result<&Path, ManifestDiagnostic> {
        self.entrypoint.as_deref().ok_or_else(|| {
            ManifestDiagnostic::new(
                ManifestDiagnosticKind::EntrypointRequired,
                "an executable entrypoint is required for this operation",
            )
        })
    }
    pub fn srcs(&self) -> &[String] {
        &self.srcs
    }
    pub fn dependencies(&self) -> &[DependencyDescriptor] {
        &self.dependencies
    }

    pub fn expand_sources(
        &self,
        root: impl AsRef<Path>,
    ) -> Result<Vec<PathBuf>, ManifestDiagnostic> {
        let root = root.as_ref();
        let canonical_root = fs::canonicalize(root).map_err(|_| {
            ManifestDiagnostic::new(ManifestDiagnosticKind::Io, root.display().to_string())
        })?;
        let mut candidates = Vec::new();
        collect_files(root, &canonical_root, Path::new(""), &mut candidates)?;
        let mut selected = BTreeSet::new();
        for pattern in &self.srcs {
            let mut matched = false;
            for (relative, absolute) in &candidates {
                if pattern_matches(pattern, relative) {
                    matched = true;
                    selected.insert(absolute.clone());
                }
            }
            if !matched {
                return Err(ManifestDiagnostic::new(
                    ManifestDiagnosticKind::EmptyGlob,
                    pattern,
                ));
            }
        }
        if let Some(entrypoint) = &self.entrypoint {
            let entry = root.join(entrypoint);
            if !selected.contains(&entry) {
                return Err(ManifestDiagnostic::new(
                    ManifestDiagnosticKind::EntrypointNotSelected,
                    entrypoint.display().to_string(),
                ));
            }
        }
        Ok(selected.into_iter().collect())
    }

    pub fn load_sources(
        &self,
        root: impl AsRef<Path>,
    ) -> Result<Vec<VirtualSource>, ManifestDiagnostic> {
        let root = root.as_ref();
        self.expand_sources(root)?
            .into_iter()
            .map(|path| {
                let relative = path.strip_prefix(root).map_err(|_| {
                    ManifestDiagnostic::new(
                        ManifestDiagnosticKind::InvalidPath,
                        path.display().to_string(),
                    )
                })?;
                let source = fs::read_to_string(&path).map_err(|_| {
                    ManifestDiagnostic::new(ManifestDiagnosticKind::Io, path.display().to_string())
                })?;
                Ok(VirtualSource::new(relative, source))
            })
            .collect()
    }
}

fn validate_dependency(raw: RawDependency) -> Result<DependencyDescriptor, ManifestDiagnostic> {
    let kind = raw.r#type.unwrap_or_else(|| "git".to_owned());
    if kind != "git" || !raw.url.starts_with("https://") || raw.tag.is_empty() {
        return Err(ManifestDiagnostic::new(
            ManifestDiagnosticKind::InvalidDependency,
            raw.url,
        ));
    }
    Ok(DependencyDescriptor {
        url: raw.url,
        kind,
        tag: raw.tag,
    })
}

fn validate_relative_path(path: &Path) -> Result<PathBuf, ManifestDiagnostic> {
    if path.is_absolute() {
        return Err(ManifestDiagnostic::new(
            ManifestDiagnosticKind::AbsolutePath,
            path.display().to_string(),
        ));
    }
    let mut result = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(part) => result.push(part),
            Component::ParentDir => {
                return Err(ManifestDiagnostic::new(
                    ManifestDiagnosticKind::PathTraversal,
                    path.display().to_string(),
                ));
            }
            Component::RootDir | Component::Prefix(_) => {
                return Err(ManifestDiagnostic::new(
                    ManifestDiagnosticKind::AbsolutePath,
                    path.display().to_string(),
                ));
            }
        }
    }
    Ok(result)
}

fn validate_pattern(pattern: &str) -> Result<(), ManifestDiagnostic> {
    validate_relative_path(Path::new(pattern))?;
    if pattern.is_empty() || pattern.split('/').any(|part| part.is_empty()) {
        return Err(ManifestDiagnostic::new(
            ManifestDiagnosticKind::InvalidGlob,
            pattern,
        ));
    }
    if !pattern.ends_with(".neu") {
        return Err(ManifestDiagnostic::new(
            ManifestDiagnosticKind::InvalidGlob,
            pattern,
        ));
    }
    Ok(())
}

fn collect_files(
    directory: &Path,
    canonical_root: &Path,
    relative: &Path,
    output: &mut Vec<(PathBuf, PathBuf)>,
) -> Result<(), ManifestDiagnostic> {
    let mut entries = fs::read_dir(directory)
        .map_err(|_| {
            ManifestDiagnostic::new(ManifestDiagnosticKind::Io, directory.display().to_string())
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| {
            ManifestDiagnostic::new(ManifestDiagnosticKind::Io, directory.display().to_string())
        })?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let name = entry.file_name();
        if name.to_string_lossy().starts_with('.') {
            continue;
        }
        let path = entry.path();
        let relative_path = relative.join(&name);
        let metadata = fs::symlink_metadata(&path).map_err(|_| {
            ManifestDiagnostic::new(ManifestDiagnosticKind::Io, path.display().to_string())
        })?;
        let canonical = fs::canonicalize(&path).map_err(|_| {
            ManifestDiagnostic::new(ManifestDiagnosticKind::Io, path.display().to_string())
        })?;
        if !canonical.starts_with(canonical_root) {
            return Err(ManifestDiagnostic::new(
                ManifestDiagnosticKind::SymlinkEscape,
                path.display().to_string(),
            ));
        }
        if metadata.is_dir() {
            collect_files(&path, canonical_root, &relative_path, output)?;
        } else if metadata.is_file()
            && path.extension().and_then(|extension| extension.to_str()) == Some("neu")
        {
            output.push((relative_path, path));
        }
    }
    Ok(())
}

fn pattern_matches(pattern: &str, path: &Path) -> bool {
    let patterns: Vec<_> = pattern.split('/').collect();
    let parts: Vec<_> = path.iter().filter_map(|part| part.to_str()).collect();
    match_segments(&patterns, &parts)
}

fn match_segments(pattern: &[&str], parts: &[&str]) -> bool {
    match (pattern.first(), parts.first()) {
        (None, None) => true,
        (Some(&"**"), _) => {
            match_segments(&pattern[1..], parts)
                || (!parts.is_empty() && match_segments(pattern, &parts[1..]))
        }
        (Some(segment), Some(part)) => {
            segment_matches(segment, part) && match_segments(&pattern[1..], &parts[1..])
        }
        _ => false,
    }
}

fn segment_matches(pattern: &str, value: &str) -> bool {
    let mut pattern = pattern.chars().peekable();
    let mut value = value.chars().peekable();
    while let Some(character) = pattern.next() {
        match character {
            '*' => {
                if pattern.peek().is_none() {
                    return true;
                }
                while value.next().is_some() {
                    if segment_matches(
                        pattern.clone().collect::<String>().as_str(),
                        value.clone().collect::<String>().as_str(),
                    ) {
                        return true;
                    }
                }
                return false;
            }
            '?' => {
                if value.next().is_none() {
                    return false;
                }
            }
            literal => {
                if value.next() != Some(literal) {
                    return false;
                }
            }
        }
    }
    value.next().is_none()
}
