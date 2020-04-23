use std::collections::HashSet;

/*pub struct game_state {
    game_state_impl: GameState,
}
*/

#[derive(Debug, PartialEq)]
pub struct GameState {
    SecretWord: String,
    GuessedLetters: HashSet<char>,
    pub State: State,
}

#[derive(Debug, PartialEq)]
pub enum State {
    Active(i16),
    GameOver,
    GameWon,
}

impl GameState {
    pub fn new(secret_word: String) -> GameState {
        GameState {
            SecretWord: secret_word,
            GuessedLetters: HashSet::new(),
            State: State::Active(5),
        }
    }

    pub fn evaluate_guess(&mut self, guess: char) {
        self.GuessedLetters.insert(guess);
        if !self.SecretWord.contains(guess) {
            self.update_after_wrong_guess();
        } else {
            self.update_after_correct_guess();
        }
    }

    pub fn get_disp_str(&self) -> String {
        let mut disp_str = String::new();
        for c in self.SecretWord.chars() {
            if self.GuessedLetters.contains(&c) {
                disp_str.push(c);
            } else {
                disp_str.push('_');
            }
        }
        disp_str
    }

    fn update_after_wrong_guess(&mut self) {
        match self.State {
            State::Active(x) => {
                if x > 1 {
                    self.State = State::Active(x - 1);
                } else {
                    self.State = State::GameOver;
                }
            }
            _ => return,
        }
    }

    fn update_after_correct_guess(&mut self) {
        match self.State {
            State::Active(x) => {
                if self.get_disp_str() == self.SecretWord {
                    self.State = State::GameWon;
                } 
            }
            _ => panic!("Unexpected state!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    #[test]
    fn test_ctor() {
        let expected = GameState {
            SecretWord: "foo".to_string(),
            GuessedLetters: HashSet::new(),
            State: State::Active(5),
        };
        let gs = GameState::new("foo".to_string());
        assert_eq!(gs, expected)
    }

    #[test]
    fn evaluate_wrong_guess() {
        let mut gs = GameState::new("foo".to_string());
        gs.evaluate_guess('a');
        let mut expected_set = HashSet::new();
        expected_set.insert('a');
        assert_eq!(gs.State, State::Active(4));
        assert_eq!(gs.GuessedLetters, expected_set);
    }

    #[test]
    fn evaluate_correct_sequence() {
        let mut gs = GameState::new("foo".to_string());
        gs.evaluate_guess('f');
        assert_eq!(gs.State, State::Active(5));
        gs.evaluate_guess('o');
        assert_eq!(gs.State, State::GameWon);
    }

    #[test]
    fn evaluate_wrong_sequence() {
        let mut gs = GameState::new("foo".to_string());
        gs.evaluate_guess('a');
        assert_eq!(gs.State, State::Active(4));
        gs.evaluate_guess('b');
        assert_eq!(gs.State, State::Active(3));
        gs.evaluate_guess('c');
        assert_eq!(gs.State, State::Active(2));
        gs.evaluate_guess('d');
        assert_eq!(gs.State, State::Active(1));
        gs.evaluate_guess('e');
        assert_eq!(gs.State, State::GameOver);

    }

    #[test]
    fn test_disp_str() {
        let mut gs = GameState::new("foo".to_string());
        let disp_str = gs.get_disp_str();
        assert_eq!(disp_str, "___");
        gs.evaluate_guess('f');
        let disp_str = gs.get_disp_str();
        assert_eq!(disp_str, "f__");
    }
}
