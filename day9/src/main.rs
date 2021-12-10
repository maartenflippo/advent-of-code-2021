mod grid;

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("input.txt");
    let grid = grid::Grid::new(lines);

    let low_points = grid.cells()
        .filter(|&(row, col)| grid.neighbors(row, col).into_iter()
            .all(|(r, c)| grid.height_at(row, col) < grid.height_at(r, c)))
        .collect::<Vec<_>>();

    let sum_risk_levels = low_points.iter()
        .map(|&(row, col)| grid.height_at(row, col) + 1)
        .sum::<u32>();

    println!("Sum of risk levels: {}", sum_risk_levels);

    let mut visited = low_points.iter()
        .cloned()
        .collect::<HashSet<_>>();

    let mut basin_sizes = low_points.iter()
        .map(|&(row, col)| {
            1 + grid.neighbors(row, col).into_iter()
                .map(|(r, c)| {
                    get_basin_size(&grid, &mut visited, r, c)
                })
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    basin_sizes.sort();
    let p2_ans = basin_sizes.iter().rev().take(3).product::<u32>();
    println!("Answer p2: {}", p2_ans);
}

fn get_basin_size(
    grid: &grid::Grid,
    visited: &mut HashSet<(usize, usize)>,
    row: usize,
    col: usize,
) -> u32 {
    let neighbors = grid.neighbors(row, col);

    // A point is in the basin, if among its visited neighbors at least one
    // of them is at most as heigh as this one, and the current height is less
    // than 9.
    let in_basin = grid.height_at(row, col) < 9 && neighbors.iter()
        .filter(|&p| visited.contains(p))
        .any(|&(r, c)| grid.height_at(r, c) <= grid.height_at(row, col));

    if !in_basin || visited.contains(&(row, col)) {
        return 0;
    }

    visited.insert((row, col));

    1 + neighbors.into_iter()
        .map(|(r, c)| get_basin_size(grid, visited, r, c))
        .sum::<u32>()
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
