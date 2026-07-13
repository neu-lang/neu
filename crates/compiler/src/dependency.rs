use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use serde::{Deserialize, Serialize};

use crate::manifest::{DependencyDescriptor, ManifestDiagnostic, ProjectManifest};
use crate::module::{VirtualDependency, VirtualSource};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DependencyDiagnosticKind {
    InvalidUrl,
    InvalidTag,
    CacheUnavailable,
    GitFailure,
    MissingManifest,
    InvalidManifest,
    DependencyCycle,
    DuplicateModule,
    ConflictingDependency,
    LockfileMalformed,
    LockfileMismatch,
    OfflineMiss,
    UnsupportedRepository,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyDiagnostic {
    kind: DependencyDiagnosticKind,
    detail: String,
}

impl DependencyDiagnostic {
    fn new(kind: DependencyDiagnosticKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> DependencyDiagnosticKind {
        self.kind
    }
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LockfileEntry {
    module: String,
    url: String,
    r#type: String,
    tag: String,
    commit: String,
}

impl LockfileEntry {
    pub fn new(
        module: impl Into<String>,
        url: impl Into<String>,
        tag: impl Into<String>,
        commit: impl Into<String>,
    ) -> Self {
        Self {
            module: module.into(),
            url: url.into(),
            r#type: "git".to_owned(),
            tag: tag.into(),
            commit: commit.into(),
        }
    }
    pub fn module(&self) -> &str {
        &self.module
    }
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn kind(&self) -> &str {
        &self.r#type
    }
    pub fn tag(&self) -> &str {
        &self.tag
    }
    pub fn commit(&self) -> &str {
        &self.commit
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Lockfile {
    version: u32,
    entries: Vec<LockfileEntry>,
}

impl Lockfile {
    pub fn entries(&self) -> &[LockfileEntry] {
        &self.entries
    }

    pub fn from_json(text: &str) -> Result<Self, DependencyDiagnostic> {
        serde_json::from_str(text).map_err(|error| {
            DependencyDiagnostic::new(
                DependencyDiagnosticKind::LockfileMalformed,
                error.to_string(),
            )
        })
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("lockfile serialization is infallible")
    }
}

#[derive(Clone, Debug)]
pub struct GitDependencyResolver {
    cache_root: PathBuf,
    offline: bool,
}

impl GitDependencyResolver {
    pub fn new(cache_root: impl Into<PathBuf>) -> Self {
        Self {
            cache_root: cache_root.into(),
            offline: false,
        }
    }

    pub fn from_environment() -> Result<Self, DependencyDiagnostic> {
        let root = env::var_os("NEU_PATH")
            .map(PathBuf::from)
            .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".neu")))
            .ok_or_else(|| {
                DependencyDiagnostic::new(
                    DependencyDiagnosticKind::CacheUnavailable,
                    "HOME and NEU_PATH are unset",
                )
            })?;
        Ok(Self::new(root))
    }

    pub fn offline(mut self, offline: bool) -> Self {
        self.offline = offline;
        self
    }

    pub fn cache_root(&self) -> &Path {
        &self.cache_root
    }

    pub fn lockfile(mut entries: Vec<LockfileEntry>) -> Lockfile {
        entries.sort_by(|left, right| left.module.cmp(&right.module));
        Lockfile {
            version: 1,
            entries,
        }
    }

    pub fn cache_path(&self, url: &str) -> Result<PathBuf, DependencyDiagnostic> {
        let trimmed = url
            .strip_prefix("https://")
            .ok_or_else(|| DependencyDiagnostic::new(DependencyDiagnosticKind::InvalidUrl, url))?;
        if trimmed.contains(['?', '#']) {
            return Err(DependencyDiagnostic::new(
                DependencyDiagnosticKind::InvalidUrl,
                url,
            ));
        }
        let mut pieces = trimmed.split('/');
        let host = pieces.next().unwrap_or_default();
        let parts: Vec<_> = pieces.collect();
        if host.is_empty()
            || parts.len() < 2
            || parts
                .iter()
                .any(|part| part.is_empty() || *part == "." || *part == "..")
        {
            return Err(DependencyDiagnostic::new(
                DependencyDiagnosticKind::InvalidUrl,
                url,
            ));
        }
        let mut path = self.cache_root.join("pkg").join(host);
        for part in parts {
            path.push(part.strip_suffix(".git").unwrap_or(part));
        }
        Ok(path)
    }

    pub fn resolve_project(
        &self,
        manifest_path: impl AsRef<Path>,
    ) -> Result<Lockfile, DependencyDiagnostic> {
        let manifest_path = manifest_path.as_ref();
        let (manifest, root) = ProjectManifest::load(manifest_path).map_err(manifest_error)?;
        let lock_path = root.join("neu.lock.json");
        let existing = if lock_path.is_file() {
            Some(Lockfile::from_json(
                &fs::read_to_string(&lock_path).map_err(|_| {
                    DependencyDiagnostic::new(
                        DependencyDiagnosticKind::LockfileMalformed,
                        lock_path.display().to_string(),
                    )
                })?,
            )?)
        } else {
            None
        };
        let mut entries = Vec::new();
        let mut by_module = BTreeMap::new();
        let mut active = BTreeSet::new();
        for dependency in manifest.dependencies() {
            self.resolve_dependency(
                dependency,
                existing.as_ref(),
                &mut entries,
                &mut by_module,
                &mut active,
            )?;
        }
        entries.sort_by(|left, right| left.module.cmp(&right.module));
        if let Some(lock) = &existing {
            if lock.entries.len() != entries.len()
                || lock
                    .entries
                    .iter()
                    .zip(&entries)
                    .any(|(old, new)| old != new)
            {
                return Err(DependencyDiagnostic::new(
                    DependencyDiagnosticKind::LockfileMismatch,
                    "neu.lock.json does not match resolved dependencies",
                ));
            }
        } else if !entries.is_empty() {
            write_lockfile(&lock_path, &Self::lockfile(entries.clone()))?;
        }
        Ok(Self::lockfile(entries))
    }

    pub fn load_project_dependencies(
        &self,
        manifest_path: impl AsRef<Path>,
    ) -> Result<Vec<VirtualDependency>, DependencyDiagnostic> {
        let manifest_path = manifest_path.as_ref();
        let lockfile = self.resolve_project(manifest_path)?;
        let mut dependencies = Vec::new();
        for entry in lockfile.entries {
            let repository = self.cache_path(&entry.url)?;
            let (manifest, root) =
                ProjectManifest::load(repository.join("neu.json")).map_err(manifest_error)?;
            let sources = manifest
                .load_sources(&root)
                .map_err(manifest_error)?
                .into_iter()
                .map(|source| VirtualSource::new(source.path(), source.source()))
                .collect::<Vec<_>>();
            dependencies.push(VirtualDependency::new(entry.url, sources));
        }
        Ok(dependencies)
    }

    fn resolve_dependency(
        &self,
        dependency: &DependencyDescriptor,
        existing: Option<&Lockfile>,
        entries: &mut Vec<LockfileEntry>,
        by_module: &mut BTreeMap<String, LockfileEntry>,
        active: &mut BTreeSet<String>,
    ) -> Result<(), DependencyDiagnostic> {
        let repository = self.checkout(dependency)?;
        let manifest_path = repository.join("neu.json");
        let (manifest, _) = ProjectManifest::load(&manifest_path).map_err(manifest_error)?;
        let module = manifest.name().to_owned();
        if !active.insert(module.clone()) {
            return Err(DependencyDiagnostic::new(
                DependencyDiagnosticKind::DependencyCycle,
                module,
            ));
        }
        let locked = existing.and_then(|lock| {
            lock.entries.iter().find(|entry| {
                entry.module == module
                    && entry.url == dependency.url()
                    && entry.tag == dependency.tag()
            })
        });
        let commit = if let Some(entry) = locked {
            git_output(
                &repository,
                ["cat-file", "-e", &format!("{}^{{commit}}", entry.commit)],
            )?;
            entry.commit.clone()
        } else {
            git_output(
                &repository,
                [
                    "rev-parse",
                    &format!("refs/tags/{}^{{commit}}", dependency.tag()),
                ],
            )?
        };
        let checkout_status = Command::new("git")
            .args([
                "-C",
                &repository.to_string_lossy(),
                "checkout",
                "--detach",
                &commit,
            ])
            .status()
            .map_err(|_| {
                DependencyDiagnostic::new(DependencyDiagnosticKind::GitFailure, &module)
            })?;
        if !checkout_status.success() {
            return Err(DependencyDiagnostic::new(
                DependencyDiagnosticKind::GitFailure,
                &module,
            ));
        }
        let entry = LockfileEntry::new(&module, dependency.url(), dependency.tag(), commit);
        if let Some(old) = by_module.insert(module.clone(), entry.clone())
            && old != entry
        {
            return Err(DependencyDiagnostic::new(
                DependencyDiagnosticKind::ConflictingDependency,
                module,
            ));
        }
        entries.retain(|entry| entry.module != module);
        entries.push(entry);
        for child in manifest.dependencies() {
            self.resolve_dependency(child, existing, entries, by_module, active)?;
        }
        active.remove(&module);
        Ok(())
    }

    fn checkout(&self, dependency: &DependencyDescriptor) -> Result<PathBuf, DependencyDiagnostic> {
        let destination = self.cache_path(dependency.url())?;
        if !destination.join(".git").is_dir() {
            if self.offline {
                return Err(DependencyDiagnostic::new(
                    DependencyDiagnosticKind::OfflineMiss,
                    destination.display().to_string(),
                ));
            }
            if let Some(parent) = destination.parent() {
                fs::create_dir_all(parent).map_err(|_| {
                    DependencyDiagnostic::new(
                        DependencyDiagnosticKind::CacheUnavailable,
                        parent.display().to_string(),
                    )
                })?;
            }
            let status = Command::new("git")
                .args([
                    "clone",
                    "--no-checkout",
                    dependency.url(),
                    &destination.to_string_lossy(),
                ])
                .status()
                .map_err(|_| {
                    DependencyDiagnostic::new(
                        DependencyDiagnosticKind::GitFailure,
                        dependency.url(),
                    )
                })?;
            if !status.success() {
                return Err(DependencyDiagnostic::new(
                    DependencyDiagnosticKind::GitFailure,
                    dependency.url(),
                ));
            }
        } else if !self.offline {
            let status = Command::new("git")
                .args([
                    "-C",
                    &destination.to_string_lossy(),
                    "fetch",
                    "--tags",
                    "--prune",
                    "origin",
                ])
                .status()
                .map_err(|_| {
                    DependencyDiagnostic::new(
                        DependencyDiagnosticKind::GitFailure,
                        dependency.url(),
                    )
                })?;
            if !status.success() {
                return Err(DependencyDiagnostic::new(
                    DependencyDiagnosticKind::GitFailure,
                    dependency.url(),
                ));
            }
        }
        if destination.join(".gitmodules").exists() {
            return Err(DependencyDiagnostic::new(
                DependencyDiagnosticKind::UnsupportedRepository,
                ".gitmodules is not supported",
            ));
        }
        Ok(destination)
    }
}

fn git_output<const N: usize>(
    directory: &Path,
    args: [&str; N],
) -> Result<String, DependencyDiagnostic> {
    let output = Command::new("git")
        .arg("-C")
        .arg(directory)
        .args(args)
        .output()
        .map_err(|_| {
            DependencyDiagnostic::new(
                DependencyDiagnosticKind::GitFailure,
                directory.display().to_string(),
            )
        })?;
    if !output.status.success() {
        return Err(DependencyDiagnostic::new(
            DependencyDiagnosticKind::GitFailure,
            String::from_utf8_lossy(&output.stderr),
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn write_lockfile(path: &Path, lockfile: &Lockfile) -> Result<(), DependencyDiagnostic> {
    let temporary = path.with_extension(format!("json.tmp-{}", std::process::id()));
    fs::write(&temporary, lockfile.to_json()).map_err(|_| {
        DependencyDiagnostic::new(
            DependencyDiagnosticKind::LockfileMalformed,
            temporary.display().to_string(),
        )
    })?;
    fs::rename(&temporary, path).map_err(|_| {
        DependencyDiagnostic::new(
            DependencyDiagnosticKind::LockfileMalformed,
            path.display().to_string(),
        )
    })
}

fn manifest_error(error: ManifestDiagnostic) -> DependencyDiagnostic {
    DependencyDiagnostic::new(DependencyDiagnosticKind::InvalidManifest, error.detail())
}
