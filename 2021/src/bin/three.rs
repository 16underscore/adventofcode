use std::fs;

fn main() {
	let input = fs::read_to_string("three").unwrap();
	let rows: Vec<u32> = input
		.split('\n')
		.map(|x| u32::from_str_radix(x, 2).unwrap())
		.collect();

	part1(&rows);
	part2(&rows);
}

fn part1(rows: &Vec<u32>) {
	let (mut gamma, mut epsilon): (u32, u32) = (0, 0);
	let half = rows.len() / 2;

	for i in 0..12 {
		let column = 0b1 << i;

		let counter = column_counter(&rows, column, false);

		if counter > half {
			gamma |= column;
		} else if counter < half {
			epsilon |= column;
		}
	}
	println!("The result for the first part is: {}", gamma * epsilon);
}

fn part2(rows: &Vec<u32>) {
	let mut oxygen = rows.clone();
	let mut co2 = rows.to_owned();

	for i in 0..12 {
		let column = 0b1 << i;

		let oxygen_counter = column_counter(&oxygen, column, false);
		let co2_counter = column_counter(&co2, column, true);

		if oxygen.len() > 1 {
			if oxygen_counter >= oxygen.len() / 2 {
				oxygen.retain(|&x| x & column == column);
			} else {
				oxygen.retain(|&x| x & column != column);
			}
		}

		if co2.len() > 1 {
			if co2_counter >= co2.len() / 2 {
				co2.retain(|&x| x & column != column);
			} else {
				co2.retain(|&x| x & column == column);
			}
		}
	}
	if oxygen.len() == 1 && co2.len() == 1 {
		let (&oxygen, &co2) = (oxygen.get(0).unwrap(), co2.get(0).unwrap());
		println!("The result for the second part is: {}", oxygen * co2);
	} else {
		println!("There is something wrong!");
		println!("oxygen length: {} co2 length: {}", oxygen.len(), co2.len());
	}
}

fn column_counter(rows: &Vec<u32>, column: u32, convert: bool) -> usize {
	let mut counter: usize = 0;
	for &row in rows {
		if row & column == column && !convert {
			counter += 1;
		} else if row & column != column && convert {
			counter += 1;
		}
	}
	counter
}
