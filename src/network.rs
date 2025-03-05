use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct Node {
    peers: Vec<String>,
    port: u16,
}

impl Node {
    pub fn new(port: u16) -> Self {
        Node {
            peers: Vec::new(),
            port,
        }
    }

    pub fn start(&self) {
        // Start listening for incoming connections
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port)).unwrap();

        for stream in listener.incoming() {
            // Handle incoming messages from other nodes
            let stream = stream.unwrap();
            self.handle_connection(stream);
        }
    }

    fn handle_connection(&self, mut _stream: TcpStream) {
        // Read and process messages from peers
    }

    pub fn broadcast_block(&self, block_data: &[u8]) {
        // Send new block to all peers
        for peer in &self.peers {
            if let Ok(mut stream) = TcpStream::connect(peer) {
                let _ = stream.write_all(block_data);
            }
        }
    }
}
