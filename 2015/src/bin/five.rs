use std::fs;

fn main() {
	let input = fs::read_to_string("2015/five").unwrap();
	let rows: Vec<&str> = input.split('\n').collect();
	part1(&rows);
	part2(&rows);
}

fn part1(rows: &Vec<&str>) {
	let mut nice: u16 = 0;
	for row in rows {
		let mut vowels: u8 = 0;
		for character in row.chars() {
			match character {
				'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
				_ => continue,
			}
		}
		if vowels < 3 {
			continue;
		}
		if !has_double_letter(row) {
			continue;
		}
		if row.contains("ab") || row.contains("cd") || row.contains("pq") || row.contains("xy") {
			continue;
		}
		nice += 1;
	}
	println!("The result for the first part is: {}", nice);
}

fn part2(rows: &Vec<&str>) {
	let mut nice: u16 = 0;
	for row in rows {
		if !has_pair(row) {
			continue;
		}
		if !repeats_with_one_letter_between(row) {
			continue;
		}
		nice += 1;
	}
	println!("The result for the second part is: {}", nice);
}

fn has_double_letter(row: &str) -> bool {
	for i in 0..row.len() - 1 {
		if row.get(i..i + 1) == row.get(i + 1..i + 2) {
			return true;
		}
	}
	false
}

fn has_pair(row: &str) -> bool {
	for i in 0..row.len() - 1 {
		for j in i + 2..row.len() - 1 {
			if row.get(i..i + 2) == row.get(j..j + 2) {
				return true;
			}
		}
	}
	false
}

fn repeats_with_one_letter_between(row: &str) -> bool {
	for i in 0..row.len() - 2 {
		if row.get(i..i + 1) == row.get(i + 2..i + 3) {
			return true;
		}
	}
	false
}
