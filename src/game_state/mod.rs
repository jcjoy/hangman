use std::collections::HashSet;

/*pub struct game_state {
    game_state_impl: GameState,
}
*/

#[derive(Debug, PartialEq)]
pub struct GameState {
    SecretWord: String,
    GuessedLetters: HashSet<char>,
    GuessesRemaining: i16,
}

impl GameState {
    pub fn new(secret_word: String) -> GameState {
        GameState {
            SecretWord: secret_word,
            GuessedLetters: HashSet::new(),
            GuessesRemaining: 5,
        }
    }

    pub fn evaluate_guess(&mut self, guess: char) {
        if !self.SecretWord.contains(guess) {
            self.GuessesRemaining -= 1;
        }
        self.GuessedLetters.insert(guess);
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
            GuessesRemaining: 5,
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
        assert_eq!(gs.GuessesRemaining, 4);
        assert_eq!(gs.GuessedLetters, expected_set);
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
