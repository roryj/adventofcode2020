use std::{collections::HashSet, io::Error, str::FromStr};

pub fn part1(input: &[String]) -> isize {
    let instructions: Vec<Instruction> = input
        .iter()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect();

    let mut cpu = CPU::new(instructions);
    cpu.run_till_repeat();

    cpu.accumulator
}

struct CPU {
    accumulator: isize,
    instructions: Vec<Instruction>,
}

impl CPU {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            accumulator: 0,
            instructions,
        }
    }

    fn run_till_repeat(&mut self) {
        let mut instruction_cache = HashSet::<usize>::new();

        let mut instruction_num: usize = 0;
        loop {
            if instruction_cache.get(&(instruction_num)).is_some() {
                return;
            }

            assert!(self.instructions.len() > instruction_num);

            let instruction = self.instructions.get(instruction_num).unwrap();
            let mut instruction_increment = 1;

            println!("running instruction: {:?}", instruction);

            match instruction {
                Instruction::NOP => {}
                Instruction::ACC(value) => self.accumulator += value,
                Instruction::JMP(jmp) => instruction_increment = *jmp,
            };

            instruction_cache.insert(instruction_num);

            instruction_num = (instruction_num as isize + instruction_increment) as usize;
        }
    }
}

#[derive(Debug)]
enum Instruction {
    NOP,
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
            "nop" => Ok(Instruction::NOP),
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

    use super::{Instruction, CPU};

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

        let mut cpu = CPU::new(instructions);
        cpu.run_till_repeat();

        assert_eq!(cpu.accumulator, 5);
    }
}
