use std::collections::HashMap;

pub fn part1(input: &[usize]) -> usize {
    let mut working_set = input.to_vec().clone();

    // we should sort the input first
    working_set.sort();

    // println!("sorted: {:?}", input);

    let mut num_one_jolt_diff: usize = 0;
    let mut num_three_jolt_diff: usize = 0;

    let mut prev_value = 0;

    working_set.iter().for_each(|value| {
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

// we need to find all combinations where we can reach 22
// this works for the tests, but will take forever to do for the larger dataset
pub fn part2(input: &[usize]) -> usize {
    let mut working_set = input.clone().to_owned();
    working_set.sort();

    // max joltage supported is the max in the list + 3
    let max_joltage = (*working_set.iter().max().unwrap()) + 3;

    let mut cache: HashMap<usize, usize> = HashMap::new();

    fn find_paths(
        max_joltage: usize,
        curr_list_last: usize,
        curr_list: &mut [usize],
        remaining_list: &[usize],
        cache: &mut HashMap<usize, usize>,
    ) -> usize {
        // println!("distinct paths: {:?}", total_distinct_paths);
        // println!(
        //     "curr_list: {:?}, remaining_list: {:?}",
        //     curr_list, remaining_list
        // );

        if max_joltage - curr_list_last <= 3 {
            // println!("we found the missing link!");

            return 1;
        }

        // check to see if we have calculated the values for this part of the
        // list yet, and use it if we have!
        if let Some(cached_value) = cache.get(&curr_list_last) {
            return *cached_value;
        }

        let sum_of_results: usize = remaining_list
            .iter()
            .enumerate()
            .filter(|(_, v)| ((**v) > curr_list_last) && (**v - curr_list_last) <= 3)
            .map(|(i, v)| {
                // now we remove that number from the list
                let index = i.clone();

                // println!("last in curr list: {:?} -> {:?}", curr_list_last, v);

                curr_list.to_vec().push(*v);

                find_paths(
                    max_joltage,
                    *v,
                    curr_list.as_mut(),
                    &remaining_list[index..],
                    cache,
                )
            })
            .sum();

        cache.insert(curr_list_last, sum_of_results);

        sum_of_results
    };

    find_paths(max_joltage, 0, &mut vec![0], &mut working_set, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

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

    #[test]
    fn part2_test() {
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

        let result = part2(&mut lines);

        assert_eq!(19208, result);
    }
}
