use compiler::dependency::{GitDependencyResolver, LockfileEntry};

#[test]
fn lockfile_entries_are_deterministically_ordered() {
    let entries = vec![
        LockfileEntry::new("z.app", "https://github.com/example/z.git", "v1", "abc"),
        LockfileEntry::new("a.app", "https://github.com/example/a.git", "v1", "def"),
    ];
    let lockfile = GitDependencyResolver::lockfile(entries);
    assert_eq!(lockfile.entries()[0].module(), "a.app");
    assert_eq!(lockfile.entries()[1].module(), "z.app");
}

#[test]
fn git_cache_layout_is_url_based_but_module_identity_is_not() {
    let resolver = GitDependencyResolver::new("/tmp/neu-cache");
    assert_eq!(
        resolver
            .cache_path("https://github.com/example/shared.git")
            .unwrap(),
        std::path::Path::new("/tmp/neu-cache/pkg/github.com/example/shared")
    );
    assert!(
        resolver
            .cache_path("ssh://github.com/example/shared.git")
            .is_err()
    );
}

#[test]
fn lockfile_round_trips_with_stable_schema() {
    let lockfile = GitDependencyResolver::lockfile(vec![LockfileEntry::new(
        "example.shared",
        "https://github.com/example/shared.git",
        "v1.0.0",
        "0123456789abcdef",
    )]);
    let parsed = compiler::dependency::Lockfile::from_json(&lockfile.to_json()).unwrap();
    assert_eq!(parsed.entries(), lockfile.entries());
}
