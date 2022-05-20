use std::{collections::HashSet, convert::TryInto, fs};
enum Instruction {
	ACC(i16),
	JMP(i16),
	NOP,
}

impl Instruction {
	fn new(operation: &str, argument: i16) -> Instruction {
		match operation {
			"acc" => Instruction::ACC(argument),
			"jmp" => Instruction::JMP(argument),
			_ => Instruction::NOP,
		}
	}
}

fn main() {
	let input = fs::read_to_string("2020/eight").unwrap();
	let instructions: Vec<Instruction> = input
		.split('\n')
		.map(|row| {
			let (operation, argument) = row.split_once(' ').unwrap();
			Instruction::new(operation, argument.parse().unwrap())
		})
		.collect();
	part1(&instructions);
}

fn part1(instructions: &Vec<Instruction>) {
	let mut program_counter: i16 = 0;
	let mut accumulator: i16 = 0;
	let mut visited: HashSet<i16> = HashSet::new();
	while program_counter < instructions.len().try_into().unwrap() {
		match instructions.get(program_counter as usize).unwrap() {
			Instruction::ACC(argument) => {
				accumulator += argument;
				program_counter += 1;
			}
			Instruction::JMP(argument) => program_counter += argument,
			Instruction::NOP => program_counter += 1,
		}
		if !visited.insert(program_counter) {
			break;
		}
	}
	println!("The result for the first part is: {}", accumulator);
}
