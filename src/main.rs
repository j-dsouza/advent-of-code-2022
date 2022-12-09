use std::time::Instant;
use clap::Parser;

use crate::aoc_core::{Args, Solution};

mod aoc_core;
mod day_1;
mod day_2;
mod day_3;

fn main() {
    let args = Args::parse();

    let start = Instant::now();

    let output = match args.day {
        1 => day_1::solve().unwrap(),
        2 => day_2::solve().unwrap(),
        3 => day_3::solve().unwrap(),
        _ => Solution{part_1: -1, part_2: -1}
    };

    println!("{}", output);

    println!("Time taken (ms): {}", start.elapsed().as_micros() as f32/1000.0)
}
