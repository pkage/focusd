
// use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ClientRequest {
    Ping,
    Start(String),
    Remaining,
    Halt
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum ServerResponse {
    Pong,
    Remaining(u64),
    Failed,
    Halting,
    Starting,
    AlreadyStarted
}

pub fn client_pack(req: ClientRequest) -> Vec<u8> {
    rmp_serde::to_vec(&req).unwrap()
}

pub fn client_unpack(req: Vec<u8>) -> ClientRequest {
    rmp_serde::from_read_ref(&req).unwrap()
}

pub fn server_pack(res: ServerResponse) -> Vec<u8> {
    rmp_serde::to_vec(&res).unwrap()
}

pub fn server_unpack(res: Vec<u8>) -> ServerResponse {
    rmp_serde::from_read_ref(&res).unwrap()
}

