use adventofcode2020::*;

fn main() {
    let day1_input = input_utils::read_all_as::<u32>("inputs/day1");

    println!("Day 1 - Part 1: {}", day1::part1(&day1_input));
    println!("Day 1 - Part 2: {}", day1::part2(&day1_input));

    let day2_input = input_utils::read_all("inputs/day2");

    println!("Day 2 - Part 1: {}", day2::part1(&day2_input));
    println!("Day 2 - Part 2: {}", day2::part2(&day2_input));

    let day3_input = input_utils::read_all("inputs/day3");

    println!("Day 3 - Part 1: {}", day3::part1(&day3_input));
    println!("Day 3 - Part 2: {}", day3::part2(&day3_input));

    let day4_input = input_utils::read_all("inputs/day4");

    println!("Day 4 - Part 1: {}", day4::part1(&day4_input));
    println!("Day 4 - Part 2: {}", day4::part2(&day4_input));
}
