use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("input.txt");

    let data = lines.into_iter()
        .map(|s| s.chars().map(|c| match c {
            '0' => false,
            '1' => true,
            c => panic!("Expected '0' or '1', got '{}'", c),
        }).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    
    let most_common_bits = get_most_common_bits(&data);
    let least_common_bits = get_least_common_bits(&most_common_bits);
    
    // Part 1 does not specify what to do in an equal number of bits at one 
    // position, so we assume it does not occur.
    let most_common_bits_unwrapped = most_common_bits.iter()
        .map(|v| v.unwrap())
        .collect::<Vec<_>>();
    let least_common_bits_unwrapped = least_common_bits.iter()
        .map(|v| v.unwrap())
        .collect::<Vec<_>>();
    
    let gamma_rate = to_decimal(&most_common_bits_unwrapped);
    let epsilon_rate = to_decimal(&least_common_bits_unwrapped);

    println!("Power consumption: {}", gamma_rate * epsilon_rate);

    let mut ogr_options: Vec<Vec<bool>> = data.clone();
    let mut i = 0;
    while ogr_options.len() > 1 {
        let msb = get_most_common_bits(&ogr_options);

        // We're assuming i cannot exceed the number of bits per line of the 
        // input.
        ogr_options = ogr_options.into_iter()
            .filter(|str| match msb[i] {
                Some(b) if b == str[i] => true,
                None => str[i] == true,
                _ => false,
            })
            .collect();

        i += 1;
    }

    let mut co2_options: Vec<Vec<bool>> = data.clone();
    let mut i = 0;
    while co2_options.len() > 1 {
        let mcb = get_most_common_bits(&co2_options);
        let lcb = get_least_common_bits(&mcb);

        // We're assuming i cannot exceed the number of bits per line of the 
        // input.
        co2_options = co2_options.into_iter()
            .filter(|str| match lcb[i] {
                Some(b) if b == str[i] => true,
                None => str[i] == false,
                _ => false,
            })
            .collect();

        i += 1;
    }

    let oxygen_generator_rating = to_decimal(&ogr_options[0]);
    let co2_scrubber_rating = to_decimal(&co2_options[0]);

    println!("Life support rating: {}", oxygen_generator_rating * co2_scrubber_rating);
}

fn get_most_common_bits(data: &Vec<Vec<bool>>) -> Vec<Option<bool>> {
    let num_data_points = data[0].len();

    // For each position i, holds the number of 1 bits at position i over all 
    // the lines.
    let counts = data.iter()
        .fold(vec![0; num_data_points], |mut counts, chars| {
            chars.iter()
                .enumerate()
                .filter(|(_, &b)| b)
                .for_each(|(i, _)| {
                    counts[i] += 1;
                });

            counts
        });

    let half_len = data.len() / 2;
    counts.iter()
        // Because half_len is rounded down, in the case where there are an
        // odd number of data points, 2 * half_len does not have to equal
        // the length of the data.
        .map(|&count| if 2 * count == data.len() {
            None
        } else {
            Some(count > half_len)
        })
        .collect::<Vec<_>>()
}

fn get_least_common_bits(most_common_bits: &Vec<Option<bool>>) -> Vec<Option<bool>> {
    most_common_bits.iter()
        .map(|&v| match v {
            Some(b) => Some(!b),
            None => None,
        })
        .collect::<Vec<_>>()
}

fn to_decimal<'a>(number: &Vec<bool>) -> u32 {
    number.iter()
        .rev()
        .enumerate()
        .fold(0u32, |acc, (i, &b)| if b {
            acc + u32::pow(2, i as u32)
        } else {
            acc
        })
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