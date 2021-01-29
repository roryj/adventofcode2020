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

    let day5_input = input_utils::read_all("inputs/day5");

    println!("Day 5 - Part 1: {}", day5::part1(&day5_input));
    println!("Day 5 - Part 2: {}", day5::part2(&day5_input));

    let day6_input = input_utils::read_all("inputs/day6");

    println!("Day 6 - Part 1: {}", day6::part1(&day6_input));
    println!("Day 6 - Part 2: {}", day6::part2(&day6_input));

    let day7_input = input_utils::read_all("inputs/day7");

    println!("Day 7 - Part 1: {}", day7::part1(&day7_input));
    println!("Day 7 - Part 2: {}", day7::part2(&day7_input));

    let day8_input = input_utils::read_all("inputs/day8");

    println!("Day 8 - Part 1: {}", day8::part1(&day8_input));
    println!("Day 8 - Part 2: {}", day8::part2(&day8_input));

    let day9_input = input_utils::read_all_as::<usize>("inputs/day9");

    println!("Day 9 - Part 1: {}", day9::part1(&day9_input, 25));
    // this one is very slow, but gets the right answer. Too lazy to optimize ¯\_(ツ)_/¯
    println!("Day 9 - Part 2 (Commented out because it is very slow). Uncomment to get answer!");
    // println!("Day 9 - Part 2: {}", day9::part2(&day9_input, 10884537));

    let day10_input = input_utils::read_all_as::<usize>("inputs/day10");

    println!("Day 10 - Part 1: {}", day10::part1(&day10_input));
    println!("Day 10 - Part 2: {}", day10::part2(&day10_input));

    let day11_input = input_utils::read_all("inputs/day11");

    println!("Day 11 - Part 1: {}", day11::part1(&day11_input));
    println!("Day 11 - Part 2: {}", day11::part2(&day11_input));
}
