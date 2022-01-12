use structopt::StructOpt;

use wordsmith::game::Game;
use wordsmith::solver::Solver;
use wordsmith::words::WordSource;
use wordsmith::Word;

#[derive(Debug, StructOpt)]
#[structopt(name = "ws-first")]
struct Opt {
    #[structopt(short, long, default_value = "wordle")]
    word_source: WordSource,
}

fn main() {
    let opt = Opt::from_args();
    let mut game = Game::from_source(&opt.word_source);
    let wordlist = game.wordlist.clone();
    let solver: Solver = Solver::new(&mut game);

    let mut best_guess: String = "".to_string();
    let mut best_ev: f64 = wordlist.allowed_solutions.len() as f64;
    let wordlist_len = wordlist.words.len();
    let soln_len: f64 = wordlist.allowed_solutions.len() as f64;
    let mut results: Vec<(String, f64)> = vec![];
    for (i, guess) in wordlist.words.iter().enumerate() {
        let guess_word: Word = (&guess[..]).into();
        let ev = solver.compute_expected_n_solutions(&guess_word);
        results.push((guess.to_string(), soln_len / ev));
        if ev < best_ev {
            best_guess = guess.to_string();
            best_ev = ev;
        }
        println!(
            "{}/{} Evaluated {}: {:.2}. Best: {}: {:.2}",
            i + 1,
            wordlist_len,
            guess,
            soln_len / ev,
            best_guess,
            soln_len / best_ev
        );
    }
    results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    results.reverse();
    println!("Top 10 for {:?}", opt.word_source);
    for (guess, score) in results.iter().take(10) {
        println!("{}: {:.2}", guess, score);
    }
}
