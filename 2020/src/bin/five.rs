use std::fs;

fn main() {
	let input = fs::read_to_string("2020/five").unwrap();
	let rows: Vec<&str> = input.split('\n').collect();
	part1(&rows);
}

fn part1(rows: &Vec<&str>) {
	let mut highest: u16 = 0;
	for row in rows {
		let (row, column) = row.split_at(7);
		let row = &row.replace('F', "0");
		let row = &row.replace('B', "1");
		let row = u16::from_str_radix(row, 2).unwrap();
		let column = &column.replace('L', "0");
		let column = &column.replace('R', "1");
		let column = u16::from_str_radix(column, 2).unwrap();
		let id: u16 = row * 8 + column;
		if id > highest {
			highest = id;
		}
	}
	println!("The result for the first part is: {}", highest);
}
