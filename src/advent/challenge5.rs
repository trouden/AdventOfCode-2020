use std::collections::BTreeSet;
use std::fs;

const ROWS: u8 = 128;
const COLUMNS: u8 = 8;

#[derive(Debug)]
pub struct Challenge5 {
    boarding_passes: Vec<String>,
}

impl Challenge5 {
    pub fn new() -> Self {
        let file_content = fs::read_to_string("./src/Data/puzzle5.data")
            .expect("Something went wrong while reading the file");

        Challenge5 {
            boarding_passes: file_content
                .lines()
                .map(|x| String::from(x))
                .collect::<Vec<_>>(),
        }
    }

    pub fn part1(&self) -> String {
        let mut highest_seat_id: u16 = 0;

        for elem in &self.boarding_passes {
            let seating_id = calculate_seat_id(elem).unwrap();

            if seating_id > highest_seat_id {
                highest_seat_id = seating_id;
            }
        }

        highest_seat_id.to_string()
    }

    pub fn part2(&self) -> String {      
        let highest_seat_id = (ROWS - 1) as u16 * 8 + (COLUMNS - 1) as u16;

        let mut existing_seat_ids: BTreeSet<u16> = BTreeSet::new();

        for elem in &self.boarding_passes {
            let seating_id = calculate_seat_id(elem).unwrap();
            existing_seat_ids.insert(seating_id);
        }

        let possible_seat_ids: BTreeSet<u16> = (0..highest_seat_id).collect();

        let difference: BTreeSet<_> = possible_seat_ids.difference(&existing_seat_ids).collect();

        for elem in &difference {
            if *(*elem) == 0 {
                continue;
            }

            if existing_seat_ids.contains(&(*elem-1)) && existing_seat_ids.contains(&(*elem+1)) {
                return (*elem).to_string();
            }
        }

        String::from("Result not found")
    }
}

fn calculate_seat_id(boarding_id: &str) -> Option<u16> {
    let mut row_lower: u8 = 0;
    let mut row_upper: u8 = ROWS - 1;

    let mut column_lower: u8 = 0;
    let mut column_upper: u8 = COLUMNS - 1;

    for column in (&boarding_id[..7]).chars() {
        let half = (row_upper - row_lower + 1) / 2;

        if column == 'F' {
            row_upper -= half;
        } else if column == 'B' {
            row_lower += half;
        } else {
            return None;
        }
    }

    if row_lower != row_upper {
        return None;
    }

    for row in (&boarding_id[7..]).chars() {
        let half = (column_upper - column_lower + 1) / 2;

        if row == 'L' {
            column_upper -= half;
        } else if row == 'R' {
            column_lower += half;
        } else {
            return None;
        }
    }

    if column_lower != column_upper {
        return None;
    }

    Some(row_lower as u16 * 8 + column_lower as u16)
}
