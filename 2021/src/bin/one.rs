use std::fs;

fn main() {
	let input: String = fs::read_to_string("2021/one").unwrap();
	let rows: Vec<u16> = input.split('\n').map(|x| x.parse().unwrap()).collect();
	part1(&rows);
	part2(&rows);
}

fn part1(rows: &Vec<u16>) {
	let mut counter: u16 = 0;
	for i in 1..rows.len() {
		if rows.get(i - 1) < rows.get(i) {
			counter += 1;
		}
	}
	println!("The result for the first part is: {}", counter);
}

fn part2(rows: &Vec<u16>) {
	let mut counter: u16 = 0;
	for i in 3..rows.len() {
		let sum1 = sum_three(i - 1, &rows).unwrap();
		let sum2 = sum_three(i, &rows).unwrap();
		if sum1 < sum2 {
			counter += 1;
		}
	}
	println!("The result for the second part is: {}", counter);
}

fn sum_three(i: usize, rows: &Vec<u16>) -> Option<u16> {
	let sum = rows.get(i - 2)? + rows.get(i - 1)? + rows.get(i)?;
	Some(sum)
}
