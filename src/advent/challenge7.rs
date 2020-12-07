use std::fs;

#[derive(Debug)]
pub struct Challenge7 {
    rules: Vec<String>,
}

impl Challenge7 {
    pub fn new() -> Self {
        let file_content = fs::read_to_string("./src/data/puzzle7.data")
            .expect("Something went wrong while reading the file");

        Challenge7 {
            rules: file_content.lines().map(|x| String::from(x)).collect::<Vec<_>>(),
        }
    }

    pub fn part1(&self) -> String {
        String::from("TODO")
    }

    pub fn part2(&self) -> String {
        String::from("TODO")
    }
}
