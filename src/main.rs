mod dictionary;
mod player_interface;
mod game_controller;
mod game_state;

fn main() {
    let mut dummy_controller = game_controller::GameController::new(
            player_interface::CommandLinePlayer::new("Billy Bob".to_string())
    );
    dummy_controller.run();

    println!("Hello, world!");
}
