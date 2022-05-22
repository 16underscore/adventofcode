use std::fs;

fn main() {
	let input = fs::read_to_string("2021/four").unwrap();
	let mut bingo: Vec<&str> = input.split("\n\n").collect();
	let _bingo_numbers: Vec<u8> = bingo
		.remove(0)
		.split(',')
		.map(|num| num.parse().unwrap())
		.collect();
}
