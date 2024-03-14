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



Sending a Message

1. User Command: Node A sends message to Node B via CLI. The app serializes the message, encrypts it, and prepares it for transmission.

2. Discovery: If Node A does not have Node B in the list of known peers, it uses a discovery mechanism to locate Node B.

3. Connection Establishment: Once Node A knows how to find Node B, checks if there is an existing connection. If no such connection exists, establish a connection using Tokio's async networking capabilities. NAT traversal techniques is applied here.

4. Message Transmission: Message is sent over the established connection. Tokio handles the async operation.

5. Message Reception: Node B receives the message. Tokio runtime on Node B handles the async read from the network, decrypts, and seserializes the message.

6. Recepient Confirmation: Node B may send an acknowledgement back to Node A to confirm the message was sent



Tokio Runtime Environment

1. Asynchronous Operations: Tokio enables the app to perform non-blocking network IO, meaning nodes can handle multiple connections and tasks  simultaneously w/o waiting for one task to complete before starting another. 

2. Concurrency Management: Tokio provides a way to manage concurrency with minimal overhead, using futures and tasks instead of threads, which is particularly beneficial for IO-bound apps like a P2P chat.

3. Resource Efficiency: By allowing for concurrent handling of tasks within a single or a few threads, Tokio makes efficient use of system resources, which is essential for scalable apps.
