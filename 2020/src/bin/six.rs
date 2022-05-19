use std::fs;

fn main() {
	let input = fs::read_to_string("2020/six").unwrap();
	let groups: Vec<&str> = input.split("\n\n").collect();
	part1(&groups);
	part2(&groups);
}

fn part1(groups: &Vec<&str>) {
	let mut sum = 0;
	for group in groups {
		let mut group: Vec<char> = group.replace('\n', "").chars().collect();
		group.sort();
		group.dedup();
		sum += group.len();
	}
	println!("The result for the first part is: {}", sum);
}

fn part2(groups: &Vec<&str>) {
	let mut sum: u16 = 0;
	for group in groups {
		let people: Vec<&str> = group.split('\n').collect();
		let mut most_yes: &str = "";
		for person in &people {
			if person.len() > most_yes.len() {
				most_yes = person;
			}
		}
		let most_yes: Vec<char> = most_yes.chars().collect();
		let mut yes_counter: u16 = 0;
		'next_question: for yes_answer in most_yes {
			for person in &people {
				if !person.contains(yes_answer) {
					continue 'next_question;
				}
			}
			yes_counter += 1;
		}
		sum += yes_counter;
	}
	println!("The result for the second part is: {}", sum);
}
