use std::{collections::HashSet, fs};

#[derive(Debug)]
pub struct Challenge6 {
    customs_forms: Vec<Vec<Vec<char>>>,
}

impl Challenge6 {
    pub fn new() -> Self {
        let file_content = fs::read_to_string("./src/data/puzzle6.data")
            .expect("Something went wrong while reading the file");

        Challenge6 {
            customs_forms: file_content
                .split("\n\n")
                .map(|x| x.lines().map(|y| y.chars().collect()).collect())
                .collect::<Vec<_>>(),
        }
    }

    pub fn part1(&self) -> String {
        let mut count: usize = 0;

        for group in &self.customs_forms {
            let mut hash_set: HashSet<char> = HashSet::new();

            for person in group {
                for answer in person {
                    hash_set.insert(*answer);
                }
            }

            count += hash_set.len();
        }

        count.to_string()
    }

    pub fn part2(&self) -> String {
        let mut count: usize = 0;

        for group in &self.customs_forms {
            let mut hash_set: HashSet<char> = HashSet::new();

            let mut first = true;

            for person in group {
                if first {
                    for answer in person {
                        hash_set.insert(*answer);
                    }
                    first = false;
                } else {
                    let person_set: HashSet<_> = person.iter().cloned().collect();
                    hash_set = hash_set.intersection(&person_set).cloned().collect();
                }
            }

            count += hash_set.len();
        }

        count.to_string()
    }
}
