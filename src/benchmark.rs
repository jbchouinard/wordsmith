use std::time::Instant;

use wordsmith::game::{Game, State};
use wordsmith::solver::{Solver, SolverMode};
use wordsmith::words::WordSource;

fn main() {
    let mut game = Game::from_source(&WordSource::Scrabble {
        letter_count: 5,
        top_n: 1000,
    });

    let mut n_guesses: Vec<usize> = vec![];
    let mut n_failed: usize = 0;
    let allowed_solutions = game.wordlist.allowed_solutions.clone();
    let n_total: usize = allowed_solutions.len();

    let start = Instant::now();
    for (i, solution) in allowed_solutions.iter().enumerate() {
        println!("Solving game {} of {} ({}).", i, n_total, solution);
        game.set_solution(solution.to_string());
        game.restart();
        let mut solver: Solver = Solver::new(&mut game);
        while let State::Unsolved = solver.game.state() {
            solver.guess(SolverMode::Good);
        }
        match solver.game.state() {
            State::Solved => n_guesses.push(solver.game.guesses.len()),
            _ => n_failed += 1,
        }
    }
    let end = Instant::now();
    let mut sum: f64 = 0.0;
    let mut count: f64 = 0.0;
    for n in n_guesses {
        sum += n as f64;
        count += 1.0;
    }
    println!("Failed to solve {} puzzles.", n_failed);
    println!(
        "Solved in average of {:.2} guesses, {:.2} ms/puzzle",
        (sum / count),
        ((end - start).as_millis() as f64 / count)
    );
}
