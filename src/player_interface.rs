use std::io::stdin;

trait PlayerInterface {
    fn notify(&self, message: &String) -> ();
    fn query(&self, message: &String) -> String;
}

pub struct CommandLinePlayer {
    PlayerId: String,
}

impl CommandLinePlayer {
    fn new(player_id: String) -> CommandLinePlayer {
        CommandLinePlayer {
            PlayerId: player_id,
        }
    }
}

impl PlayerInterface for CommandLinePlayer {
    fn notify(&self, message: &String) -> () {
        println!("{}", message);
    }

    fn query(&self, message: &String) -> String {
        println!("{}", message);
        let mut response = String::new();
        stdin()
            .read_line(&mut response)
            .expect("Could not read line");
        response
    }
}

fn main() {
    let interface = CommandLinePlayer::new("Bob".to_string());
    interface.notify(&"Trying this out".to_string());
    let response = interface.query(&"What's your answer?".to_string());
    println!("RESPONSE: {}", response);
}
