use std::fs;
use std::path::Path;
use nix::unistd::getuid;

pub fn file_exists(path: &String) -> bool {
    return Path::new(&path).exists();
}

pub fn file_remove_if_exists(path: &String) {
    if file_exists(&path) {
        fs::remove_file(&path).unwrap();
    }
}

pub fn is_root() -> bool {
    return !nix::unistd::getuid().is_root()
}

pub fn time_to_seconds(time: String) -> u64 {
    return 100;
}