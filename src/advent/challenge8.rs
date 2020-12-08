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
                        "nop" => Instruction::Nop(i),
                        "acc" => Instruction::Acc(i, number),
                        "jmp" => Instruction::Jmp(i, number),
                        _ => panic!("Invalid value"),
                    };
                    instruction
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn part1(&self) -> String {
        let mut accumulator: i32 = 0;

        let instruction_length = self.instructions.len();
        let mut index: usize = 0;

        let mut executed_instructions_index: Vec<usize> = Vec::new();

        while index < instruction_length {
            if executed_instructions_index.contains(&index) {
                break;
            }

            executed_instructions_index.push(index);

            let instruction = &self.instructions[index];

            // Remark: it's not possible to destructure a tuple into existing variables
            let (new_index, new_accumulator) =
                instruction.execute_and_return_next_instruction_index_plus_accumulator(accumulator);

            index = new_index;
            accumulator = new_accumulator;
        }

        accumulator.to_string()
    }

    pub fn part2(&self) -> String {
        String::from("TODO")
    }
}

#[derive(Debug)]
enum Instruction {
    Acc(usize, i32),
    Jmp(usize, i32),
    Nop(usize),
}

impl Instruction {
    pub fn execute_and_return_next_instruction_index_plus_accumulator(
        &self,
        accumalator: i32,
    ) -> (usize, i32) {
        match self {
            Instruction::Nop(i) => (i + 1, accumalator),
            Instruction::Jmp(i, j) => {
                if j.is_negative() {
                    (i - (j.wrapping_abs() as u32 as usize), accumalator)
                } else {
                    (i + (*j as usize), accumalator)
                }
            }
            Instruction::Acc(i, j) => (i + 1, accumalator + j),
        }
    }
}
