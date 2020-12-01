use itertools::Itertools;
use std::fs;

#[derive(Debug)]
pub struct Challenge1 {
    expenses: Vec<i32>,
}

impl Challenge1 {
    pub fn new() -> Challenge1 {
        let file_content = fs::read_to_string("./src/Data/puzzle1.data")
            .expect("Something went wrong while reading the file");

        let mut challenge = Challenge1 {
            expenses: Vec::new(),
        };

        for elem in file_content.split("\n").map(|s| { s.parse::<i32>().unwrap()}).sorted() {
            challenge.expenses.push(elem)
        }

        challenge
    }

    pub fn part1(&self) -> String {
        match get_product_if_sum_equals(&self.expenses, 2, 2020) {
            None => return String::from("No result found"),
            Some(number) => return number.to_string(),
        }
    }

    pub fn part2(&self) -> String {
        match get_product_if_sum_equals(&self.expenses, 3, 2020) {
            None => return String::from("No result found"),
            Some(number) => return number.to_string(),
        }
    }
}

fn get_product_if_sum_equals(set: &[i32], combination_length: usize, required_sum: i32) -> Option<i64> {

    let combinations = set.iter().combinations(combination_length);

    let mut sum = 0;
    let mut product: i64 = 0;

    for combination in combinations {

        for elem in combination {
            sum = sum + *elem;
    
            if product == 0 {
                product = i64::from(*elem);
            } else {
                product = product * i64::from(*elem);
            }
    
        }
    
        if sum == required_sum {
            return Some(product);
        }

        sum = 0;
        product = 0;
    }
    
    None
}