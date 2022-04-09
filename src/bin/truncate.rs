use std::env::args;
use std::fs::{read_dir, remove_file};
use crate::util::cli::ensure_canonical_directory;

mod util;

/// Deletes all files under the `directory` provided but preserves any directories.
fn truncate(directory: String) {
    let canonical_directory = ensure_canonical_directory(&directory);

    read_dir(&canonical_directory)
        .unwrap_or_else(|error| panic!(
            "Could not list directory '{}'! {:?}",
            directory,
            error))
        .map(|dir_entry_result| dir_entry_result
            .unwrap_or_else(|error| panic!(
                "Unknown error! {:?}",
                error))
            .path())
        .for_each(|path|
            if path.is_file() {
                remove_file(path)
                    .unwrap_or_else(|error| panic!(
                        "Could not delete file '{}'",
                        path.file_name().to_str().unwrap()))
            });
}

fn main() {
    let directory = args()
        .nth(1)
        .expect("No path provided!");
    explode(directory)
}
