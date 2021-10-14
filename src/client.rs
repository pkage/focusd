use super::messages::*;
use colored::*;

pub enum FocusClientError {
    TimedOut,
    NoConnection,
    ServerError
}

pub struct FocusClient {
    zmq_context: zmq::Context,
    zmq_socket:  zmq::Socket,
    socket_file: String,
}

impl FocusClient {
    pub fn new(socket_file: String) -> Result<FocusClient, FocusClientError> {
        let context = zmq::Context::new();

        let requester = context.socket(zmq::REQ).unwrap();
        let connection = format!("ipc://{}", socket_file);

        requester.set_rcvtimeo(1000).unwrap();

        match requester.connect(&connection) {
            Ok(()) => (),
            Err(_) => return Err(FocusClientError::NoConnection)
        }
        
        return Ok(FocusClient {
            socket_file: connection,
            zmq_socket: requester,
            zmq_context: context
        })
    }

    pub fn destroy(&mut self) -> () {
        println!("disconnecting socket from {}", self.socket_file);
        self.zmq_socket.disconnect(&self.socket_file).unwrap();
        println!("destroying context...");
        self.zmq_context.destroy().unwrap();
        println!("cleanup should be done?");
    }

    fn make_request(&self, msg: ClientRequest) -> Result<ServerResponse, FocusClientError> {
        let packed = client_pack(msg);

        self.zmq_socket.send(packed, 0).unwrap();

        let mut msg = zmq::Message::new();

        match self.zmq_socket.recv(&mut msg, 0) {
            Ok(_) => {
                return Ok(server_unpack(msg.to_vec()))
            },
            Err(_) => {
                return Err(FocusClientError::ServerError)
            }
        }
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
            println!("{}", "failed to pong!".green());
        }
    }
}

