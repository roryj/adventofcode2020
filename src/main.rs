use adventofcode2020::*;


fn main() {

    let day1_input = input_utils::read_all_as::<u32>("inputs/day1");

    println!("Day 1 - Part 1: {}", day1::part1(&day1_input));
    println!("Day 1 - Part 2: {}", day1::part2(&day1_input));

    let day2_input = input_utils::read_all("inputs/day2");

    println!("Day 2 - Part 1: {}", day2::part1(&day2_input));
    println!("Day 2 - Part 2: {}", day2::part2(&day2_input));
}
