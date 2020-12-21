pub fn part1(expenses: &[u32]) -> u32 {
    for (pos1, expense1) in expenses.iter().enumerate() {
        for (pos2, expense2) in expenses.iter().enumerate() {
            if pos1 != pos2 && expense1 + expense2 == 2020 {
                return expense1 * expense2;
            }
        }
    }

    panic!("couldnt find two inputs that add to 2020");
}

pub fn part2(expenses: &[u32]) -> u32 {
    for (pos1, expense1) in expenses.iter().enumerate() {
        for (pos2, expense2) in expenses.iter().enumerate() {
            for (pos3, expense3) in expenses.iter().enumerate() {
                if pos1 != pos2  && pos2 != pos3 && expense1 + expense2 + expense3 == 2020 {
                    return expense1 * expense2 * expense3;
                }
            }
        }
    }

    panic!("couldnt find three inputs that add to 2020");
}