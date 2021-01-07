use std::{collections::HashMap, env::consts::FAMILY};

pub fn part1(input: &[String]) -> usize {
    let rules = input.iter().map(|line| parse_rule(line)).fold(
        HashMap::new(),
        |mut acc, (inner_bag, map)| {
            acc.insert(inner_bag, map);
            acc
        },
    );

    let mut cache = HashMap::<String, bool>::new();

    fn contains_shiny_gold(
        rules: &HashMap<&str, HashMap<&str, u32>>,
        contains: &mut HashMap<String, bool>,
        bag_colour: &str,
    ) -> bool {
        return contains.get(bag_colour).copied().unwrap_or_else(|| {
            for (inner_colour, _) in rules.get(bag_colour).unwrap().iter() {
                if *inner_colour == "shiny gold bag"
                    || contains_shiny_gold(rules, contains, inner_colour)
                {
                    contains.insert(String::from(bag_colour), true);
                    return true;
                }
            }
            contains.insert(String::from(bag_colour), false);
            false
        });
    }

    let mut count = 0;
    rules.iter().for_each(|(k, _v)| {
        if contains_shiny_gold(&rules, &mut cache, k) {
            count += 1;
        }
    });

    count
}

pub fn part2(input: &[String]) -> u32 {
    let rules = input.iter().map(|line| parse_rule(line)).fold(
        HashMap::new(),
        |mut acc, (inner_bag, map)| {
            acc.insert(inner_bag, map);
            acc
        },
    );

    let mut cache = HashMap::<String, usize>::new();

    fn get_inner_bag_count(
        rules: &HashMap<&str, HashMap<&str, u32>>,
        cache: &mut HashMap<String, usize>,
        bag_colour: &str,
    ) -> usize {
        return cache.get(bag_colour).copied().unwrap_or_else(|| {
            let mut count = 0;
            for (inner_colour, quantity) in rules.get(bag_colour).unwrap().iter() {
                count +=
                    (1 + get_inner_bag_count(rules, cache, inner_colour)) * (*quantity as usize);
            }
            cache.insert(String::from(bag_colour), count);
            count
        });
    }

    get_inner_bag_count(&rules, &mut cache, "shiny gold bag") as u32
}

fn parse_rule(rule_line: &str) -> (&str, HashMap<&str, u32>) {
    let rule_split: Vec<&str> = rule_line.split("contain").collect();
    assert_eq!(rule_split.len(), 2);

    let outer_bag = rule_split[0].trim().trim_end_matches("s");
    let inner_bag_rules = rule_split[1].trim();

    let inner_bags: HashMap<&str, u32> = inner_bag_rules
        .split_terminator::<_>(",")
        .map(|rule| {
            let rule_details: Vec<&str> =
                rule.trim_end_matches(".").trim().splitn(2, ' ').collect();
            // println!("Rule: {:?}", rule);
            // println!("split result: {:?}", rule_details);
            (rule_details[1].trim_end_matches("s"), rule_details[0])
        })
        .fold(HashMap::new(), |mut acc, (k, v)| {
            match v.parse::<u32>() {
                Ok(num_bags) => {
                    acc.insert(k, num_bags);
                    ()
                }
                _ => (), // no parse because it is a "no other bags"
            }
            acc
        });

    (outer_bag, inner_bags)
}

#[cfg(test)]
mod tests {
    use super::{parse_rule, part2};

    #[test]
    fn parse_rule_happy() {
        let line = "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.";

        let (result_outer_bag, result_map) = parse_rule(line);

        assert_eq!(result_outer_bag, "muted yellow bag");
        assert!(result_map.contains_key("shiny gold bag"));
        let gold_bag_count = result_map.get("shiny gold bag").unwrap().to_owned();
        assert_eq!(gold_bag_count, 2);
        let faded_blue_bag_count = result_map.get("faded blue bag").unwrap().to_owned();
        assert_eq!(faded_blue_bag_count, 9);
    }

    #[test]
    fn parse_rule_no_inner_bag() {
        let line = "dotted black bags contain no other bags.";
        let (result_outer_bag, result_map) = parse_rule(line);

        assert_eq!(result_outer_bag, "dotted black bag");
        assert!(result_map.is_empty());
    }

    #[test]
    fn part2_happy1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        // 1 + (1 * ((3 * 1) + (4 * 1))) + 2 + (2 * ((5 * 1) + (6 *1)))

        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = part2(&lines);

        assert_eq!(result, 32);
    }

    #[test]
    fn part2_happy2() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let result = part2(&lines);

        assert_eq!(result, 126);
    }
}
