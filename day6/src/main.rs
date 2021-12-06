use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    const DAYS: usize = 256;
    const CYCLE: usize = 8;
    let line = &read_lines("input.txt")[0];

    let fish = line.split(',')
        .map(|num| num.parse::<u64>().expect("Failed to parse number."))
        .collect::<Vec<_>>();

    let mut bins = vec![0u64; CYCLE + 1];
    fish.iter().for_each(|&f| bins[f as usize] += 1);
    
    for _ in 0..DAYS {
        let created_fish = bins[0];

        for i in 0..CYCLE {
            bins[i] = bins[i + 1];
        }

        bins[6] += created_fish;
        bins[CYCLE] = created_fish;
    }

    println!("Number of fish after {} days: {}", DAYS, bins.iter().sum::<u64>());
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
