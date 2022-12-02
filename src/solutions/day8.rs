const INPUT: &str = include_str!("../inputs/day8");

pub fn run() -> (String, String) {
    let instructions: Vec<Instruction> = INPUT.lines().map(parse_line).collect();
    let instruction_length = instructions.len();
    let mut program = Program::new(instructions);
    let part1 = match program.run(None) {
        TerminationState::InfiniteLoop(i) => i,
        TerminationState::Completed(_) => unreachable!(),
    };
    let mut mutation_index = 0;
    let part2 = loop {
        program.reset();
        match program.run(Some(mutation_index)) {
            TerminationState::InfiniteLoop(_) => mutation_index += 1,
            TerminationState::Completed(i) => break i,
        }
        if mutation_index > instruction_length {
            panic!("No valid mutation found");
        }
    };
    (part1.to_string(), part2.to_string())
}

fn parse_line(l: &str) -> Instruction {
    let (ins, num) = l.split_once(' ').unwrap();
    match ins {
        "acc" => Instruction::Acc(num.parse().unwrap()),
        "jmp" => Instruction::Jump(num.parse().unwrap()),
        "nop" => Instruction::Nop(num.parse().unwrap()),
        _ => unreachable!(),
    }
}
enum Instruction {
    Nop(i32),
    Jump(i32),
    Acc(i32),
}

enum TerminationState {
    InfiniteLoop(i32),
    Completed(i32),
}
struct Program {
    counter: i32,
    acc: i32,
    instructions: Vec<Instruction>,
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Self {
        Program {
            counter: 0,
            acc: 0,
            instructions,
        }
    }

    fn reset(&mut self) {
        self.counter = 0;
        self.acc = 0;
    }

    fn run(&mut self, mutation_index: Option<usize>) -> TerminationState {
        let program_length = self.instructions.len();
        let mut seen = vec![false; program_length];
        loop {
            if self.counter as usize == program_length {
                break TerminationState::Completed(self.acc);
            }
            if seen[self.counter as usize] {
                break TerminationState::InfiniteLoop(self.acc);
            }
            seen[self.counter as usize] = true;

            let instruction = &self.instructions[self.counter as usize];

            if let Some(index) = mutation_index {
                if index == self.counter as usize {
                    match instruction {
                        Instruction::Nop(i) => self.counter += i,
                        Instruction::Jump(_) => self.counter += 1,
                        Instruction::Acc(i) => {
                            self.acc += i;
                            self.counter += 1;
                        }
                    }
                    continue;
                }
            }
            match instruction {
                Instruction::Nop(_) => self.counter += 1,
                Instruction::Jump(i) => self.counter += i,
                Instruction::Acc(i) => {
                    self.acc += i;
                    self.counter += 1;
                }
            }
        }
    }
}
