use std::fs;

#[derive(Debug)]
pub struct Challenge2 {
    passwords: Vec<(usize, usize, char, String)>,
}

impl Challenge2 {

    pub fn new() -> Challenge2 {
        let file_content = fs::read_to_string("./src/Data/puzzle2.data")
            .expect("Something went wrong while reading the file");

        let mut challenge = Challenge2 {
            passwords: Vec::new(),
        };

        let password_with_policy = file_content.split("\n").map(|x| {
            let split = x.split(":").collect::<Vec<_>>();

            if split.len() != 2 {
                panic!("Invalid input")
            }
            let password = split[1].trim();
            let part1 = split[0].split(" ").collect::<Vec<_>>();
            if part1.len() != 2 {
                panic!("Invalid input")
            }

            let character = part1[1];
            let numbers = part1[0].split("-").collect::<Vec<_>>();

            if numbers.len() != 2 {
                panic!("Invalid input, expect 2 numbers.")
            }

            (numbers[0].parse::<usize>().unwrap(),  numbers[1].parse::<usize>().unwrap(), character.parse::<char>().unwrap(), String::from(password))
        });
        
        for elem in password_with_policy {
            challenge.passwords.push(elem)
        }

        challenge
    }

    pub fn part1(&self) -> String {
        let mut valid_passwords = 0;

        for elem in self.passwords.iter() {
            let (_lower_bound, _upper_bound, _character, _password) = elem;

            let char_count = _password.matches(*_character).count();

            if char_count >= *_lower_bound && char_count <= *_upper_bound {
                valid_passwords = valid_passwords + 1;
            }
        }

        valid_passwords.to_string()
    }

    pub fn part2(&self) -> String {
        let mut valid_passwords = 0;

        for elem in self.passwords.iter() {
            let (_index1, _index2, _character, _password) = elem;

            let char_at_index1 = _password.as_bytes()[*_index1 - 1] as char;
            let char_at_index2 = _password.as_bytes()[*_index2 - 1] as char;

            if (char_at_index1 == *_character) ^ (char_at_index2 == *_character) {
                valid_passwords = valid_passwords + 1;
            }
        }

        valid_passwords.to_string()
    }
}