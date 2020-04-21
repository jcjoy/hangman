use rand::Rng;
use std::fs;
use std::vec::Vec;

pub trait HangmanDictionary {
    fn random_word(&self) -> String;
    fn all_words(&self) -> Vec<String>;
}

#[derive(Clone)]
pub struct SimpleDictionary {
    _words: Vec<String>,
    _size: usize,
}

impl SimpleDictionary {
    pub fn new(filename: String) -> SimpleDictionary {
        let wordstring = fs::read_to_string(filename).expect("Something went wrong with the read!");
        let words: Vec<String> = wordstring
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();
        let size: usize = words.len();
        SimpleDictionary {
            _words: words,
            _size: size,
        }
    }
}

impl HangmanDictionary for SimpleDictionary {
    fn random_word(&self) -> String {
        let idx: usize = rand::thread_rng().gen_range(0, self._size);
        self._words[idx].clone()
    }

    fn all_words(&self) -> Vec<String> {
        self._words.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ctor() {
        let dict = SimpleDictionary::new("src/testdict.txt".to_string());
        let words = dict.all_words();
        let expected = vec!("hello", "world", "good", "bad");
        assert_eq!(words, expected);
    }
}
