use std::io::stdin;

pub trait PlayerInterface {
    fn notify(&self, message: &str) -> ();
    fn query(&self, message: &str) -> char;
}

pub struct CommandLinePlayer {
    PlayerId: String,
}

impl CommandLinePlayer {
    pub fn new(player_id: String) -> CommandLinePlayer {
        CommandLinePlayer {
            PlayerId: player_id,
        }
    }
}

impl PlayerInterface for CommandLinePlayer {
    fn notify(&self, message: &str) -> () {
        println!("{}", message);
    }

    fn query(&self, message: &str) -> char {
        println!("{}", message);
        loop {
            let mut buf: String = String::new();
            stdin().read_line(&mut buf).expect("Could not read line");
            // Check to see if 2 chars are entered.
            // This is sort of a hack to get around the fact that
            // newline will be read by stdin as well.
            if buf.chars().count() == 2 {
                match buf.chars().next() {
                    Some(x) => return x,
                    _ => {println!("Please input a valid character!")},
                }
            } else {
                println!("Please only input one character at a time!");
            }
        }
    }
}

fn main() {
    let interface = CommandLinePlayer::new("Bob".to_string());
    interface.notify(&"Trying this out".to_string());
    let response = interface.query(&"What's your answer?");
    println!("RESPONSE: {}", response);
}
