use std::fs;
use std::{cmp::Ordering, collections::VecDeque};

const PREAMBLE_LENGTH: usize = 25;

#[derive(Debug)]
pub struct Challenge9 {
    preamble: VecDeque<u64>,
    numbers: Vec<u64>,
}

impl Challenge9 {
    pub fn new() -> Self {
        let file_content = fs::read_to_string("./src/data/puzzle9.data")
            .expect("Something went wrong while reading the file");

        let all_numbers = file_content
            .lines()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        Challenge9 {
            preamble: all_numbers[..PREAMBLE_LENGTH].iter().copied().collect(),
            numbers: all_numbers,
        }
    }

    pub fn part1(&self) -> String {
        match self.find_invalid_number() {
            None => String::from("Could not find invalid number."),
            Some(n) => n.to_string(),
        }
    }

    pub fn part2(&self) -> String {
        let invalid_number = self.find_invalid_number().unwrap();

        for i in 0..self.numbers.len() - 1 {
            let start = self.numbers[i];

            if start >= invalid_number {
                continue;
            }

            let mut j: usize = 1;
            let mut number_set: Vec<u64> = vec![start];

            while (i + j) < self.numbers.len() {
                number_set.push(self.numbers[i + j]);

                if number_set.iter().sum::<u64>() >= invalid_number {
                    break;
                }

                j += 1;
            }

            if number_set.iter().sum::<u64>() == invalid_number {
                number_set.sort();
                return (number_set[0] + number_set[number_set.len() - 1]).to_string();
            }
        }

        String::from("Nothing Found")
    }

    fn find_invalid_number(&self) -> Option<u64> {
        let mut queue: VecDeque<u64> = self.preamble.iter().copied().collect();

        for number in self.numbers.iter().skip(queue.len()) {
            if !queue_contains_2_sum_parts(&queue, number) {
                return Some(*number);
            }

            queue.pop_front();
            queue.push_back(*number);
        }

        None
    }
}

// Instead of checking all possible combinations, copy and sort the queue
fn queue_contains_2_sum_parts(queue: &VecDeque<u64>, number: &u64) -> bool {
    let mut sorted: Vec<_> = queue.iter().cloned().collect();
    sorted.sort();

    let mut i: usize = 0;

    while i < sorted.len() - 1 {
        let part_1 = sorted[i];

        if part_1 >= *number {
            return false;
        }

        for j in i + 1..sorted.len() {
            match (part_1 + sorted[j]).cmp(number) {
                Ordering::Less => continue,
                Ordering::Greater => break,
                Ordering::Equal => return true,
            }
        }

        i += 1;
    }

    false
}
