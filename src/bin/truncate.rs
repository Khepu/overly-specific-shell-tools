use std::fs::{read_dir, remove_file};

use clap::Parser;

use crate::config::truncate_config::TruncateConfig;
use crate::util::file_system::ensure_canonical_directory;

mod config;
mod util;

/// Deletes all files under the `directory` provided but preserves any directories.
fn truncate(directory: String, depth: u16) {
    let canonical_directory = ensure_canonical_directory(&directory);

    read_dir(&canonical_directory)
        .unwrap_or_else(|error| panic!("Could not list directory '{}'! {:?}", directory, error))
        .map(|dir_entry_result| {
            dir_entry_result
                .unwrap_or_else(|error| panic!("Unknown error! {:?}", error))
                .path()
        })
        .for_each(|path| {
            if path.is_file() {
                remove_file(&path).unwrap_or_else(|error| {
                    panic!(
                        "Could not delete file '{}'! {:?}",
                        path.file_name().unwrap().to_str().unwrap(),
                        error
                    )
                })
            } else if depth > 0 {
                let nested_directory = path
                    .to_str()
                    .unwrap_or_else(|| panic!("Could not truncate nested directory!"))
                    .to_string();

                truncate(nested_directory, depth - 1)
            }
        })
}

fn main() {
    let args: TruncateConfig = TruncateConfig::parse();

    truncate(args.directory, args.depth)
}
