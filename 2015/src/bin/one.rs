use std::fs;

fn main() {
	let input = fs::read_to_string("2015/one").unwrap();
	part1(&input);
	part2(&input);
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

fn part2(input: &String) {
	let mut counter: i16 = 0;
	for i in 0..input.len() {
		match input.get(i..i + 1).unwrap() {
			"(" => counter += 1,
			")" => {
				if counter == 0 {
					println!("The result for the second part is: {}", i + 1);
					return;
				}
				counter -= 1
			}
			_ => panic!(""),
		}
	}
}
