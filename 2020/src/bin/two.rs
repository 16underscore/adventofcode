use std::fs;

fn main() {
	let input = fs::read_to_string("2020/two").unwrap();
	let rows: Vec<&str> = input.split('\n').collect();
	part1(&rows);
}

fn part1(rows: &Vec<&str>) {
	
}
