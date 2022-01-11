use crate::game::{Game, GuessResult};
use crate::Word;

const TRESHOLD_BEST: f64 = 0.0;
const TRESHOLD_GOOD: f64 = 0.125;
const TRESHOLD_FAST: f64 = 0.25;

pub enum SolverMode {
    Best,
    Good,
    Fast,
}

impl SolverMode {
    pub fn value(&self) -> f64 {
        match self {
            Self::Best => TRESHOLD_BEST,
            Self::Good => TRESHOLD_GOOD,
            Self::Fast => TRESHOLD_FAST,
        }
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

    fn count_solutions(game: &Game, gr: &GuessResult, solutions: &Vec<String>) -> usize {
        let mut sum: usize = 0;
        for solution in solutions {
            if gr == &GuessResult::check(&gr.guess, solution, game.letter_count) {
                sum += 1;
            }
        }
        sum
    }

    fn compute_expected_n_solutions(&self, guess: &Word) -> f64 {
        let mut ns: Vec<usize> = vec![];
        for soln in &self.possible_solutions {
            let res = GuessResult::check(guess, soln, self.game.letter_count);
            ns.push(Self::count_solutions(
                self.game,
                &res,
                &self.possible_solutions,
            ));
        }
        let mut sum: f64 = 0.0;
        let mut count: f64 = 0.0;
        for n in ns {
            sum += n as f64;
            count += 1.0;
        }
        sum / count
    }

    fn find_guess(&self, mode: SolverMode) -> Word {
        if self.possible_solutions.len() == 1 {
            return (&self.possible_solutions[0][..]).into();
        }
        let possible_guesses: Vec<Word> = self
            .game
            .wordlist
            .words
            .iter()
            .map(|s| (&s[..]).into())
            .collect();
        let mut best_guess = possible_guesses[0].clone();
        let current_n = self.possible_solutions.len() as f64;
        let mut best_n = current_n;
        for guess in possible_guesses {
            let expected_n = self.compute_expected_n_solutions(&guess);
            if expected_n <= (current_n * mode.value()) {
                return guess;
            }
            if expected_n == best_n
                && !self.possible_solutions.contains(&best_guess.to_string())
                && self.possible_solutions.contains(&guess.to_string())
            {
                best_guess = guess;
            } else if expected_n < best_n {
                best_guess = guess;
                best_n = expected_n;
            }
        }
        best_guess
    }

    pub fn guess(&mut self, mode: SolverMode) {
        // Pre-computed best first guess
        let guess: Word = if self.first_guess {
            self.first_guess = false;
            "roast".into()
        } else {
            self.find_guess(mode)
        };
        let res = self.game.guess(guess.to_string()).unwrap();
        Self::filter_solutions(self.game, &res, &mut self.possible_solutions);
    }
}
