use std::fs;

fn main() {
	let input = fs::read_to_string("2015/eight").unwrap();
	let rows: Vec<&str> = input.split('\n').collect();
	part1(&rows);
}

fn part1(rows: &Vec<&str>) {
	let mut character_count = 0;
	for row in rows {}
	println!("The result for the first part is: {}", character_count);
}
