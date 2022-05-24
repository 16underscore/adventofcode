use std::{collections::HashSet, fs};

fn main() {
	let input = fs::read_to_string("2015/three").unwrap();
	part1(&input);
}

fn part1(input: &String) {
	let characters = input.chars();
	let mut houses: HashSet<(i16, i16)> = HashSet::new();
	let mut x: i16 = 0;
	let mut y: i16 = 0;
	houses.insert((x, y));
	for character in characters {
		match character {
			'<' => x -= 1,
			'>' => x += 1,
			'v' => y -= 1,
			'^' => y += 1,
			_ => panic!(""),
		}
		houses.insert((x, y));
	}
	println!("The result for the first part is: {}", houses.len());
}
