use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct InvalidChunkDelimiter(char);

#[derive(PartialEq, Eq)]
enum ChunkDelimiterType {
    Brace,
    Bracket,
    Parenthesis,
    Angled
}

enum ChunkDelimiter {
    Open(ChunkDelimiterType),
    Close(ChunkDelimiterType),
}

impl From<ChunkDelimiter> for char {
    fn from(value: ChunkDelimiter) -> Self {
        match value {
            ChunkDelimiter::Open(ChunkDelimiterType::Parenthesis) => '(',
            ChunkDelimiter::Close(ChunkDelimiterType::Parenthesis) => ')',
            ChunkDelimiter::Open(ChunkDelimiterType::Brace) => '{',
            ChunkDelimiter::Close(ChunkDelimiterType::Brace) => '}',
            ChunkDelimiter::Open(ChunkDelimiterType::Bracket) => '[',
            ChunkDelimiter::Close(ChunkDelimiterType::Bracket) => ']',
            ChunkDelimiter::Open(ChunkDelimiterType::Angled) => '<',
            ChunkDelimiter::Close(ChunkDelimiterType::Angled) => '>',
        }
    }
}

impl TryFrom<char> for ChunkDelimiter {
    type Error = InvalidChunkDelimiter;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Self::Open(ChunkDelimiterType::Parenthesis)),
            ')' => Ok(Self::Close(ChunkDelimiterType::Parenthesis)),
            '{' => Ok(Self::Open(ChunkDelimiterType::Brace)),
            '}' => Ok(Self::Close(ChunkDelimiterType::Brace)),
            '[' => Ok(Self::Open(ChunkDelimiterType::Bracket)),
            ']' => Ok(Self::Close(ChunkDelimiterType::Bracket)),
            '<' => Ok(Self::Open(ChunkDelimiterType::Angled)),
            '>' => Ok(Self::Close(ChunkDelimiterType::Angled)),
            c => Err(InvalidChunkDelimiter(c)),
        }
    }
}

fn get_first_illegal_character(line: &str) -> Option<ChunkDelimiterType> {
    let mut stack = Vec::new();

    for c in line.chars() {
        match ChunkDelimiter::try_from(c) {
            Ok(ChunkDelimiter::Open(delimiter_type)) => {
                stack.push(delimiter_type);
            }
            Ok(ChunkDelimiter::Close(delimiter_type))
                if stack.last()
                    .map(|dl| dl == &delimiter_type)
                    .unwrap_or(false)
            => {
                stack.pop();
            }
            Ok(ChunkDelimiter::Close(dl)) => return Some(dl),
            _ => panic!("Should never happen."),
        }
    }

    None
}

fn get_missing_closing_delimiters(line: &str) -> String {
    let mut stack = Vec::new();

    for c in line.chars() {
        match ChunkDelimiter::try_from(c) {
            Ok(ChunkDelimiter::Open(delimiter_type)) => {
                stack.push(delimiter_type);
            }
            Ok(ChunkDelimiter::Close(delimiter_type))
                if stack.last()
                    .map(|dl| dl == &delimiter_type)
                    .unwrap_or(false)
            => {
                stack.pop();
            }
            _ => panic!("Should never happen."),
        }
    }

    stack.into_iter()
        .map(|dl| char::from(ChunkDelimiter::Close(dl)))
        .rev()
        .collect::<String>()
}

fn main() {
    let lines = read_lines("input.txt");

    let syntax_error_score = lines.iter()
        .filter_map(|l| get_first_illegal_character(l))
        .map(|dl| match dl {
            ChunkDelimiterType::Parenthesis => 3,
            ChunkDelimiterType::Bracket => 57,
            ChunkDelimiterType::Brace => 1197,
            ChunkDelimiterType::Angled => 25137,
        })
        .sum::<usize>();

    println!("Syntax error score: {}", syntax_error_score);

    let mut autocomplete_scores = lines.iter()
        .filter(|l| get_first_illegal_character(l).is_none())
        .map(|l| get_missing_closing_delimiters(l))
        .map(|str| str.chars().fold(0u64, |acc, c| {
            let add = match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("Unexpected char"),
            };

            acc * 5 + add
        }))
        .collect::<Vec<_>>();

    autocomplete_scores.sort();

    println!("Middle autocomplete score: {}", autocomplete_scores[autocomplete_scores.len() / 2]);
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
