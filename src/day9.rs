use std::collections::HashSet;

pub fn part1(input: &[usize], preamble_size: usize) -> usize {
    // first 25 numbers is preamble
    // all future numbers must be a sum of two of those numbers
    assert!(preamble_size < input.len());

    let mut decryptor = XMASDecryptor::new(preamble_size);

    input[0..preamble_size]
        .iter()
        .for_each(|v| decryptor.add_new_value(v.clone()));

    input
        .iter()
        .skip(preamble_size)
        .find(|v| {
            let value = *v.clone();
            if !decryptor.is_value_in_running_sums(value) {
                return true;
            }
            decryptor.add_new_value(value);
            false
        })
        .unwrap()
        .to_owned()
}

pub fn part2(input: &[usize], expected_sum: usize) -> usize {
    let mut decryptor = XMASDecryptor::new(input.len());

    // get where the sum is equal of a contiguous set of numbers is equal to the expected_sum
    input
        .iter()
        .find(|v| {
            let value = *v.clone();

            // println!("adding value: {:?}", value);

            decryptor.add_new_value(value);

            loop {
                // println!("running values: {:?}", decryptor.values);
                // println!("current running sum: {:?}", decryptor.running_sum());
                if decryptor.running_sum() == expected_sum {
                    return true;
                }

                if decryptor.running_sum() < expected_sum {
                    break;
                }

                // println!("sum is too high! removing from decryptor");
                decryptor.remove_from_front();
                // println!("post-removal: {:?}", decryptor.values);
            }
            false
        })
        .unwrap();

    // now that decryptor has the numbers, we need to add the smallest and largest!
    decryptor.min_value() + decryptor.max_value()
}

#[derive(Debug)]
struct XMASDecryptor {
    running_sum_length: usize,
    all_sums: HashSet<usize>,
    values: Vec<usize>,
}

impl XMASDecryptor {
    fn new(running_sum_length: usize) -> Self {
        Self {
            running_sum_length,
            all_sums: HashSet::new(),
            values: Vec::new(),
        }
    }

    fn is_value_in_running_sums(&self, value: usize) -> bool {
        self.all_sums.contains(&value)
    }

    fn add_new_value(&mut self, value: usize) {
        self.values.push(value);

        if self.values.len() > self.running_sum_length {
            self.remove_from_front();
        }

        self.recalculate_sums();
    }

    fn min_value(&self) -> usize {
        self.values.iter().min().unwrap().to_owned()
    }

    fn max_value(&self) -> usize {
        self.values.iter().max().unwrap().to_owned()
    }

    fn running_sum(&self) -> usize {
        self.values.iter().sum()
    }

    fn remove_from_front(&mut self) {
        self.values.remove(0);
    }

    fn recalculate_sums(&mut self) {
        let mut all_sums = HashSet::new();

        for i in 0..self.values.len() {
            self.values.iter().skip(i + 1).for_each(|v| {
                let sum = self.values[i] + v;
                // println!("{:?} + {:?} = {:?}", self.values[i], v, sum);
                all_sums.insert(sum);
            })
        }

        self.all_sums = all_sums;
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn part1_test() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

        let lines: Vec<usize> = input
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .collect();

        let result = part1(&lines, 5);

        assert_eq!(result, 127);
    }

    #[test]
    fn part2_test() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

        let lines: Vec<usize> = input
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .collect();

        let result = part2(&lines, 127);

        assert_eq!(result, 62);
    }
}
