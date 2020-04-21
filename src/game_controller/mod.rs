use crate::dictionary;
use crate::game_state;
use crate::player_interface;

pub struct GameController<T: player_interface::PlayerInterface, U: dictionary::HangmanDictionary> {
    player_int: T,
    dict: U,
    gs: game_state::GameState,
}

impl<T: player_interface::PlayerInterface, U: dictionary::HangmanDictionary> GameController<T, U> {
    pub fn new(player_int: T, dict: U) -> GameController<T, U> {
        GameController {
            player_int: player_int,
            dict: dict,
            gs: game_state::GameState::new("".to_string()),
        }
    }

    pub fn run(&self) -> () {
        self.player_int.notify(&"Hi There".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dictionary;
    use crate::game_state;
    use crate::player_interface;

    #[test]
    fn test_ctor() {
        // This test is written without assertions
        // as a TDD Test against compilation
        let dummy_controller = GameController::new(
            player_interface::CommandLinePlayer::new(""),
            dictionary::SimpleDictionary::new("src/testdict.txt"),
        );
        dummy_controller.run();
    }
}

fn main() {
    let dummy_controller = GameController::new(
        player_interface::CommandLinePlayer::new("Bill".to_string()),
        dictionary::SimpleDictionary::new("src/testdict.txt".to_string()),
    );
    dummy_controller.run();
}
