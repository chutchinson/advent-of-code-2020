mod prelude;
mod context;
mod solutions;

use context::{Context, Solution};
use solutions::*;

fn main() {
    let solutions = vec![
        (1, day1::solve)
    ];

    let result = solutions.into_iter()
        .map(|(day, solver)| solve(day, solver))
        .fold(true, |x, y| x & y);
        
    std::process::exit(if result { 0 } else { 1 });
}

fn solve(day: usize, solver: Solution) -> bool {
    match Context::solve(day, solver) {
        Ok(_) => true,
        Err(_) => {
            eprintln!("failed to solve day {}", day);
            false
        }
    }
}