use std::fs;

fn main() {
	let input = fs::read_to_string("2020/one").unwrap();
	let rows: Vec<u32> = input.split('\n').map(|x| x.parse().unwrap()).collect();
	part1(&rows);
	part2(&rows);
}

fn part1(rows: &Vec<u32>) {
	for &i_row in rows {
		for &j_row in rows {
			if i_row + j_row == 2020 {
				println!("The result for the first part is: {}", i_row * j_row);
				return;
			}
		}
	}
}

fn part2(rows: &Vec<u32>) {
	for &i_row in rows {
		for &j_row in rows {
			for &k_row in rows {
				if i_row + j_row + k_row == 2020 {
					println!("The result for the second part is: {}", i_row * j_row * k_row);
					return;
				}
			}
		}
	}
}
