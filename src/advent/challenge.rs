use std::fs;

#[derive(Debug)]
pub struct Challenge1 {
    expenses: Vec<i32>
}

impl Challenge1 {
    pub fn new() -> Challenge1 {
        let file_content = fs::read_to_string("./src/Data/puzzle1.data")
            .expect("Something went wrong while reading the file");

        let mut challenge = Challenge1 {
            expenses: Vec::new(),
        };

        for elem in file_content.split("\n") {
            challenge.expenses.push(elem.parse::<i32>().unwrap())
        }

        challenge
    }
    
    pub fn run(&self) -> String {
        let mut index = 0;
        let mut elem1: i32 = 0;
        let mut elem2: i32 = 0;
        let mut found = false;

        while !found {
            if index >= self.expenses.len() {
                break;
            }

            elem1 = self.expenses[index];

            for (i, elem) in self.expenses.iter().enumerate() {
                if i == index {
                    continue;
                }
                if elem1 + *elem == 2020 {
                    elem2 = *elem;
                    found = true;
                    break;
                }
            }

            index = index + 1;
        }

        if found {
            let result = (elem1 * elem2).to_string();
            result
        } else {
            String::from("No result")
        }
    }
}
