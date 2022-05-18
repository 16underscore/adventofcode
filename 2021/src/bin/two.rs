use std::fs;

fn main() {
	let input: String = fs::read_to_string("2021/two").unwrap();
	part1(&input);
	part2(&input);
}

fn part1(input: &String) {
	let (mut horizontal, mut depth) = (0, 0);
	input.split('\n').for_each(|x| {
		let pair: Vec<&str> = x.split_whitespace().collect();
		let direction: &str = pair.get(0).unwrap();
		let value: i32 = pair.get(1).unwrap().parse().unwrap();
		match direction {
			"forward" => horizontal += value,
			"down" => depth += value,
			"up" => depth -= value,
			_ => println!("This should not happen"),
		}
	});
	println!("The result for the first part is: {}", horizontal * depth);
}
fn part2(input: &String) {
	let (mut horizontal, mut depth) = (0, 0);
	let mut aim = 0;
	input.split('\n').for_each(|x| {
		let pair: Vec<&str> = x.split_whitespace().collect();
		let direction: &str = pair.get(0).unwrap();
		let value: i32 = pair.get(1).unwrap().parse().unwrap();
		match direction {
			"forward" => {
				horizontal += value;
				depth += aim * value;
			}
			"down" => aim += value,
			"up" => aim -= value,
			_ => println!("This should not happen"),
		}
	});
	println!("The result for the second part is: {}", horizontal * depth);
}
