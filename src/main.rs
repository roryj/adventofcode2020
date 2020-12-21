use adventofcode2020::*;


fn main() {

    let day1_input = input_utils::read_all_as::<u32>("inputs/day1");

    println!("Day 1 - Part 1: {}", day1::part1(&day1_input));
    println!("Day 1 - Part 2: {}", day1::part2(&day1_input));
}
