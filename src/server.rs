use super::common::*;
use super::messages::*;

pub enum FocusServerError {
    AlreadyRunning,
    NoPermissions
}

pub struct FocusServer {
    zmq_context: zmq::Context,
    zmq_socket:  zmq::Socket,
    socket_file: String,
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

        let context = zmq::Context::new();
        let requester = context.socket(zmq::REP).unwrap();
        let connection = format!("ipc://{}", socket_file);
        requester.set_rcvtimeo(1000).unwrap();
        match requester.bind(&connection) {
            Ok(_)  => (),
            Err(_) => return Err(FocusServerError::AlreadyRunning)
        }

        return Ok(
            FocusServer {
                zmq_context: context,
                socket_file,
                zmq_socket: requester,
                expires: Option::None
            }
        )
    }

    pub fn cleanup(&self) {
        file_remove_if_exists(&self.socket_file)
    }

    pub fn listen(&self) {
        let mut msg = zmq::Message::new();
        loop {
            self.zmq_socket.recv(&mut msg, 0).unwrap();
            let cmsg = client_unpack(msg.to_vec());

            let packed: Vec<u8> = match cmsg {
                ClientRequest::Ping => {
                    server_pack(ServerResponse::Pong)
                },
                ClientRequest::Stop => panic!("notimplemented"),
                ClientRequest::Start(remaining) => panic!("notimplemented"),
                ClientRequest::Remaining => panic!("notimplemented"),
            };
            self.zmq_socket.send(packed, 0).unwrap()
        }
    }
}
