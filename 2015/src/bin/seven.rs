use std::{collections::HashMap, fs};

fn main() {
	let input = fs::read_to_string("2015/seven").unwrap();
	let instructions: Vec<&str> = input.split('\n').collect();
	part1(&instructions);
}

fn part1(instructions: &Vec<&str>) {
	let mut map: HashMap<&str, &str> = HashMap::new();
	for instruction in instructions {
		let (value, key) = instruction.split_once(" -> ").unwrap();
		map.insert(key, value);
	}
	let signal = recursion(&map, "a");
	println!("The result for the first part is: {}", signal);
}

fn recursion(map: &HashMap<&str, &str>, key: &str) -> u32 {
	let value = map.get(key).expect("MAP ERROR");
	return if value.contains("AND") {
		let (l_value, r_value) = value.split_once(" AND ").expect("AND NONE");
		match l_value.parse::<u32>() {
			Ok(l_value) => l_value & recursion(map, r_value),
			Err(_) => recursion(map, l_value) & recursion(map, r_value),
		}
	} else if value.contains("OR") {
		let (l_value, r_value) = value.split_once(" OR ").expect("OR NONE");
		recursion(map, l_value) | recursion(map, r_value)
	} else if value.contains("LSHIFT") {
		let (l_value, r_value) = value.split_once(" LSHIFT ").expect("LSHIFT NONE");
		let r_value: u32 = r_value.parse().expect("LSHIFT PARSE ERROR");
		recursion(map, l_value) << r_value
	} else if value.contains("RSHIFT") {
		let (l_value, r_value) = value.split_once(" RSHIFT ").expect("RSHIFT NONE");
		let r_value: u32 = r_value.parse().expect("RSHIFT PARSE ERROR");
		recursion(map, l_value) >> r_value
	} else if value.contains("NOT") {
		let value = value.replace("NOT ", "");
		!recursion(map, &value)
	} else {
		match value.trim().parse::<u32>() {
			Ok(value) => value,
			Err(_) => recursion(map, value),
		}
	};
}
