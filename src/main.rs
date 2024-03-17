mod cli;
mod serialization;

fn main() {
    cli::run();

    let msg = serialization::Message::new( 
        "node1".to_string(), 
        "This is from node1".to_string(), 
        1622543192);

    let json_msg = serialization::serialize_message(msg);
    println!("Serialized Message: {}", json_msg);

}
