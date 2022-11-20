mod solution;

use crate::solution::two_sum::two_sum;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let problem: String = args[1].clone();

    match problem.as_str() {
        "two_sum" => two_sum::solve(vec![1, 2, 3], 5),
        _ => println!("No solution for problem {}", problem),
    }
}
