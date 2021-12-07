#![feature(int_abs_diff)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn p1_fuel_cost(start: u32, end: u32) -> u32 {
    start.abs_diff(end)
}

fn p2_fuel_cost(start: u32, end: u32) -> u32 {
    let diff = start.abs_diff(end);

    (diff + 1) * diff / 2
}

fn main() {
    let numbers = &read_lines("input.txt")[0];
    let numbers = numbers.split(',')
        .map(|num| num.parse::<u32>().expect("Failed to parse number."))
        .collect::<Vec<_>>();

    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();


    let p1 = (min..=max)
        .map(|p| numbers.iter().map(|&n| p1_fuel_cost(p, n)).sum::<u32>())
        .min()
        .unwrap();

    let p2 = (min..=max)
        .map(|p| numbers.iter().map(|&n| p2_fuel_cost(p, n)).sum::<u32>())
        .min()
        .unwrap();

    println!("Globally closest (part 1): {}", p1);
    println!("Globally closest (part 2): {}", p2);
}

// The output is wrapped in a Result to allow matching on errors
// Returns a vector of the lines of the file.
//
// Based on:
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let file = File::open(filename).expect("Failed to read file.");

    io::BufReader::new(file)
        .lines()
        .into_iter()
        .map(|l| l.expect("Failed to read line."))
        .collect()
}
