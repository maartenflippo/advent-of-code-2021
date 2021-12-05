#![feature(int_abs_diff)]

mod line;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use line::*;

fn main() {
    let lines = read_lines("input.txt");

    let lines: Vec<Line> = lines.into_iter()
        .map(|l| l.as_str().into())
        .collect();

    let x_max = lines.iter()
        .map(|l| if l.start.x > l.end.x { l.start } else { l.end })
        .map(|p| p.x)
        .max()
        .unwrap();

    let y_max = lines.iter()
        .map(|l| if l.start.y > l.end.y { l.start } else { l.end })
        .map(|p| p.y)
        .max()
        .unwrap();

    let seen_twice = (0..=y_max).into_iter()
        .flat_map(|row| (0..=x_max).into_iter().map(move |col| (row, col)))
        .map(|p| p.into())
        .map(|p| lines.iter()
            .filter(|l| l.covers(p))
            .count())
        .filter(|&count| count >= 2)
        .count();

    println!("Number of points covered twice: {}", seen_twice);
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
