use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn process_lines<B>(lines: &[String], init: B, acc: impl Fn(B, (&str, u32)) -> B) -> B {
    lines
        .iter()
        .map(|s| s.split(' ').collect::<Vec<&str>>())
        .map(|split| (split[0], split[1].parse::<u32>().expect("Failed to parse u32")))
        .fold(init, acc)
}

fn main() {
    let lines = read_lines("input.txt");

    let (horizontal, depth) = process_lines(&lines, (0, 0), |(horizontal, depth), (axis, amount)| match axis {
        "forward" => (horizontal + amount, depth),
        "up" => (horizontal, depth - amount),
        "down" => (horizontal, depth + amount),
        other => panic!("Unknown axis: {}", other),
    });
    println!("Part 1: {}", horizontal * depth);

    let (horizontal, depth, _) = process_lines(&lines, (0, 0, 0), |(horizontal, depth, aim), (axis, amount)| match axis {
        "forward" => (horizontal + amount, depth + aim * amount, aim),
        "up" => (horizontal, depth, aim - amount),
        "down" => (horizontal, depth, aim + amount),
        other => panic!("Unknown axis: {}", other),
    });
    println!("Part 2: {}", horizontal * depth);
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