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
            self.values.remove(0);
        }

        self.recalculate_sums();
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
    use super::part1;

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
}
