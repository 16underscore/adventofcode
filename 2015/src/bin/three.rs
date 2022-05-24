use std::{collections::HashSet, fs};

fn main() {
	let input = fs::read_to_string("2015/three").unwrap();
	part1(&input);
	part2(&input);
}

fn part1(input: &String) {
	println!("The result for the first part is: {}", deliver(input, 1));
}

fn part2(input: &String) {
	println!("The result for the second part is: {}", deliver(input, 2));
}

fn deliver(characters: &String, santas: usize) -> usize {
	let mut houses: HashSet<(i16, i16)> = HashSet::new();
	let mut x: Vec<i16> = vec![0; santas];
	let mut y: Vec<i16> = vec![0; santas];
	houses.insert((0, 0));
	for c in 0..characters.len() {
		let i = c % santas;
		match characters.get(c..c + 1).unwrap() {
			"<" => x[i] -= 1,
			">" => x[i] += 1,
			"v" => y[i] -= 1,
			"^" => y[i] += 1,
			_ => panic!(""),
		}
		houses.insert((x[i], y[i]));
		houses.insert((x[i], y[i]));
	}
	houses.len()
}
