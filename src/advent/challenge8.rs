use std::fs;

#[derive(Debug)]
pub struct Challenge8 {
    instructions: Vec<Instruction>,
}

impl Challenge8 {
    pub fn new() -> Self {
        let file_content = fs::read_to_string("./src/data/puzzle8.data")
            .expect("Something went wrong while reading the file");

        Challenge8 {
            instructions: file_content
                .lines()
                .enumerate()
                .map(|(i, x)| {
                    let split = x.split(" ").collect::<Vec<_>>();

                    if split.len() != 2 {
                        panic!("Invalid input");
                    }

                    let instruction_string = split[0];
                    let number = split[1].parse::<i32>().unwrap();

                    let instruction = match instruction_string {
                        "nop" => Instruction::NOP(i, number),
                        "acc" => Instruction::ACC(i, number),
                        "jmp" => Instruction::JMP(i, number),
                        _ => panic!("Invalid value"),
                    };
                    instruction
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn part1(&self) -> String {
        let (_, accumulator) =
            iterate_instructions_return_loop_detected_and_accumulator(&self.instructions);
        accumulator.to_string()
    }

    pub fn part2(&self) -> String {
        let instruction_length = self.instructions.len();
        let mut index: usize = 0;

        let mut executed_instructions_index: Vec<usize> = Vec::new();
        let mut pattern: Vec<usize> = Vec::new();

        while index < instruction_length {
            if executed_instructions_index.contains(&index) {
                let start_of_pattern = executed_instructions_index
                    .iter()
                    .position(|x| *x == index)
                    .unwrap();

                pattern = executed_instructions_index[start_of_pattern..]
                    .iter()
                    .filter(|&x| match self.instructions[*x] {
                        Instruction::ACC(_, _) => false,
                        Instruction::JMP(_, _) => true,
                        Instruction::NOP(_, _) => true,
                    })
                    .copied()
                    .collect::<Vec<_>>();

                break;
            }

            executed_instructions_index.push(index);

            let instruction = &self.instructions[index];

            // Remark: it's not possible to destructure a tuple into existing variables
            let (new_index, _) =
                instruction.execute_and_return_next_instruction_index_plus_accumulator(0);

            index = new_index;
        }

        for pattern_index in pattern {
            let mut instructions: Vec<Instruction> = self.instructions.iter().copied().collect();
            instructions[pattern_index] = match instructions[pattern_index] {
                Instruction::ACC(_, _) => panic!("Invalid input"),
                Instruction::NOP(i, j) => Instruction::JMP(i, j),
                Instruction::JMP(i, j) => Instruction::NOP(i, j),
            };

            let (has_loop, accumulator) =
                iterate_instructions_return_loop_detected_and_accumulator(&instructions);

            if !has_loop {
                return accumulator.to_string();
            }
        }

        String::from("Could not fix loop!")
    }
}

fn iterate_instructions_return_loop_detected_and_accumulator(
    instructions: &Vec<Instruction>,
) -> (bool, i32) {
    let mut accumulator: i32 = 0;
    let mut has_loop = false;

    let instruction_length = instructions.len();
    let mut index: usize = 0;

    let mut executed_instructions_index: Vec<usize> = Vec::new();

    while index < instruction_length {
        if executed_instructions_index.contains(&index) {
            has_loop = true;
            break;
        }

        executed_instructions_index.push(index);

        let instruction = &instructions[index];

        // Remark: it's not possible to destructure a tuple into existing variables
        let (new_index, new_accumulator) =
            instruction.execute_and_return_next_instruction_index_plus_accumulator(accumulator);

        index = new_index;
        accumulator = new_accumulator;
    }

    (has_loop, accumulator)
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    ACC(usize, i32),
    JMP(usize, i32),
    NOP(usize, i32),
}

impl Instruction {
    pub fn execute_and_return_next_instruction_index_plus_accumulator(
        &self,
        accumalator: i32,
    ) -> (usize, i32) {
        match self {
            Instruction::NOP(i, _j) => (i + 1, accumalator),
            Instruction::JMP(i, j) => {
                if j.is_negative() {
                    (i - (j.wrapping_abs() as u32 as usize), accumalator)
                } else {
                    (i + (*j as usize), accumalator)
                }
            }
            Instruction::ACC(i, j) => (i + 1, accumalator + j),
        }
    }
}
