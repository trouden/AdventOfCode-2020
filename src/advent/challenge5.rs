use std::collections::BTreeSet;
use std::fs;

#[derive(Debug)]
pub struct Challenge5 {
    boarding_passes: Vec<String>,
}

impl Challenge5 {
    pub fn new() -> Self {
        let file_content = fs::read_to_string("./src/data/puzzle5.data")
            .expect("Something went wrong while reading the file");

        Challenge5 {
            boarding_passes: file_content
                .lines()
                .map(|x| String::from(x))
                .collect::<Vec<_>>(),
        }
    }

    pub fn part1(&self) -> String {
        self.boarding_passes
            .iter()
            .map(|x| calculate_seat_id(x).unwrap())
            .max()
            .unwrap()
            .to_string()
    }

    pub fn part2(&self) -> String {
        let highest_seat_id: u16 = 0b1111111111;

        let existing_seat_ids: BTreeSet<u16> = self
            .boarding_passes
            .iter()
            .map(|x| calculate_seat_id(x).unwrap())
            .collect();

        let possible_seat_ids: BTreeSet<u16> = (0..highest_seat_id).collect();

        let difference: BTreeSet<_> = possible_seat_ids.difference(&existing_seat_ids).collect();

        for elem in &difference {
            if *(*elem) == 0 {
                continue;
            }

            if existing_seat_ids.contains(&(*elem - 1)) && existing_seat_ids.contains(&(*elem + 1))
            {
                return (*elem).to_string();
            }
        }

        String::from("Result not found")
    }
}

fn calculate_seat_id(boarding_id: &str) -> Option<u16> {
    let mut seat_id: u16 = 0;
    let length: usize = 9;

    for (pos, character) in boarding_id.chars().enumerate() {
        if character == 'B' || character == 'R' {
            seat_id |= 1 << (length - pos);
        }
    }

    Some(seat_id)
}
