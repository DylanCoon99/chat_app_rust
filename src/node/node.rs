use std::net::SocketAddr;
use tokio::sync::{mpsc, Mutex};
use std::collections::HashSet;
use std::sync::Arc;


pub struct Node {
    
    id: String,
    address: SocketAddr,
    peers: Mutex<HashSet<SocketAddr>>,
    sender: mpsc::Sender<String>,
    receiver: mpsc::Receiver<String>,
}

impl Node {
    
    pub fn new(id:String, address: SocketAddr) => Self {
        // Init Constructor here
    }

    pub async fn connect_to_peer(&self, peer_addr: SocketAddr) {
        
    }

    pub aysnc fn disconnect_from_peer(&self, peer_addr: SocketAddr) {

    }

    pub async fn send_message() {

    }

    pub async fn listen_for_messages(&self) {
        
    }


    fn handle_message(&self, message: String) {
        
    }

    fn add_peer(&self, peer_addr: SocketAddr) {

    }

    fn remove_peer(&self, peer_addr: SocketAddr) {
        
    }

    pub async fn discover_peers(&self) {
        
    }


}
