use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Message {
    sender: String,
    content: String,
    timestamp: u64,
}

impl Message {

    pub fn new(sender: String, content: String, timestamp: u64) -> Message {
        let msg = Message {
            sender: sender,
            content: content,
            timestamp: timestamp,
        };
        msg
    }

}


pub fn serialize_message(msg: Message) -> String {
    serde_json::to_string(&msg).unwrap()
}


pub fn deserialize_message(msg: String) -> Message {
    serde_json::from_str(&msg).unwrap()
}

