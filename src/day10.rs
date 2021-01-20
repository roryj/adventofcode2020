pub fn part1(input: &mut [usize]) -> usize {
    // we should sort the input first
    input.sort();

    // println!("sorted: {:?}", input);

    let mut num_one_jolt_diff: usize = 0;
    let mut num_three_jolt_diff: usize = 0;

    let mut prev_value = 0;

    input.iter().for_each(|value| {
        // println!("prev: {:?}, value: {:?}", prev_value, value);
        match value {
            v if v.clone() == prev_value + 3 => {
                num_three_jolt_diff += 1;
            }
            v if v.clone() == prev_value + 1 => {
                num_one_jolt_diff += 1;
            }
            _ => panic!("should not have happened :|"),
        };

        // println!("1-jolt-diff: {:?}", num_one_jolt_diff);
        // println!("3-jolt-diff: {:?}", num_three_jolt_diff);

        prev_value = value.clone();
    });

    // add one more 3-jolt diff because:
    // "Finally, your device's built-in adapter is always 3 higher than the
    // highest adapter, so its rating is 22 jolts (always a difference of 3)."
    num_three_jolt_diff += 1;

    num_one_jolt_diff * num_three_jolt_diff
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn part1_test() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let mut lines: Vec<usize> = input
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .collect();

        let result = part1(&mut lines);

        assert_eq!(22 * 10, result);
    }
}
