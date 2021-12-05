mod board;

use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use board::*;

fn main() {
    let lines = read_lines("input.txt");
    
    let mut moves = lines[0].split(',')
        .map(|num| num.parse::<u32>().expect("Failed to parse number."))
        .collect::<VecDeque<_>>();

    let mut line = 2;
    let mut boards: Vec<Board> = vec![];
    while line < lines.len() {
        boards.push(Board::new(&lines[line..]));
        line += BOARD_SIZE + 1;
    }

    let mut last_winner_score: Option<u32> = None;
    while let Some(number) = moves.pop_front() {
        for board in boards.iter_mut() {
            if let Some(score) = board.try_complete(number) {
                if last_winner_score.is_none() {
                    println!("First winner: {}", score);
                }

                last_winner_score = Some(score);
            }
        }
    }

    println!("Last winner: {}", last_winner_score.unwrap());
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