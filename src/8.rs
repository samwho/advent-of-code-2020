use std::{
    collections::BTreeSet, error::Error, fs::File, io::BufRead, io::BufReader, str::FromStr,
};

#[derive(Copy, Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let instruction = match parts.as_slice() {
            ["nop", arg] => Instruction::Nop(arg.parse()?),
            ["acc", arg] => Instruction::Acc(arg.parse()?),
            ["jmp", arg] => Instruction::Jmp(arg.parse()?),
            _ => panic!("oh no"),
        };
        Ok(instruction)
    }
}

#[derive(Debug)]
struct Machine {
    ip: i32,
    accumulator: i32,
}

enum ExecutionResult {
    Success,
    InfiniteLoop,
}

impl Machine {
    fn new() -> Self {
        Machine {
            ip: 0,
            accumulator: 0,
        }
    }

    fn run(&mut self, instructions: &[Instruction]) -> Result<ExecutionResult, Box<dyn Error>> {
        self.ip = 0;
        self.accumulator = 0;
        let mut visited: BTreeSet<i32> = BTreeSet::new();

        loop {
            if visited.contains(&self.ip) {
                return Ok(ExecutionResult::InfiniteLoop);
            }

            let instruction = match instructions.get(self.ip as usize) {
                Some(i) => i,
                None => break,
            };

            visited.insert(self.ip);

            match instruction {
                Instruction::Nop(_) => {}
                Instruction::Acc(amt) => self.accumulator += amt,
                Instruction::Jmp(to) => self.ip += to - 1,
            };

            self.ip += 1;
        }

        Ok(ExecutionResult::Success)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/8")?;
    let reader = BufReader::new(file);
    let mut instructions: Vec<Instruction> = Vec::new();

    for result in reader.lines() {
        let line = result?;
        instructions.push(line.parse()?);
    }

    let mut machine = Machine::new();

    match machine.run(&instructions)? {
        ExecutionResult::Success => {}
        ExecutionResult::InfiniteLoop => {}
    };

    println!("final machine state: {:?}", machine);

    for i in 0..instructions.len() {
        let instruction = instructions.get(i).unwrap();
        match instruction {
            Instruction::Acc(_) => continue,
            Instruction::Nop(val) => {
                let mut new_instructions = instructions.clone();
                new_instructions[i] = Instruction::Jmp(*val);

                if let Ok(ExecutionResult::Success) = machine.run(&new_instructions) {
                    println!("swap {} from nop to jmp, acc: {}", i, machine.accumulator);
                    break;
                }
            }
            Instruction::Jmp(val) => {
                let mut new_instructions = instructions.clone();
                new_instructions[i] = Instruction::Nop(*val);

                if let Ok(ExecutionResult::Success) = machine.run(&new_instructions) {
                    println!("swap {} from jmp to nop, acc: {}", i, machine.accumulator);
                    break;
                }
            }
        }
    }

    Ok(())
}
