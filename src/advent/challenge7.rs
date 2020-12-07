use std::{collections::HashMap, fs};

#[derive(Debug)]
pub struct Challenge7 {
    rules: HashMap<String, Vec<(String, u8)>>,
}

impl Challenge7 {
    pub fn new() -> Self {
        let file_content = fs::read_to_string("./src/data/puzzle7.data")
            .expect("Something went wrong while reading the file");

        Challenge7 {
            rules: file_content
                .lines()
                .map(|x| {
                    let original = x.trim_end_matches(".").split("contain").collect::<Vec<_>>();

                    if original.len() != 2 {
                        panic!("Invalid input")
                    }

                    let init_color = original[0].trim().replace(" bags", "");
                    let mut init_content: Vec<(String, u8)> = Vec::new();

                    let contents = original[1].trim().split(", ").collect::<Vec<_>>();

                    if contents.len() == 1 && contents[0].trim() == "no other bags" {
                        return (init_color, init_content);
                    }

                    for content in contents {
                        let count = content
                            .chars()
                            .take_while(|c| c.is_digit(10))
                            .collect::<String>()
                            .parse::<u8>()
                            .unwrap();
                        let content_color = content
                            .chars()
                            .skip_while(|c| c.is_digit(10))
                            .collect::<String>()
                            .trim()
                            .replace(" bags", "")
                            .replace(" bag", "");

                        init_content.push((content_color, count));
                    }

                    (init_color, init_content)
                })
                .collect::<HashMap<_, _>>(),
        }
    }

    pub fn part1(&self) -> String {
        let mut gold_bag_count: u16 = 0;
        let shiny_gold = String::from("shiny gold");

        for (initial_bag, _) in self.rules.iter() {
            match self.get_content(initial_bag) {
                None => (),
                Some(contents) => {
                    if contents.contains(&shiny_gold) {
                        gold_bag_count += 1;
                    }
                }
            }
        }

        gold_bag_count.to_string()
    }

    pub fn part2(&self) -> String {
        String::from("TODO")
    }

    fn get_content(&self, initial_bag: &str) -> Option<Vec<String>> {
        match self.rules.get(initial_bag) {
            None => None,
            Some(contents) => {
                if contents.len() == 0 {
                    return None;
                }

                let mut all_content: Vec<String> = Vec::new();

                for sub_content in contents.iter().map(|x| {
                    let (content, _) = x;
                    String::from(content)
                }) {
                    match self.get_content(&sub_content) {
                        None => (),
                        Some(mut v) => all_content.append(&mut v),
                    }

                    all_content.push(sub_content);
                }

                Some(all_content)
            }
        }
    }
}
