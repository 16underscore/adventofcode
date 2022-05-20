use std::{collections::HashSet, convert::TryInto, fs};

#[derive(Clone)]
enum Instruction {
	ACC(i16),
	JMP(i16),
	NOP(i16),
}

impl Instruction {
	fn new(operation: &str, argument: i16) -> Instruction {
		match operation {
			"acc" => Instruction::ACC(argument),
			"jmp" => Instruction::JMP(argument),
			"nop" => Instruction::NOP(argument),
			_ => panic!("Something went wrong!"),
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
	part2(&instructions);
}

fn part1(instructions: &Vec<Instruction>) {
	let result = terminates(instructions);
	println!("The result for the first part is: {}", result.unwrap_err());
}

fn part2(instructions: &Vec<Instruction>) {
	for i in 0..instructions.len() {
		let replacement = match instructions.get(i).unwrap() {
			Instruction::ACC(_) => continue,
			Instruction::JMP(argument) => Instruction::NOP(*argument),
			Instruction::NOP(argument) => Instruction::JMP(*argument),
		};
		let edited_instructions: &mut Vec<Instruction> = &mut instructions.clone();
		edited_instructions.insert(i, replacement);
		let acc = terminates(edited_instructions);
		if acc.is_ok() {
			println!("The result for the second part is: {}", acc.unwrap());
			break;
		}
	}
}

fn terminates(instructions: &Vec<Instruction>) -> Result<i16, i16> {
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
			Instruction::NOP(_) => program_counter += 1,
		}
		if !visited.insert(program_counter) {
			return Err(accumulator);
		}
	}
	Ok(accumulator)
}
