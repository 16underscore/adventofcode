use std::fs;

fn main() {
	let input = fs::read_to_string("2020/two").unwrap();
	let rows: Vec<&str> = input.split('\n').collect();
	part1(&rows).unwrap();
	part2(&rows).unwrap();
}

fn part1(rows: &Vec<&str>) -> Option<()> {
	let mut valid_passwords: u16 = 0;
	for &row in rows {
		let row: Vec<&str> = row.split_whitespace().collect();
		let numbers: Vec<u8> = row.get(0)?.split('-').map(|x| x.parse().unwrap()).collect();
		let letter = row.get(1)?.get(0..1)?;
		let password = row.get(2)?;
		let mut count = 0;
		for char in password.chars() {
			if letter.contains(char) {
				count += 1;
			}
		}
		if count >= *numbers.get(0)? && count <= *numbers.get(1)? {
			valid_passwords += 1;
		}
	}
	println!("The result for the first part is: {}", valid_passwords);
	Some(())
}

fn part2(rows: &Vec<&str>) -> Option<()> {
	let mut valid_passwords: u16 = 0;
	for &row in rows {
		let row: Vec<&str> = row.split_whitespace().collect();
		let numbers: Vec<u8> = row.get(0)?.split('-').map(|x| x.parse().unwrap()).collect();
		let letter = row.get(1)?.get(0..1)?;
		let password: Vec<char> = row.get(2)?.chars().collect();
		let index = *numbers.get(0)? as usize - 1;
		let &word = password.get(index)?;
		let mut is_valid = false;
		if letter.contains(word) {
			is_valid = true;
		}
		let index = *numbers.get(1)? as usize - 1;
		let &word = password.get(index)?;
		if letter.contains(word) {
			is_valid = !is_valid;
		}
		if is_valid {
			valid_passwords += 1;
		}
	}
	println!("The result for the first part is: {}", valid_passwords);
	Some(())
}
