use std::fs;

fn main() {
	let mut input = fs::read_to_string("2020/seven").unwrap();
	input = input.replace(" bags", "");
	input = input.replace(" bag", "");
	let bags: Vec<&str> = input.split(".\n").collect();
	part1(&bags);
}

fn part1(bags: &Vec<&str>) {
	let count = search(bags, &Vec::from(["shiny gold"]));
	println!("The result for the first part is: {}", count);
}

fn search(bags: &Vec<&str>, should_contain_bags: &Vec<&str>) -> u16 {
	let mut bags_with_specified_bags: Vec<&str> = Vec::new();
	for bag in bags {
		let (bag_name, containing_bags) = bag.split_once(" contain ").unwrap();
		for should_contain_bag in should_contain_bags {
			if containing_bags.contains(should_contain_bag) {
				bags_with_specified_bags.push(bag_name);
			}
		}
	}
	if !bags_with_specified_bags.is_empty() {
		search(bags, &bags_with_specified_bags) + bags_with_specified_bags.len() as u16
	} else {
		0
	}
}
