mod dictionary;
mod player_interface;
mod game_controller;
mod game_state;

fn main() {
    let dummy_controller = game_controller::GameController::new(
        player_interface::CommandLinePlayer::new("Billy Bob".to_string()),
        dictionary::SimpleDictionary::new("testdict.txt".to_string()),
    );
    dummy_controller.run();

    println!("Hello, world!");
}
