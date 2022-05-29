use std::fs;

enum Instruction {
	ON(usize, usize, usize, usize),
	OFF(usize, usize, usize, usize),
	TOGGLE(usize, usize, usize, usize),
}

fn main() {
	let input = fs::read_to_string("2015/six").unwrap();
	let instructions: Vec<Instruction> = input.split('\n').map(parse).collect();
	part1(&instructions);
	part2(&instructions);
}

fn parse(row: &str) -> Instruction {
	let row = row.replace(',', " ");
	let instruction: Vec<&str> = row.split_whitespace().collect();
	if row.contains("toggle") {
		let (x0, y0, x1, y1) = get_area(&instruction, 0).unwrap();
		return Instruction::TOGGLE(x0, y0, x1, y1);
	} else {
		let (x0, y0, x1, y1) = get_area(&instruction, 1).unwrap();
		if *instruction.get(1).unwrap() == "on" {
			return Instruction::ON(x0, y0, x1, y1);
		}
		return Instruction::OFF(x0, y0, x1, y1);
	}
}

fn get_area(instruction: &Vec<&str>, offset: usize) -> Option<(usize, usize, usize, usize)> {
	let x0: usize = instruction.get(1 + offset)?.parse().unwrap();
	let y0: usize = instruction.get(2 + offset)?.parse().unwrap();
	let x1: usize = instruction.get(4 + offset)?.parse().unwrap();
	let y1: usize = instruction.get(5 + offset)?.parse().unwrap();
	Some((x0, y0, x1, y1))
}

fn part1(instructions: &Vec<Instruction>) {
	let mut lights = [[false; 1000]; 1000];
	for instruction in instructions {
		match instruction {
			Instruction::ON(x0, y0, x1, y1) => {
				for x in *x0..*x1 + 1 {
					for y in *y0..*y1 + 1 {
						lights[x][y] = true;
					}
				}
			}
			Instruction::OFF(x0, y0, x1, y1) => {
				for x in *x0..*x1 + 1 {
					for y in *y0..*y1 + 1 {
						lights[x][y] = false;
					}
				}
			}
			Instruction::TOGGLE(x0, y0, x1, y1) => {
				for x in *x0..*x1 + 1 {
					for y in *y0..*y1 + 1 {
						lights[x][y] = !lights[x][y];
					}
				}
			}
		}
	}
	let mut count: u32 = 0;
	for i in 0..lights.len() {
		for j in 0..lights[i].len() {
			if lights[i][j] {
				count += 1;
			}
		}
	}
	println!("The result for the first part is: {}", count);
}

fn part2(instructions: &Vec<Instruction>) {
	let mut lights: [[u32; 1000]; 1000] = [[0; 1000]; 1000];
	for instruction in instructions {
		match instruction {
			Instruction::ON(x0, y0, x1, y1) => {
				for x in *x0..*x1 + 1 {
					for y in *y0..*y1 + 1 {
						lights[x][y] += 1;
					}
				}
			}
			Instruction::OFF(x0, y0, x1, y1) => {
				for x in *x0..*x1 + 1 {
					for y in *y0..*y1 + 1 {
						if lights[x][y] > 0 {
							lights[x][y] -= 1;
						}
					}
				}
			}
			Instruction::TOGGLE(x0, y0, x1, y1) => {
				for x in *x0..*x1 + 1 {
					for y in *y0..*y1 + 1 {
						lights[x][y] += 2;
					}
				}
			}
		}
	}
	let mut count: u32 = 0;
	for i in 0..lights.len() {
		for j in 0..lights[i].len() {
			count += lights[i][j];
		}
	}
	println!("The result for the second part is: {}", count);
}
