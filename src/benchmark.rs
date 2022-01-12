use std::time::Instant;

use structopt::StructOpt;

use wordsmith::game::{Game, State};
use wordsmith::solver::{Solver, SolverMode};
use wordsmith::words::WordSource;

#[derive(Debug, StructOpt)]
#[structopt(name = "ws-benchmark")]
struct Opt {
    #[structopt(short, long, default_value = "wordle")]
    word_source: WordSource,
    #[structopt(short, long, default_value = "best")]
    mode: SolverMode,
}

fn main() {
    let opt = Opt::from_args();
    let mut game = Game::from_source(&opt.word_source);

    let mut n_guesses: Vec<usize> = vec![];
    let mut n_failed: usize = 0;
    let allowed_solutions = game.wordlist.allowed_solutions.clone();
    let n_total: usize = allowed_solutions.len();

    let start = Instant::now();
    for (i, solution) in ["hitch"].iter().enumerate() {
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
                    "{}/{} Solved {} in {} guesses.",
                    i + 1,
                    n_total,
                    solution,
                    n
                );
                n_guesses.push(n);
            }
            _ => {
                println!("{}/{} Failed to solve {}.", i + 1, n_total, solution);
                n_failed += 1
            }
        }
    }
    let end = Instant::now();
    let mut sum: f64 = 0.0;
    let mut count: f64 = 0.0;
    let mut max: usize = 0;
    for n in n_guesses {
        sum += n as f64;
        count += 1.0;
        if n > max {
            max = n;
        }
    }
    println!("Failed to solve {} puzzles.", n_failed);
    println!(
        "Solved in average of {:.2} guesses, {:.2} ms/puzzle, max {} guesses",
        (sum / count),
        ((end - start).as_millis() as f64 / count),
        max
    );
}
