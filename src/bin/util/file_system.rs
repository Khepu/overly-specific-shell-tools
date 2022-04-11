use std::fs::canonicalize;
use std::path::PathBuf;

pub fn ensure_canonical_directory(path: &String) -> PathBuf {
    let canonical_path = canonicalize(path)
        .unwrap_or_else(|error| panic!(
            "Could not canonicalize path '{}'! {:?}",
            *path,
            error));

    if canonical_path.is_dir() {
        canonical_path
    } else {
        panic!(
            "'{}' is not a directory!",
            *path)
    }
}
