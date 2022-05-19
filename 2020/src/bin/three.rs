use std::fs;

fn main() {
	let input = fs::read_to_string("2020/three").unwrap();
	let rows: Vec<&str> = input.split_whitespace().collect();
	part1(&rows).unwrap();
}

fn part1(rows: &Vec<&str>) -> Option<()> {
	let mut position: usize = 0;
	let mut counter = 0;
	for row in rows {
		if position >= row.len() {
			position -= row.len();
		}
		let row: Vec<char> = row.chars().collect();
		if row.get(position)?.eq(&'#') {
			counter += 1;
		}
		position += 3;
	}
	println!("The result for the second part is: {}", counter);
	Some(())
}
