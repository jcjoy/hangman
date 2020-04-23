use crate::dictionary;
use crate::game_state;
use crate::player_interface;

pub struct GameController<T : player_interface::PlayerInterface> {
    player_int: T,
    gs: game_state::GameState,
}

impl<T: player_interface::PlayerInterface> GameController<T> {
    pub fn new(player_int: T) -> GameController<T> {
        // We must bring the trait into scope in order to use its methods
        use dictionary::HangmanDictionary;
        let dict = dictionary::SimpleDictionary::new("src/testdict.txt".to_string());
        let secret_word = dict.random_word();
        GameController {
            player_int: player_int,
            gs: game_state::GameState::new(secret_word),
        }
    }

    pub fn run(&mut self) -> () {
        self.player_int.notify(&"Hi There".to_string());
        self.player_int.notify(&"Let's play hangman!".to_string());
        loop {
            match self.gs.State {
                game_state::State::Active(_) => {
                    let disp_str = self.gs.get_disp_str();
                    self.player_int.notify(&"Current string ");
                    self.player_int.notify(&disp_str);
                    let guess = self.player_int.query(&"Enter Guess: ".to_string());
                    self.gs.evaluate_guess(guess);
                },
                game_state::State::GameWon => {
                    self.player_int.notify(&"Congratulations! You Win!");
                    break
                },
                game_state::State::GameOver => {
                    self.player_int.notify(&"Sorry! You lose!");
                    break
                },
            }
        }
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
        let mut dummy_controller = GameController::new(
            player_interface::CommandLinePlayer::new("".to_string()),
        );
        dummy_controller.run();
    }
}

fn main() {
    let mut dummy_controller = GameController::new(
        player_interface::CommandLinePlayer::new("Bill".to_string()),
    );
    dummy_controller.run();
}
