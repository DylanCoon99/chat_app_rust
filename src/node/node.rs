use std::net::SocketAddr;
use tokio::sync::{mpsc, Mutex};
use std::collections::HashSet;
//use std::sync::Arc;
use rsa::{RsaPrivateKey, RsaPublicKey};


pub struct Node {
    
    id: String,
    address: SocketAddr,
    peers: Mutex<HashSet<SocketAddr>>,
    sender: mpsc::Sender<String>,
    receiver: mpsc::Receiver<String>,
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,    
}

impl Node {
    
    pub fn new(id: String, address: SocketAddr) -> Node {
        // Init Constructor here
        let (sender, mut receiver) = mpsc::channel(1);

        let mut rng = rand::thread_rng();
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);

        let mut node = Node {
            id: id,
            address: address,
            peers: Mutex::new(HashSet::new()),
            sender: sender,
            receiver: receiver,
            private_key: private_key,
            public_key: public_key,
        };
        node
    }

    
    pub fn get_id(&self) -> String {
        return (self.id).clone();
    }

    pub fn get_addr(&self) -> SocketAddr {
        return (self.address).clone();
    }


    pub fn get_public_key(&self) -> RsaPublicKey {
        return (self.public_key).clone();
    }


    pub async fn add_peer(&self, peer_addr: SocketAddr) {
        let mut peers = self.peers.lock().await;
        peers.insert(peer_addr);
    }

    pub async fn remove_peer(&self, peer_addr: SocketAddr) {
        let mut peers = self.peers.lock().await;
        peers.remove(&peer_addr);
    }


      
    pub async fn connect_to_peer(&self, peer_addr: SocketAddr) {
        
    }

    pub async fn disconnect_from_peer(&self, peer_addr: SocketAddr) {
    
    }


    /*
    
    pub async fn send_message(&self, message: String, peer_addr_: SocketAddr) {
        // takes in an encrypted and serialized message and sends it to the correct recepient
        
    }

    pub async fn listen_for_messages(&self) {
        
    }


    fn handle_message(&self, message: String) {
        
    }
    

    
    pub async fn discover_peers(&self) {
        
    }
    */

}
