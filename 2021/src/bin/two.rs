use std::fs;

fn main() {
	let input: String = fs::read_to_string("2021/two").unwrap();
	let rows: Vec<&str> = input.split('\n').collect();
	part1(&rows).unwrap();
	part2(&rows).unwrap();
}

fn part1(rows: &Vec<&str>) -> Option<()> {
	let (mut horizontal, mut depth) = (0, 0);
	for x in rows {
		let pair: Vec<&str> = x.split_whitespace().collect();
		let direction: &str = pair.get(0)?;
		let value: i32 = pair.get(1)?.parse().unwrap();
		match direction {
			"forward" => horizontal += value,
			"down" => depth += value,
			"up" => depth -= value,
			_ => println!("This should not happen"),
		}
	}
	println!("The result for the first part is: {}", horizontal * depth);
	Some(())
}

fn part2(rows: &Vec<&str>) -> Option<()> {
	let (mut horizontal, mut depth, mut aim) = (0, 0, 0);
	for x in rows {
		let pair: Vec<&str> = x.split_whitespace().collect();
		let direction: &str = pair.get(0)?;
		let value: i32 = pair.get(1)?.parse().unwrap();
		match direction {
			"forward" => {
				horizontal += value;
				depth += aim * value;
			}
			"down" => aim += value,
			"up" => aim -= value,
			_ => println!("This should not happen"),
		}
	}
	println!("The result for the second part is: {}", horizontal * depth);
	Some(())
}
