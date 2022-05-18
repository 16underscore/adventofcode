use std::fs;

fn main() {
	let input = fs::read_to_string("2020/two").unwrap();
	let rows: Vec<&str> = input.split('\n').collect();
	part1(&rows);
}

fn part1(rows: &Vec<&str>) {
	let mut valid_passwords: u16 = 0;
	for &row in rows {
		let row: Vec<&str> = row.split_whitespace().collect();
		let numbers: Vec<u8> = row
			.get(0)
			.unwrap()
			.split('-')
			.map(|x| x.parse().unwrap())
			.collect();
		let letter = row.get(1).unwrap().get(0..1).unwrap();
		let password = row.get(2).unwrap();
		let mut count = 0;
		for char in password.chars() {
			if letter.contains(char) {
				count += 1;
			}
		}
		if count >= *numbers.get(0).unwrap() && count <= *numbers.get(1).unwrap() {
			valid_passwords += 1;
		}
	}
	println!("The result for the first part is: {}", valid_passwords);
}
