use std::fs;

fn main() {
	let input = fs::read_to_string("2020/four").unwrap();
	let passports: Vec<&str> = input.split("\n\n").collect();
	part1(&passports);
}

fn part1(passports: &Vec<&str>) {
	let mut count: u16 = 0;
	for passport in passports {
		if is_valid(passport) {
			count += 1;
		}
	}
	println!("The result for the first part is: {}", count);
}

fn is_valid(passport: &str) -> bool {
	let keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
	for key in &keys {
		if !passport.contains(key) {
			return false;
		}
	}
	true
}