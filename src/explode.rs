use std::fs::{canonicalize, metadata, read_dir, remove_dir, rename};

pub fn explode(directory: String) {
    let canonical_directory = canonicalize(&directory)
        .unwrap_or_else(|error| panic!(
            "Could not canonicalize path '{}'! {:?}",
            directory,
            error));

    if !canonical_directory.is_dir() {
        panic!(
            "'{}' is not a directory!",
            directory)
    }

    let parent = canonical_directory
        .parent()
        .unwrap_or_else(|| panic!(
            "Could not find parent of '{}'!",
            directory));

    read_dir(&directory)
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

    remove_dir(&directory)
        .unwrap_or_else(|error| panic!(
            "Could not delete '{}'! {:?}",
            directory,
            error))
}
