use std::collections::BTreeMap;
use std::fs;

#[derive(Debug)]
pub struct Challenge4 {
    passport_batch: Vec<BTreeMap<String, String>>,
}

impl Challenge4 {
    pub fn new() -> Challenge4 {
        let file_content = fs::read_to_string("./src/data/puzzle4.data")
            .expect("Something went wrong while reading the file");

        let mut challenge = Challenge4 {
            passport_batch: Vec::new(),
        };

        for elem in file_content.split("\n\n").collect::<Vec<_>>() {
            let parts = elem.split(&['\n', ' '][..]).collect::<Vec<_>>();

            let mut hash_map = BTreeMap::new();

            for kv_non_split in parts {
                let kv = kv_non_split.split(':').collect::<Vec<_>>();

                if kv.len() != 2 {
                    println!("Invalid kv");
                    continue;
                }

                hash_map.insert(String::from(kv[0]), String::from(kv[1]));
            }

            challenge.passport_batch.push(hash_map)
        }

        challenge
    }

    pub fn part1(&self) -> String {
        let mut valid_passports: u8 = 0;

        let required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

        for passport in &self.passport_batch {
            let mut has_required_keys = true;

            for key in required_keys.iter() {
                match passport.get(*key) {
                    None => has_required_keys = false,
                    Some(_) => continue,
                }

                if !has_required_keys {
                    break;
                }
            }

            if has_required_keys {
                valid_passports += 1;
            }
        }

        valid_passports.to_string()
    }

    pub fn part2(&self) -> String {
        let mut valid_passports: u8 = 0;

        let eye_colours = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        for passport in &self.passport_batch {
            match try_get_i32_from_hash_map(passport, "byr") {
                None => continue,
                Some(v) => {
                    if !(v >= 1920 && v <= 2002) {
                        continue;
                    }
                }
            }

            match try_get_i32_from_hash_map(passport, "iyr") {
                None => continue,
                Some(v) => {
                    if !(v >= 2010 && v <= 2020) {
                        continue;
                    }
                }
            }

            match try_get_i32_from_hash_map(passport, "eyr") {
                None => continue,
                Some(v) => {
                    if !(v >= 2020 && v <= 2030) {
                        continue;
                    }
                }
            }

            match passport.get("hgt") {
                None => continue,
                Some(v) => {
                    let mut validated = false;

                    if v.ends_with("cm") {
                        match (&v[..v.len() - 2]).parse::<i32>() {
                            Err(_) => continue,
                            Ok(pv) => {
                                if pv >= 150 && pv <= 193 {
                                    validated = true;
                                } else {
                                    continue;
                                }
                            }
                        }
                    }

                    if !validated || v.ends_with("in") {
                        match (&v[..v.len() - 2]).parse::<i32>() {
                            Err(_) => continue,
                            Ok(pv) => {
                                if pv >= 59 && pv <= 76 {
                                    validated = true;
                                } else {
                                    continue;
                                }
                            }
                        }
                    }

                    if !validated {
                        continue;
                    }
                }
            }

            match passport.get("hcl") {
                None => continue,
                Some(v) => {
                    if !v.starts_with('#') {
                        continue;
                    }

                    let hex_code = &v[1..];

                    if hex_code.len() != 6 {
                        if !hex_code.chars().all(|x| x.is_ascii_hexdigit()) {
                            continue;
                        }
                    }
                }
            }

            match passport.get("ecl") {
                None => continue,
                Some(v) => {
                    if !eye_colours.iter().any(|x| x == v) {
                        continue;
                    }
                }
            }

            match passport.get("pid") {
                None => continue,
                Some(v) => {
                    if v.len() != 9 {
                        continue;
                    }

                    match v.parse::<i32>() {
                        Err(_) => continue,
                        Ok(_) => (),
                    }
                }
            }

            valid_passports += 1;
        }

        valid_passports.to_string()
    }
}

fn try_get_i32_from_hash_map(map: &BTreeMap<String, String>, key: &str) -> Option<i32> {
    match map.get(key) {
        None => None,
        Some(v) => match v.parse::<i32>() {
            Err(_) => None,
            Ok(pv) => Some(pv),
        },
    }
}
