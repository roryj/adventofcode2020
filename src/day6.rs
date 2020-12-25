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

fn calculate_unique_yes_answers(group_answers: &[String]) -> u32 {
    let mut unique_yesses = HashSet::new();

    group_answers.iter().for_each(|line| {
        line.chars().for_each(|c| {
            unique_yesses.insert(c);
        })
    });

    unique_yesses.len() as u32
}

#[cfg(test)]
mod tests {
    use super::part1;

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
}
