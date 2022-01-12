use structopt::StructOpt;

use wordsmith::game::Game;
use wordsmith::solver::{Solver, SolverMode};
use wordsmith::words::WordSource;
use wordsmith::Word;

#[derive(Debug, StructOpt)]
#[structopt(name = "ws-first")]
struct Opt {
    #[structopt(short, long, default_value = "wordle")]
    word_source: WordSource,
    #[structopt(short, long, default_value = "minev")]
    mode: SolverMode,
}

fn main() {
    let opt = Opt::from_args();
    let mut game = Game::from_source(&opt.word_source);
    let wordlist = game.wordlist.clone();
    let solver: Solver = Solver::new(&mut game);

    let mut best_guess: String = "".to_string();
    let mut best_ev: f64 = f64::MAX;
    let wordlist_len = wordlist.words.len();
    let mut results: Vec<(String, f64)> = vec![];
    for (i, guess) in wordlist.words.iter().enumerate() {
        let guess_word: Word = (&guess[..]).into();
        let ev = solver.compute_score(&guess_word, &opt.mode);
        results.push((guess.to_string(), ev));
        if ev < best_ev {
            best_guess = guess.to_string();
            best_ev = ev;
        }
        println!(
            "{}/{} Evaluated {}: {:.2}. Best: {}: {:.2}",
            i + 1,
            wordlist_len,
            guess,
            ev,
            best_guess,
            best_ev,
        );
    }
    results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    println!("Top 10 for {:?} ({:?})", opt.word_source, opt.mode);
    for (guess, score) in results.iter().take(10) {
        println!("{}: {:.2}", guess, score);
    }
}
