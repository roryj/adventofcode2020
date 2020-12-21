use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

use maplit::hashmap;

#[derive(Debug)]
struct Passport {
    birth_year: Option<u16>,
    issue_year: Option<u16>,
    expiration_year: Option<u16>,
    height: Option<String>,
    hair_colour: Option<String>,
    eye_colour: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn new(lines: &[String]) -> Self {
        let mut passport_fields: HashMap<&str, &str> = HashMap::new();

        for line in lines.iter() {
            line.split_whitespace()
                .map(|s| s.split_at(s.find(":").unwrap()))
                .for_each(|(k, v)| {
                    assert!(passport_fields.insert(k, &v[1..]).is_none());
                });
        }

        Passport {
            birth_year: passport_fields
                .get("byr")
                .map(|y| y.parse::<u16>().unwrap()),
            issue_year: passport_fields
                .get("iyr")
                .map(|y| y.parse::<u16>().unwrap()),
            expiration_year: passport_fields
                .get("eyr")
                .map(|y| y.parse::<u16>().unwrap()),
            height: passport_fields.get("hgt").map(|s| s.to_string()),
            hair_colour: passport_fields.get("hcl").map(|s| s.to_string()),
            eye_colour: passport_fields.get("ecl").map(|s| s.to_string()),
            passport_id: passport_fields.get("pid").map(|s| s.to_string()),
            country_id: passport_fields.get("cid").map(|s| s.to_string()),
        }
    }

    fn validate(&self) -> bool {
        if self.birth_year.is_none() {
            return false;
        }

        if self.issue_year.is_none() {
            return false;
        }

        if self.expiration_year.is_none() {
            return false;
        }

        if self.height.is_none() {
            return false;
        }

        if self.hair_colour.is_none() {
            return false;
        }

        if self.eye_colour.is_none() {
            return false;
        }

        if self.passport_id.is_none() {
            return false;
        }

        // ignore for now
        // if self.country_id.is_none() {
        //     return false
        // }

        true
    }

    fn validate_strict(&self) -> bool {
        if !self.validate() {
            return false;
        }

        // we know that all the fields are set, now to validate

        if self.birth_year.unwrap() < 1920 || self.birth_year.unwrap() > 2002 {
            return false;
        }

        if self.issue_year.unwrap() < 2010 || self.issue_year.unwrap() > 2020 {
            return false;
        }

        if self.expiration_year.unwrap() < 2020 || self.expiration_year.unwrap() > 2030 {
            return false;
        }

        // height
        // check if cm or in present
        let h = self.height.as_ref().unwrap().clone();
        let valid_height_range = hashmap! {
            "cm" => (150, 193),
            "in" => (59, 76),
        };

        let mut height_type = "";
        let mut height_value = 0;

        if let Some(i) = h.find("cm") {
            height_type = "cm";
            height_value = h[..(i)].parse::<u32>().unwrap();
        }

        if let Some(i) = h.find("in") {
            height_type = "in";
            height_value = h[..(i)].parse::<u32>().unwrap();
        }

        if height_type.is_empty() {
            return false;
        }

        let (min, max) = valid_height_range[height_type];
        if height_value < min || height_value > max {
            return false;
        }

        // hair colour
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"#[0-9a-f]{6}$").unwrap();
        }
        if !REGEX.is_match(self.hair_colour.as_ref().unwrap()) {
            return false;
        }

        // eye colour
        let ecl = self.eye_colour.as_ref().unwrap();
        let valid_eye_colours = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if !valid_eye_colours.contains(&ecl.as_str()) {
            return false;
        }

        if self.passport_id.as_ref().unwrap().len() != 9 {
            return false;
        }

        true
    }
}

pub fn part1(input: &[String]) -> u32 {
    // parse for spaces and new lines
    // if a line is empty, end of passport and validate it

    let mut passport_lines = Vec::new();
    let mut valid_passport_count = 0;

    for line in input.iter() {
        if line.trim().is_empty() {
            let passport = Passport::new(&passport_lines);

            // println!("{:?}", passport);

            if passport.validate() {
                valid_passport_count += 1;
            }

            passport_lines.clear();
        }

        passport_lines.push(line.to_owned());
    }

    valid_passport_count
}

pub fn part2(input: &[String]) -> u32 {
    // parse for spaces and new lines
    // if a line is empty, end of passport and validate it

    let mut passport_lines = Vec::new();
    let mut valid_passport_count = 0;

    for line in input.iter() {
        if line.trim().is_empty() {
            let passport = Passport::new(&passport_lines);

            if passport.validate_strict() {
                valid_passport_count += 1;
            }

            passport_lines.clear();
        }

        passport_lines.push(line.to_owned());
    }

    valid_passport_count
}
