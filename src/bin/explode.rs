use std::fs::{read_dir, remove_dir, rename};

use clap::Parser;
use crate::config::explode_config::ExplodeConfig;

use crate::util::cli::ensure_canonical_directory;

mod util;
mod config;

/// Moves everything that is inside the `directory` provided
/// to its parent.
fn explode(directory: String) {
    let canonical_directory = ensure_canonical_directory(&directory);

    let parent = canonical_directory
        .parent()
        .unwrap_or_else(|| panic!(
            "Could not find parent of '{}'!",
            directory));

    read_dir(&canonical_directory)
        .unwrap_or_else(|error| panic!(
            "Could not list directory '{}'! {:?}",
            directory,
            error))
        .map(|dir_entry_result| dir_entry_result
            .unwrap_or_else(|error| panic!(
                "Unknown error! {:?}",
                error)))
        .for_each(|dir_entry| rename(
            dir_entry.path().as_path(),
            parent.join(dir_entry.file_name()))
            .unwrap_or_else(|error| panic!(
                "Failed to move '{}'! {:?}",
                dir_entry.path().to_str().unwrap(),
                error)));

    remove_dir(&canonical_directory)
        .unwrap_or_else(|error| panic!(
            "Could not delete '{}'! {:?}",
            directory,
            error))
}

fn main() {
    let args: ExplodeConfig = ExplodeConfig::parse();

    explode(args.directory)
}
