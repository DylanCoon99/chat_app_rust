use chat_app::node::node::Node;
use std::net::{IpAddr, Ipv4Addr};
use std::net::SocketAddr; 


#[test]
fn node_instantiation() {
    let node_id = String::from("node1");
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let node = Node::new(node_id.clone(), socket);

    assert_eq!(node.get_id(), node_id);
    assert_eq!(node.get_addr(), socket);
   
}


#[test]
fn node_insert() {
    let node_id = String::from("node1");
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let node = Node::new(node_id.clone(), socket);

    
    let socket2 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)), 8080);
    node.add_peer(socket2.clone());




}

