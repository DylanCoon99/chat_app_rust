use clap::{App, Arg, SubCommand};

pub fn run() {
    let matches = App::new("P2P Chat App")
        .version("1.0")
        .author("Dylan Coon")
        .subcommand(SubCommand::with_name("showpeers")
                .about("Displays all known peers"))
        .subcommand(SubCommand::with_name("showconnections")
                .about("Displays active connections"))
        .subcommand(SubCommand::with_name("send")
                .about("Sends message to currently selected peer")
                .arg(Arg::with_name("recepient")
                     .help("The recepient's identifier")
                     .required(true))
                .arg(Arg::with_name("message")
                     .help("The message to send")
                     .required(true)))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("showpeers") {
        // functionality to show peers
    } else if let Some(_) = matches.subcommand_matches("showconnections") {
        // functionality to show connections
    } else if let Some(matches) = matches.subcommand_matches("send") {
        let _recepient = matches.value_of("recepient").unwrap();
        let _message = matches.value_of("message").unwrap();
        //functionality to send a message
        
        // determine if the client is connected with the recepient
            // if not, make the connection
            // if connection exists -> send the message
    }
}
