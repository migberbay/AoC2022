mod days;
mod utils;

use days::{day01
    // , day02, day03, day04, day05,
    // day06, day07, day08, day09, day10,
    // day11, day12, day13, day14, day15,
    // day16, day17, day18, day19, day20,
    // day21, day22, day23, day24, day25
};

use std::env;
use std::time::Instant;

pub type SolutionPair = (String, String);

fn main() {
    // Get console arguments when launching program.
    let args: Vec<String> = env::args().collect();

    // Check for minimum of 2 argumens (a day and something else?)
    if args.len() < 2 {
        panic!("Please provide the day(s) to run as a command-line argument.");
    }

    // iterate through all days and 
    let days: Vec<u8> = args[1..].iter()
        .map(|x| x.parse().unwrap_or_else(|v| panic!("Not a valid day: {v}")))
        .collect();

    // time taken for the day to run.
    let mut runtime = 0.0;

    for day in days {
            let func = get_day_solver(day);
    
            let time = Instant::now();
            let (p1, p2) = func();
            let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
            
            println!("\n=== Day {day:02} ===");
            println!("  · Part 1: {p1}");
            println!("  · Part 2: {p2}");
            println!("  · Elapsed: {elapsed_ms:.4} ms");
    
            runtime += elapsed_ms;
        }
    
        println!("Total runtime: {runtime:.4} ms");
}

fn get_day_solver(day: u8) -> fn() -> SolutionPair {
    match day {
        1 => day01::solve,
        _ => unimplemented!(),
    }
}