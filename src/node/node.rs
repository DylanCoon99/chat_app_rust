use std::net::SocketAddr;
use tokio::sync::{mpsc, Mutex};
use std::collections::HashSet;
//use std::sync::Arc;


pub struct Node {
    
    id: String,
    address: SocketAddr,
    peers: Mutex<HashSet<SocketAddr>>,
    sender: mpsc::Sender<String>,
    receiver: mpsc::Receiver<String>,
}

impl Node {
    
    pub fn new(id: String, address: SocketAddr) -> Node {
        // Init Constructor here
        let (sender, mut receiver) = mpsc::channel(1);

        let mut node = Node {
            id: id,
            address: address,
            peers: Mutex::new(HashSet::new()),
            sender: sender,
            receiver: receiver,
        };
        node
    }

    
    pub fn get_id(&self) -> String {
        return (self.id).clone();
    }

    pub fn get_addr(&self) -> SocketAddr {
        return (self.address).clone();
    }


    /*
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
    */

    fn async add_peer(&self, peer_addr: SocketAddr) {
        let mut peers = self.peers.lock().await;
        peers.insert(peer_addr);
    }

    fn remove_peer(&self, peer_addr: SocketAddr) {
        let mut peers = self.peers.lock().await;
        peers.remove(&addr);
    }

    /*
    pub async fn discover_peers(&self) {
        
    }
    */

}
