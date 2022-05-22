use std::fs;

fn main() {
	let input = fs::read_to_string("2015/one").unwrap();
	part1(&input);
}

fn part1(input: &String) {
	let characters = input.chars();
	let mut counter: i16 = 0;
	for character in characters {
		match character {
			'(' => counter += 1,
			')' => counter -= 1,
			_ => panic!(""),
		}
	}
	println!("The result for the first part is: {}", counter);
}
