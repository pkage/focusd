use super::hosts;
use super::time::*;
use super::config::*;
use super::common::*;
use super::messages::*;

use std::path::Path;
use nix::sys::stat;
use std::fs;

pub enum FocusServerError {
    AlreadyRunning,
    // NoPermissions
}

pub struct FocusServer<'a> {
    socket_file_in: String,
    socket_file_out: String,
    config: &'a FocusConfig,
    expires: Option<u64>
}

impl FocusServer<'_> {
    pub fn new(config: &FocusConfig) -> Result<FocusServer, FocusServerError> {

        let socket_file = &config.socket_file;
        
        if file_exists(&socket_file) {
            return Err(FocusServerError::AlreadyRunning);
        }

        // if !is_root() {
        //     return Err(FocusServerError::NoPermissions);
        // }

        let socket_file_in  = format!("{}.in", socket_file);
        let socket_file_out = format!("{}.out", socket_file);

        nix::unistd::mkfifo(Path::new(&socket_file_in),  stat::Mode::S_IRWXU).unwrap();
        nix::unistd::mkfifo(Path::new(&socket_file_out), stat::Mode::S_IRWXU).unwrap();

        write_pid_file(&config.pid_file);

        return Ok(
            FocusServer {
                socket_file_in,
                socket_file_out,
                config,
                expires: Option::None
            }
        )
    }

    pub fn cleanup(config: &FocusConfig) {
        file_remove_if_exists(&format!("{}.in", config.socket_file));
        file_remove_if_exists(&format!("{}.out", config.socket_file));
        file_remove_if_exists(&config.pid_file);
    }

    fn start(&mut self, time: String) {
        let secs = parse_time_string(&time).unwrap();
        println!("[timestart] starting with {}", secs);

        self.expires = Some(get_time() + secs);

        let computed_hosts = hosts::hosts_add(&self.config.hosts_file, &self.config.blocked)
                .expect("Unable to parse hosts file");

        file_write(&self.config.hosts_file, &computed_hosts);
    }

    fn get_remaining_time(&self) -> u64 {
        match self.expires {
            Some(secs) => if secs > get_time() {
                secs - get_time()
            } else {
                0
            },
            None       => 0
        }
    }

    fn check_remaining(&mut self) -> ServerRunStatus {
        let remaining = self.get_remaining_time();
        
        if remaining == 0 {
            if hosts::hosts_active(&self.config.hosts_file).unwrap_or(false) {
                println!("[timeend] cleaning up hosts");
                let computed_hosts = hosts::hosts_remove(&self.config.hosts_file)
                    .expect("Unable to parse hosts file!");

                file_write(&self.config.hosts_file, &computed_hosts);
            }

            self.expires = None;

            return ServerRunStatus::NotRunning;
        }

        return ServerRunStatus::Running(remaining);
    }

    pub fn listen(&mut self) {
        loop {
            let request = fs::read(&self.socket_file_in)
                .expect("Unable to read from infile!");

            let mut should_halt = false;
            let response = match client_unpack(request) {
                ClientRequest::Ping => ServerResponse::Pong,
                ClientRequest::Halt => {
                    should_halt = true;
                    ServerResponse::Halting
                },
                ClientRequest::Start(time) => {
                    self.start(time);
                    ServerResponse::Starting
                },
                ClientRequest::Remaining => ServerResponse::Remaining(self.check_remaining())
            };
    
            fs::write(&self.socket_file_out, server_pack(response))
                .expect("Unable to write to outfile!");

            if should_halt {
                return
            }
        }
    }
}
