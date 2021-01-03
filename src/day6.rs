use std::collections::HashSet;

pub fn part1(input: &[String]) -> u32 {
    let mut running_sum = 0;

    let mut curr_group_lines = Vec::new();
    for line in input {
        if line.is_empty() {
            running_sum += calculate_unique_yes_answers(&curr_group_lines);
            curr_group_lines.clear();
            continue;
        }

        curr_group_lines.push(line.clone());
    }

    // get last group too
    running_sum += calculate_unique_yes_answers(&curr_group_lines);

    running_sum
}

pub fn part2(input: &[String]) -> usize {
    input
        .split(|line| line.is_empty())
        .map(|group| calculate_everyone_yes_answers(group))
        .sum()
}

fn calculate_unique_yes_answers(group_answers: &[String]) -> u32 {
    let mut unique_yesses = HashSet::new();

    group_answers.iter().for_each(|line| {
        line.chars().for_each(|c| {
            unique_yesses.insert(c);
        })
    });

    unique_yesses.len() as u32
}

fn calculate_everyone_yes_answers(group_answers: &[String]) -> usize {
    // get everything in the first list, and then check each other
    // person to see if the answer is present
    assert!(group_answers.len() > 0);

    let mut first = false;

    let result = group_answers
        .iter()
        .map::<HashSet<char>, _>(|line| line.chars().collect())
        .fold(HashSet::new(), |acc: HashSet<char>, line| {
            if !first && acc.is_empty() {
                first = true;
                return line;
            }

            acc.intersection(&line).cloned().collect()
        })
        .len();

    result
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn part_1_happy() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";

        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();

        let result = part1(&lines);

        assert_eq!(result, 11);
    }

    #[test]
    fn part_2_happy() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = part2(&lines);

        assert_eq!(result, 6);
    }
}
