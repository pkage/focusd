use std::fs;
use std::path::Path;

pub fn expand_path(path: &String) -> String {
    return shellexpand::tilde(path).to_string();
}

pub fn file_exists(path: &String) -> bool {
    let epath = expand_path(&path);
    return Path::new(&epath).exists();
}

pub fn file_remove_if_exists(path: &String) {
    let epath = expand_path(path);
    if file_exists(&epath) {
        fs::remove_file(&epath).unwrap();
    }
}

pub fn file_write(path: &String, contents: &String) {
    let epath = expand_path(path);
    fs::write(epath, contents)
        .expect("Unable to write to file!");
}

pub fn is_root() -> bool {
    return !nix::unistd::getuid().is_root()
}


