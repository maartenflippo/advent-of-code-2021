use std::{collections::HashMap, fmt::Debug};

pub const BOARD_SIZE: usize = 5;

pub struct Board {
    completed: bool,
    marked_in_column: [usize; BOARD_SIZE],
    marked_in_row: [usize; BOARD_SIZE],
    unmarked_numbers: HashMap<u32, (usize, usize)>,
    marked_numbers: HashMap<u32, (usize, usize)>,
}

impl Board {
    pub fn new(input: &[String]) -> Self {
        let mut numbers = HashMap::with_capacity(BOARD_SIZE * BOARD_SIZE);

        for line in 0..BOARD_SIZE {
            input[line].split(' ')
                .filter(|num| num.len() != 0) // Double spaces in the input file.
                .map(|num| num.parse::<u32>().expect("Failed to parse number."))
                .enumerate()
                .for_each(|(col, num)| {
                    numbers.insert(num, (line, col));
                });
        }

        assert!(numbers.len() == BOARD_SIZE * BOARD_SIZE, "Board size was {}", numbers.len());

        Self {
            completed: false,
            marked_in_column: [0; BOARD_SIZE],
            marked_in_row: [0; BOARD_SIZE],
            unmarked_numbers: numbers,
            marked_numbers: HashMap::with_capacity(BOARD_SIZE * BOARD_SIZE),
        }
    }

    /// Mark the given number on this board. Returns Some(score) if the board is
    /// completed, None otherwise.
    pub fn try_complete(&mut self, number: u32) -> Option<u32> {
        if self.completed {
            return None;
        }
        
        let map_entry = self.unmarked_numbers.remove(&number);

        match map_entry {
            Some((row, col)) => {
                self.marked_in_column[col] += 1;
                self.marked_in_row[row] += 1;
                self.marked_numbers.insert(number, (row, col));

                if self.marked_in_column[col] == BOARD_SIZE || self.marked_in_row[row] == BOARD_SIZE {
                    self.completed = true;

                    let unmarked_sum = self.unmarked_numbers.keys()
                        .cloned()
                        .sum::<u32>();

                    let score = unmarked_sum * number;
                    Some(score)
                } else {
                    
                    None
                }
            },
            None => None,
        }
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let numbers = self.unmarked_numbers.iter()
            .chain(self.marked_numbers.iter())
            .map(|(k, v)| (v, k))
            .collect::<HashMap<_,_>>();

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let number = numbers[&(row, col)];

                if self.unmarked_numbers.contains_key(number) {
                    write!(f, " {:02}  ", number)?;
                } else {
                    write!(f, "|{:02}| ", number)?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}