use std::{cell::Cell, collections::HashSet, io::Error, str::FromStr};

pub fn part1(input: &[String]) -> isize {
    let instructions: Vec<Instruction> = input
        .iter()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect();

    let cpu = CPU::new(instructions);
    cpu.run();

    cpu.get_accumulator_value()
}

pub fn part2(input: &[String]) -> isize {
    let og_instructions: Vec<Instruction> = input
        .iter()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect();

    let instruction_count = og_instructions.len();

    let mut next_start_index = 0;

    loop {
        let mut changed_instructions = og_instructions.clone();

        let mut new_instruction: Option<Instruction> = None;
        let mut curr_swap_index = next_start_index;

        // first, find the next instruction that is either JMP or NOP
        for index in next_start_index..instruction_count {
            let curr_instruction = changed_instructions.get(index).unwrap();

            match *curr_instruction {
                Instruction::JMP(v) => {
                    new_instruction = Some(Instruction::NOP(v));
                    curr_swap_index = index;
                    break;
                }
                Instruction::NOP(v) => {
                    new_instruction = Some(Instruction::JMP(v));
                    curr_swap_index = index;
                    break;
                }
                _ => {}
            }
        }

        assert!(new_instruction.is_some());

        // swap the instruction!
        changed_instructions[curr_swap_index] = new_instruction.unwrap();
        next_start_index = curr_swap_index + 1;

        // now instantiate a CPU with the new instructions and lets see if it finishes :x
        let cpu = CPU::new(changed_instructions);
        match cpu.run() {
            true => {
                // this means the cpu instructions completed! return the result
                let result = cpu.get_accumulator_value();
                return result;
            }
            false => {
                // no result :(
                // time to keep trying
                let (last_instruction_num, last_instruction) = cpu.get_last_instruction();
                println!(
                    "Still stuck in loop. Completed on instruction #{:?} -> {:?}",
                    last_instruction_num, last_instruction
                );
            }
        }
    }
}

struct CPU {
    accumulator: Cell<isize>,
    last_instruction_index: Cell<usize>,
    instructions: Vec<Instruction>,
}

impl CPU {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            accumulator: Cell::new(0),
            last_instruction_index: Cell::new(0),
            instructions,
        }
    }

    fn get_accumulator_value(&self) -> isize {
        self.accumulator.get()
    }

    fn get_last_instruction(&self) -> (usize, Instruction) {
        let last_instruction_index = self.last_instruction_index.get();
        (
            last_instruction_index,
            self.instructions
                .get(last_instruction_index)
                .unwrap()
                .clone(),
        )
    }

    // returns true if not terminated early due to repeat
    fn run(&self) -> bool {
        let mut instruction_cache = HashSet::<usize>::new();

        let mut instruction_num: usize = 0;
        loop {
            // at end of instructions, meaning the program terminated correctly
            if instruction_num >= self.instructions.len() {
                return true;
            }

            // a loop was found, meaning we terminated early
            if instruction_cache.get(&(instruction_num)).is_some() {
                return false;
            }

            assert!(self.instructions.len() > instruction_num);

            let instruction = self.instructions.get(instruction_num).unwrap();
            let mut instruction_increment = 1;

            // println!("running instruction: {:?}", instruction);

            match instruction {
                Instruction::NOP(_) => {}
                Instruction::ACC(value) => {
                    let prev = self.accumulator.get();
                    let result = prev.checked_add(*value).unwrap();
                    self.accumulator.set(result);
                }
                Instruction::JMP(jmp) => instruction_increment = *jmp,
            };

            instruction_cache.insert(instruction_num);

            instruction_num = (instruction_num as isize + instruction_increment) as usize;
            self.last_instruction_index.set(instruction_num);
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    NOP(isize),
    ACC(isize),
    JMP(isize),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s_split = s.split_whitespace();
        let instruction = s_split.next().unwrap();
        let instruction_num = s_split.next().unwrap().parse::<isize>().unwrap();

        match instruction {
            "nop" => Ok(Instruction::NOP(instruction_num)),
            "acc" => Ok(Instruction::ACC(instruction_num)),
            "jmp" => Ok(Instruction::JMP(instruction_num)),
            other => Err(Error::new(
                std::io::ErrorKind::Other,
                format!("{:?} is not a valid instruction", other),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{part2, Instruction, CPU};

    #[test]
    fn part1_test() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        let instructions: Vec<Instruction> = input
            .lines()
            .map(|line| Instruction::from_str(line).unwrap())
            .collect();

        let cpu = CPU::new(instructions);
        let result = cpu.run();

        assert!(!result);
        assert_eq!(cpu.get_accumulator_value(), 5);
    }

    #[test]
    fn part2_test() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();

        let result = part2(&lines);

        assert_eq!(result, 8);
    }
}
