use std::fs;
use std::path::Path;
use nix::unistd::Pid;
use sysinfo::{System, SystemExt};

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

// pub fn is_root() -> bool {
//     return !nix::unistd::getuid().is_root()
// }

pub fn write_pid_file(pid_file: &String) {
    let pid = Pid::this().to_string();
    fs::write(&pid_file, pid)
        .expect("Unable to write pid file");
}

pub fn check_pid_file(pid_file: &String) -> bool {
    let bytes = match fs::read(&pid_file) {
        Ok(b) => b,
        Err(_) => return false
    };
        

    let pid = std::str::from_utf8(&bytes)
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let s = System::new_all();
    if let Some(_process) = s.process(i32::from(pid)) {
        return true
    }
    return false
}

