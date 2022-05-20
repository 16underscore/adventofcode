use std::fs;

fn main() {
	let input = fs::read_to_string("2020/five").unwrap();
	let rows: Vec<&str> = input.split('\n').collect();
	part1(&rows);
	part2(&rows);
}

fn part1(rows: &Vec<&str>) {
	let mut highest: u16 = 0;
	for seat in get_seats(rows) {
		if seat > highest {
			highest = seat;
		}
	}
	println!("The result for the first part is: {}", highest);
}

fn part2(rows: &Vec<&str>) {
	let mut seats = get_seats(rows);
	seats.sort();
	for i in 1..seats.len() - 1 {
		let previous = seats.get(i - 1).unwrap();
		let current = seats.get(i).unwrap();
		let next = seats.get(i + 1).unwrap();
		let expected = (previous + next) / 2;
		if expected != *current {
			println!("The result for the second part is: {}", expected);
			return;
		}
	}
}

fn get_seats(rows: &Vec<&str>) -> Vec<u16> {
	let mut seats: Vec<u16> = Vec::new();
	for row in rows {
		let (row, column) = row.split_at(7);
		let row = &row.replace('F', "0");
		let row = &row.replace('B', "1");
		let row = u16::from_str_radix(row, 2).unwrap();
		let column = &column.replace('L', "0");
		let column = &column.replace('R', "1");
		let column = u16::from_str_radix(column, 2).unwrap();
		seats.push(row * 8 + column);
	}
	seats
}
