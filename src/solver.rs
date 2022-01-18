use std::collections::HashMap;
use std::str::FromStr;

use crate::game::{Game, GuessResult};
use crate::words::WordSource;
use crate::Word;

#[derive(Debug, Clone)]
pub enum SolverMode {
    MinEV,
    MinLogEV,
    Minimax,
}

impl FromStr for SolverMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match &s.to_lowercase()[..] {
            "minev" => Ok(Self::MinEV),
            "minlogev" => Ok(Self::MinLogEV),
            "minimax" => Ok(Self::Minimax),
            _ => Err("invalid solver mode".to_string()),
        }
    }
}

pub fn first_guess(_mode: &SolverMode, source: &WordSource) -> Option<String> {
    match source.letter_count() {
        5 => Some("tares".to_string()),
        6 => Some("salter".to_string()),
        7 => Some("saltier".to_string()),
        8 => Some("notaries".to_string()),
        _ => None,
    }
}

pub struct Solver<'a> {
    pub game: &'a mut Game,
    pub possible_solutions: Vec<String>,
    pub first_guess: bool,
}

impl<'a> Solver<'a> {
    pub fn new(game: &'a mut Game) -> Self {
        let mut possible_solutions: Vec<String> =
            game.wordlist.allowed_solutions.iter().cloned().collect();
        let mut first_guess = true;
        for guess in &game.guesses {
            first_guess = false;
            Self::filter_solutions(game, guess, &mut possible_solutions);
        }
        Self {
            game,
            possible_solutions,
            first_guess,
        }
    }

    fn filter_solutions(game: &Game, gr: &GuessResult, solutions: &mut Vec<String>) {
        solutions
            .retain(|solution| gr == &GuessResult::check(&gr.guess, solution, game.letter_count));
    }

    fn compute_score_minev(&self, guess: &Word) -> f64 {
        let mut results: HashMap<GuessResult, usize> = HashMap::new();
        for solution in &self.possible_solutions {
            let res = GuessResult::check(guess, solution, self.game.letter_count);
            let n = results.entry(res).or_insert(0);
            *n += 1;
        }
        results.values().map(|n| (n * n) as f64).sum()
    }

    fn compute_score_minlogev(&self, guess: &Word) -> f64 {
        let mut results: HashMap<GuessResult, usize> = HashMap::new();
        for solution in &self.possible_solutions {
            let res = GuessResult::check(guess, solution, self.game.letter_count);
            let n = results.entry(res).or_insert(0);
            *n += 1;
        }
        results
            .values()
            .map(|n| (*n as f64 * (*n as f64).log2()))
            .sum()
    }

    fn compute_score_minimax(&self, guess: &Word) -> f64 {
        let mut results: HashMap<GuessResult, usize> = HashMap::new();
        for solution in &self.possible_solutions {
            let res = GuessResult::check(guess, solution, self.game.letter_count);
            let n = results.entry(res).or_insert(0);
            *n += 1;
        }
        *results.values().max().unwrap() as f64
    }

    pub fn compute_score(&self, guess: &Word, mode: &SolverMode) -> f64 {
        match *mode {
            SolverMode::Minimax => self.compute_score_minimax(guess),
            SolverMode::MinEV => self.compute_score_minev(guess),
            SolverMode::MinLogEV => self.compute_score_minlogev(guess),
        }
    }

    fn find_guess(&self, mode: &SolverMode) -> Word {
        if self.possible_solutions.len() == 1 {
            return (&self.possible_solutions[0][..]).into();
        }
        let possible_guesses: Vec<Word> = self
            .game
            .wordlist
            .words_by_frequency
            .iter()
            .map(|s| (&s[..]).into())
            .collect();

        let mut best_guess = possible_guesses[0].clone();
        let mut best_score = self.compute_score(&best_guess, mode);

        for guess in possible_guesses {
            let score = self.compute_score(&guess, mode);
            // favor guess which is a potential solution
            if score == best_score
                && !self.possible_solutions.contains(&best_guess.to_string())
                && self.possible_solutions.contains(&guess.to_string())
            {
                best_guess = guess;
            } else if score < best_score {
                best_guess = guess;
                best_score = score;
            }
        }
        best_guess
    }

    pub fn guess(&mut self, mode: &SolverMode) {
        // Pre-computed best first guess
        let guess: Word = if self.first_guess {
            self.first_guess = false;
            match first_guess(mode, &self.game.wordlist.source) {
                Some(guess) => (&guess[..]).into(),
                None => self.find_guess(mode),
            }
        } else {
            self.find_guess(mode)
        };
        let res = self.game.guess(guess.to_string()).unwrap();
        Self::filter_solutions(self.game, &res, &mut self.possible_solutions);
    }
}
