use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn accumulator(pair: (u32, u32), i: u32) -> (u32, u32) {
    let (cnt, prev) = pair;
    if i > prev {
        (cnt + 1, i)
    } else {
        (cnt, i)
    }
}

fn part1(lines: &[u32]) {
    let (inc_count, _) = lines.iter()
        .cloned()
        .fold((0, u32::MAX), accumulator);

    println!("Number of increases: {}", inc_count);
}

fn part2(lines: &[u32]) {
    const WINDOW_SIZE: usize = 3;

    let (inc_count, _) = lines.windows(WINDOW_SIZE)
        .map(|window| window.iter().cloned().sum())
        .fold((0, u32::MAX), accumulator);

    println!(
        "Number of increases with sliding window of size {}: {}", 
        WINDOW_SIZE, 
        inc_count
    );
}

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let lines: Vec<u32> = lines.into_iter()
            .map(|l| l.expect("Failed to read line."))
            .map(|l| l.parse::<u32>()
                .expect("Failed to parse line as u32."))
            .collect();

        part1(&lines);
        part2(&lines);
    } else {
        eprintln!("Failed to read file.");
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
//
// Courtesy of:
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}