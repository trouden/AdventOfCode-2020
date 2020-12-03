use std::fs;

#[derive(Debug)]
pub struct Challenge3 {
    map: Vec<Vec<char>>,
}

impl Challenge3 {
    pub fn new() -> Challenge3 {
        let file_content = fs::read_to_string("./src/Data/puzzle3.data")
            .expect("Something went wrong while reading the file");

        let mut challenge = Challenge3 { map: Vec::new() };

        for elem in file_content.lines().map(|s| s.chars().collect()) {
            challenge.map.push(elem)
        }

        challenge
    }

    pub fn part1(&self) -> String {
        match self.trees_on_slope_counter(1, 3) {
            None => return String::from("Something went wrong"),
            Some(trees) => return trees.to_string(),
        }
    }

    pub fn part2(&self) -> String {
        // Don't bother doing match flows...
        let test_1 = self.trees_on_slope_counter(1, 1).unwrap();
        let test_2 = self.trees_on_slope_counter(1, 3).unwrap();
        let test_3 = self.trees_on_slope_counter(1, 5).unwrap();
        let test_4 = self.trees_on_slope_counter(1, 7).unwrap();
        let test_5 = self.trees_on_slope_counter(2, 1).unwrap();

        // Max value of u8 is 255, so to multiply 5 u8 we need at least an u64
        (test_1 as u64 * test_2 as u64 * test_3 as u64 * test_4 as u64 * test_5 as u64).to_string()
    }

    fn trees_on_slope_counter(&self, vertical: usize, horizontal: usize) -> Option<u8> {
        let height = self.map.len();

        if self.map.len() == 0 {
            return None;
        }

        if (&self.map[0]).len() == 0 {
            return None;
        }

        if vertical >= height {
            return None;
        }

        let mut x: usize = 0;
        let mut y: usize = 0;

        let mut tree_count: u8 = 0;

        while y < height {
            let row = &self.map[y];

            x = x % row.len();

            if row[x] == '#' {
                tree_count += 1;
            }

            y += vertical;
            x += horizontal;
        }

        Some(tree_count)
    }
}
