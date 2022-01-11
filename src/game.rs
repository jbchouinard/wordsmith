use std::collections::HashMap;
use std::rc::Rc;

use druid::im::Vector;
use druid::Data;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::words::{WordList, WordSource};
use crate::{Letter, Word};

#[derive(Clone, Data)]
pub struct Game {
    pub letter_count: usize,
    pub tries: usize,
    pub solution: String,
    pub wordlist: Rc<WordList>,
    pub guesses: Vector<GuessResult>,
}

impl Game {
    fn new(letter_count: usize, wordlist: Rc<WordList>) -> Self {
        let solution = wordlist
            .allowed_solutions
            .iter()
            .collect::<Vec<&String>>()
            .choose(&mut thread_rng())
            .unwrap()
            .to_string();
        Self {
            letter_count,
            wordlist,
            guesses: Vector::new(),
            tries: 5,
            solution,
        }
    }
    pub fn from_source(source: &WordSource) -> Self {
        Self::new(
            source.letter_count(),
            Rc::new(WordList::from_source(source)),
        )
    }
    pub fn set_solution(&mut self, solution: String) {
        if !self.wordlist.is_valid_solution(&solution) {
            panic!("invalid solution");
        }
        self.solution = solution;
    }
    pub fn restart(&mut self) {
        self.guesses = Vector::new();
    }
    pub fn state(&self) -> State {
        let n = self.guesses.len();
        if n == 0 {
            State::Unsolved
        } else if self.guesses[n - 1].is_solved() {
            State::Solved
        } else if n == self.tries {
            State::Failed
        } else {
            State::Unsolved
        }
    }
    pub fn letter_states(&self) -> HashMap<Letter, LetterState> {
        let mut map = HashMap::new();
        for guess in &self.guesses {
            for i in 0..guess.guess.len() {
                let letter = guess.guess[i];
                let letter_match = &guess.result[i];
                let pre_res = map.get(&letter).unwrap_or(&LetterState::Unknown);
                let res = match (pre_res, letter_match) {
                    (_, LetterMatch::Exact) => LetterState::Exact,
                    (LetterState::Exact, _) => LetterState::Exact,
                    (_, LetterMatch::Partial) => LetterState::Partial,
                    (LetterState::Partial, _) => LetterState::Partial,
                    (_, _) => LetterState::Eliminated,
                };
                map.insert(letter, res);
            }
        }
        map
    }
    pub fn guess(&mut self, guess: String) -> Result<GuessResult, GuessError> {
        if !self.wordlist.is_valid_guess(&guess) {
            return Err(GuessError::InvalidGuess(guess));
        }
        match &self.state() {
            State::Unsolved => {
                let result =
                    GuessResult::check(&((&guess[..]).into()), &self.solution, self.letter_count);
                self.guesses.push_back(result.clone());
                Ok(result)
            }
            State::Solved => Err(GuessError::GameFinished(State::Solved)),
            State::Failed => Err(GuessError::GameFinished(State::Failed)),
        }
    }
    pub fn letter_count(&self) -> usize {
        self.letter_count
    }
}

#[derive(Clone, Debug)]
pub enum State {
    Unsolved,
    Solved,
    Failed,
}

#[derive(Clone, Debug, Data, PartialEq, Eq)]
pub enum LetterMatch {
    Exact,
    Partial,
    Wrong,
}

#[derive(Clone, Debug, Data, PartialEq, Eq)]
pub struct GuessResult {
    pub guess: Word,
    pub result: Vector<LetterMatch>,
}

impl GuessResult {
    pub fn check(guess_word: &Word, solution: &str, letter_count: usize) -> Self {
        let mut res: Vector<LetterMatch> = Vector::new();

        let solution_word: Word = solution.into();

        let mut count_nonexact_guess: [u8; 26] = [0; 26];
        let mut count_nonexact_solution: [u8; 26] = [0; 26];

        for i in 0..letter_count {
            if guess_word[i] != solution_word[i] {
                count_nonexact_solution[solution_word[i].as_index()] += 1;
            }
        }

        for i in 0..letter_count {
            let guess_letter: Letter = guess_word[i];
            let solution_letter: Letter = solution_word[i];
            if guess_letter == solution_letter {
                res.push_back(LetterMatch::Exact);
            } else if count_nonexact_guess[guess_letter.as_index()]
                < count_nonexact_solution[guess_letter.as_index()]
            {
                res.push_back(LetterMatch::Partial);
                count_nonexact_guess[guess_letter.as_index()] += 1;
            } else {
                res.push_back(LetterMatch::Wrong);
            }
        }
        Self {
            guess: guess_word.clone(),
            result: res,
        }
    }
    pub fn is_solved(&self) -> bool {
        for m in &self.result {
            match m {
                LetterMatch::Exact => {
                    continue;
                }
                _ => return false,
            }
        }
        true
    }
}

#[derive(Clone, Debug)]
pub enum GuessError {
    InvalidGuess(String),
    GameFinished(State),
}

pub enum LetterState {
    Exact,
    Partial,
    Eliminated,
    Unknown,
}

#[cfg(test)]
mod test {
    use super::*;
    use druid::im::vector;
    use LetterMatch::*;

    #[test]
    fn test_guess_check_solved() {
        let actual = GuessResult::check(&"relax".into(), "relax", 5);
        let expected = GuessResult {
            guess: "relax".into(),
            result: vector![Exact, Exact, Exact, Exact, Exact],
        };
        assert_eq!(expected, actual);
        assert!(actual.is_solved());
    }

    #[test]
    fn test_guess_check_wrong() {
        let actual = GuessResult::check(&"relax".into(), "bbbbb", 5);
        let expected = GuessResult {
            guess: "relax".into(),
            result: vector![Wrong, Wrong, Wrong, Wrong, Wrong],
        };
        assert_eq!(expected, actual);
        assert!(!actual.is_solved());
    }

    #[test]
    fn test_guess_check_partial_1() {
        let actual = GuessResult::check(&"accca".into(), "aaabb", 5);
        let expected = GuessResult {
            guess: "accca".into(),
            result: vector![Exact, Wrong, Wrong, Wrong, Partial],
        };
        assert_eq!(expected, actual);
        assert!(!actual.is_solved());
    }

    #[test]
    fn test_guess_check_partial_2() {
        let actual = GuessResult::check(&"acaaa".into(), "aabbb", 5);
        let expected = GuessResult {
            guess: "acaaa".into(),
            result: vector![Exact, Wrong, Partial, Wrong, Wrong],
        };
        assert_eq!(expected, actual);
        assert!(!actual.is_solved());
    }
}
