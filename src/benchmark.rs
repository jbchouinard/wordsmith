use std::time::Instant;

use structopt::StructOpt;

use wordsmith::counter::Counter;
use wordsmith::game::{Game, State};
use wordsmith::solver::{Solver, SolverMode};
use wordsmith::words::WordSource;

#[derive(Debug, StructOpt)]
#[structopt(name = "ws-benchmark")]
struct Opt {
    #[structopt(short, long, default_value = "wordle")]
    word_source: WordSource,
    #[structopt(short, long, default_value = "minev")]
    mode: SolverMode,
}

fn main() {
    let opt = Opt::from_args();
    let mut game = Game::from_source(&opt.word_source);

    let mut guess_counter = Counter::new();
    let mut n_failed: usize = 0;
    let allowed_solutions = game.wordlist.allowed_solutions.clone();
    let n_total: usize = allowed_solutions.len();

    let start = Instant::now();
    for (i, solution) in allowed_solutions.iter().enumerate() {
        game.set_solution(solution.to_string());
        game.restart();
        let mut solver: Solver = Solver::new(&mut game);
        while let State::Unsolved = solver.game.state() {
            solver.guess(&opt.mode);
        }
        match solver.game.state() {
            State::Solved => {
                let n = solver.game.guesses.len();
                println!(
                    "{}",
                    game.guesses
                        .iter()
                        .map(|g| g.guess.to_string().to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                );
                guess_counter.add(n);
            }
            _ => {
                println!("{}/{} Failed to solve {}.", i + 1, n_total, solution);
                n_failed += 1
            }
        }
    }
    let end = Instant::now();
    println!("Failed to solve {} puzzles.", n_failed);
    let mut acc_n: usize = 0;
    for (guesses, n) in guess_counter.as_vec() {
        acc_n += n;
        println!(
            "{} guesses: {} ({:.1}% | {:.1}%)",
            guesses,
            n,
            100.0 * (n as f64) / (n_total as f64),
            100.0 * (acc_n as f64) / (n_total as f64)
        );
    }
    println!(
        "Average: {:.2} guesses, {:.2} ms/puzzle",
        (guess_counter.sum() as f64) / (guess_counter.count() as f64),
        ((end - start).as_millis() as f64 / guess_counter.count() as f64),
    );
}
