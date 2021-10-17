use super::time::*;
use super::common::*;
use super::messages::*;

use std::path::Path;
use nix::sys::stat;
use std::fs;

pub enum FocusServerError {
    AlreadyRunning,
    NoPermissions
}

pub struct FocusServer {
    socket_file_in: String,
    socket_file_out: String,
    expires: Option<u64>
}

impl FocusServer {
    pub fn new(socket_file: String) -> Result<FocusServer, FocusServerError> {
        
        if file_exists(&socket_file) {
            return Err(FocusServerError::AlreadyRunning);
        }

        if !is_root() {
            return Err(FocusServerError::NoPermissions);
        }

        let socket_file_in  = format!("{}.in", socket_file);
        let socket_file_out = format!("{}.out", socket_file);

        nix::unistd::mkfifo(Path::new(&socket_file_in),  stat::Mode::S_IRWXU).unwrap();
        nix::unistd::mkfifo(Path::new(&socket_file_out), stat::Mode::S_IRWXU).unwrap();

        return Ok(
            FocusServer {
                socket_file_in,
                socket_file_out,
                expires: Option::None
            }
        )
    }

    pub fn cleanup(&self) {
        file_remove_if_exists(&self.socket_file_in);
        file_remove_if_exists(&self.socket_file_out);
    }

    fn start(&mut self, time: String) {
        let secs = parse_time_string(&time).unwrap();
        println!("[dummy] starting with {}", secs);

        self.expires = Some(get_time() + secs)
    }

    fn get_remaining(&self) -> u64 {
        match self.expires {
            Some(secs) => secs - get_time(),
            None       => 0
        }
    }

    pub fn listen(&mut self) {
        loop {
            println!("reading infile...");
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
                ClientRequest::Remaining => ServerResponse::Remaining(self.get_remaining())
            };
    
            fs::write(&self.socket_file_out, server_pack(response))
                .expect("Unable to write to outfile!");

            if should_halt {
                return
            }
        }
    }
}
