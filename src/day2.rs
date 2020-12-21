use lazy_static::lazy_static;
use regex::{Captures, Regex};

/// the password is in the format:
/// number,-,number,space,letter,colon,space,password
/// ex: 1-3 a: abcde
/// which means a valid password must have 1-3 as inside it
pub fn part1(password_lines: &[String]) -> u32 {

    let mut valid_passwords = Vec::new();

    for password_line in password_lines.iter() {
        // get the password components
        let (min, max, c, password) = parse_password_line_parts(password_line);

        // now lets see how many times the character appears in the password
        let num_appear: i32 = password.chars().map(|curr_char| {
            if curr_char == c {
                return 1
            }
            0
        }).sum();

        if min <= num_appear && num_appear <= max {
            valid_passwords.push(password_line.clone());
        }
    }

    valid_passwords.len() as u32
}

fn parse_password_line_parts(password_line: &str) -> (i32, i32, char, &str) {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    }

    let cap: Captures = REGEX.captures(password_line).unwrap();

    let min = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let max = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let char = cap.get(3).unwrap().as_str().parse::<char>().unwrap();
    let password = cap.get(4).unwrap().as_str();

    (min, max, char, password)
}

pub fn part2(password_lines: &[String]) -> u32 {
    let mut valid_passwords = Vec::new();

    for password_line in password_lines.iter() {
        // get the password components
        let (first_position, second_position, c, password) = parse_password_line_parts(password_line);

        let mut num_matches = 0;

        // check if the first position matches!
        if password.chars().nth((first_position - 1) as usize).unwrap() == c {
            // yep!
            num_matches += 1;
        }

        // check if the second position matches!
        if password.chars().nth((second_position - 1) as usize).unwrap() == c {
            // yep!
            num_matches += 1;
        }

        // only count it if there was only a single match!
        if num_matches == 1 {
            valid_passwords.push(password_line.clone());
        }

        // otherwise not valid :<
    }

    valid_passwords.len() as u32
}
