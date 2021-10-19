use super::time;
use super::common::*;
use super::config::*;
use super::messages::*;
use colored::*;

use std::fs;

pub enum FocusClientError {
    // TimedOut,
    NoConnection,
    // ServerError
}

pub struct FocusClient {
    socket_file_in:  String,
    socket_file_out: String
}

impl FocusClient {
    pub fn new(config: &FocusConfig) -> Result<FocusClient, FocusClientError> {

        let socket_file = &config.socket_file;

        let socket_file_in  = format!("{}.in", socket_file);
        let socket_file_out = format!("{}.out", socket_file);

        if !file_exists(&socket_file_in) || !file_exists(&socket_file_out) {
            return Err(FocusClientError::NoConnection);
        }

        return Ok(FocusClient {
            socket_file_in,
            socket_file_out
        })
    }

    fn make_request(&self, msg: ClientRequest) -> Result<ServerResponse, FocusClientError> {
        let packed = client_pack(msg);

        fs::write(&self.socket_file_in, packed)
            .expect("Unable to write to infile!");

        let response = fs::read(&self.socket_file_out)
            .expect("Unable to read from outfile!");

        Ok(server_unpack(response))
    }

    pub fn ping(&self) {
        print!("pinging... ");

        let res = match self.make_request(
            ClientRequest::Ping
        ) {
            Ok(_) => true,
            Err(_)=> false
        };


        if res {
            println!("{}", "ponged!".green());
        } else {
            println!("{}", "failed to pong!".red());
        }
    }

    pub fn halt(&self) {
        print!("halting... ");
        match self.make_request(
            ClientRequest::Halt
        ) {
            Ok(_) => println!("{}", "server halted!".green()),
            Err(_) => println!("{}", "failed!".red()),
        };
    }

    pub fn start(&self, length: String) {
        let res = match self.make_request(ClientRequest::Start(length)) {
            Ok(r) => r,
            Err(_) => return
        };

        match res {
            ServerResponse::Starting       => println!("{}", "started!".green()),
            ServerResponse::AlreadyStarted => println!("{}", "already started!".red()),
            _ => ()
        }
    }

    pub fn remaining(&self, raw: bool) {
        let res = match self.make_request(ClientRequest::Remaining) {
            Ok(r) => r,
            Err(_) => return
        };

        match res {
            ServerResponse::Remaining(num) => {
                if raw {
                    println!("{}", num)
                } else {
                    println!("{}", time::create_time_string(num))
                }
            }
            _ => ()
        }
    }
}

