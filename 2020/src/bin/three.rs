use std::fs;

fn main() {
	let input = fs::read_to_string("2020/three").unwrap();
	let rows: Vec<&str> = input.split_whitespace().collect();
	part1(&rows).unwrap();
	part2(&rows).unwrap();
}

fn part1(rows: &Vec<&str>) -> Option<()> {
	println!(
		"The result for the first part is: {}",
		traverse(rows, 3, 1)?
	);
	Some(())
}

fn part2(rows: &Vec<&str>) -> Option<()> {
	let mut product: u32 = 1;
	product *= traverse(rows, 1, 1)?;
	product *= traverse(rows, 3, 1)?;
	product *= traverse(rows, 5, 1)?;
	product *= traverse(rows, 7, 1)?;
	product *= traverse(rows, 1, 2)?;
	println!("The result for the second part is: {}", product);
	Some(())
}

fn traverse(rows: &Vec<&str>, right: usize, down: usize) -> Option<u32> {
	let mut position: usize = 0;
	let mut counter: u32 = 0;
	for i in (0..rows.len()).step_by(down) {
		let row_len = rows.get(i)?.len();
		if position >= row_len {
			position -= row_len;
		}
		let row: Vec<char> = rows.get(i)?.chars().collect();
		if row.get(position)?.eq(&'#') {
			counter += 1;
		}
		position += right;
	}
	Some(counter)
}
