use std::fs;

fn main() {
	let input = fs::read_to_string("2020/nine").unwrap();
	let numbers: Vec<u64> = input
		.split('\n')
		.map(|number| number.parse().unwrap())
		.collect();
	part1(&numbers);
}

fn part1(numbers: &Vec<u64>) -> Option<()> {
	let preamble = 25;
	'label: for i in preamble..numbers.len() {
		for j in i - preamble..i {
			for k in j..i {
				let sum = numbers.get(j)? + numbers.get(k)?;
				if sum == *numbers.get(i)? {
					continue 'label;
				}
			}
		}
		println!("The result for the first part is: {}", *numbers.get(i)?);
		break;
	}
	Some(())
}
