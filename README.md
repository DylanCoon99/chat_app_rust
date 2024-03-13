# chat_app_rust

P2P chat application

Architecture Overview

1. Node Structure (node.rs): Each user is represented as a node with a unique 
identifier, a network address (socket), and a data structure containing the peers.

2. Discovery Mechanism (discovery/service.rs): Nodes use this to find and connect with other nodes (peers). 

3. Connection Management (connection/manager.rs): Manages active connections between peers, establishing new connections as needed and handling disconnections. Use Tokio's async TCP sockets for non-blocking network communication.

4. NAT Traversal (nat_traversal/techniques.rs)

5. Encryption (encryptor.rs, decryptor.rs): public-private key encryption

6. Messaging Protocol: For serialization format I will be using JSON.

7. CLI Interface (cli/interface.rs): user interface for the client nodes to send and receive messages, list peers, and select
