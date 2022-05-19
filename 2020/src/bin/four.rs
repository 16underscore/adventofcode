use std::{collections::HashMap, fs};

fn main() {
	let input = fs::read_to_string("2020/four").unwrap();
	let passports: Vec<&str> = input.split("\n\n").collect();
	part1(&passports);
	part2(&passports);
}

fn part1(passports: &Vec<&str>) {
	let count = count_valid(passports, &is_valid_simple);
	println!("The result for the first part is: {}", count);
}

fn part2(passports: &Vec<&str>) {
	let count = count_valid(passports, &is_valid_extended);
	println!("The result for the first part is: {}", count);
}

fn count_valid(passports: &Vec<&str>, validate: &dyn Fn(&str) -> bool) -> u16 {
	let mut count: u16 = 0;
	for passport in passports {
		if validate(passport) {
			count += 1;
		}
	}
	count
}

fn is_valid_simple(passport: &str) -> bool {
	let keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
	for key in keys {
		if !passport.contains(key) {
			return false;
		}
	}
	true
}

fn is_valid_extended(passport: &str) -> bool {
	none_is_invalid(passport).is_some()
}

fn none_is_invalid(passport: &str) -> Option<()> {
	let fields: Vec<&str> = passport.split_whitespace().collect();
	let mut map: HashMap<&str, &str> = HashMap::new();
	for field in fields {
		let (key, value) = field.split_once(':').unwrap();
		map.insert(key, value);
	}
	let byr: u16 = map.get("byr")?.parse().unwrap();
	if byr < 1920 || byr > 2002 {
		return None;
	}
	let iyr: u16 = map.get("iyr")?.parse().unwrap();
	if iyr < 2010 || iyr > 2020 {
		return None;
	}
	let eyr: u16 = map.get("eyr")?.parse().unwrap();
	if eyr < 2020 || eyr > 2030 {
		return None;
	}
	let hgt = map.get("hgt")?;
	if hgt.ends_with("cm") {
		let hgt: u8 = hgt.strip_suffix("cm")?.parse().unwrap();
		if hgt < 150 || hgt > 193 {
			return None;
		}
	} else if hgt.ends_with("in") {
		let hgt: u8 = hgt.strip_suffix("in")?.parse().unwrap();
		if hgt < 59 || hgt > 76 {
			return None;
		}
	} else {
		return None;
	}
	let hcl = map.get("hcl")?;
	if !hcl.starts_with('#') {
		return None;
	}
	let hcl: Vec<char> = hcl.chars().collect();
	for character in hcl {
		if !"#0123456789abcdef".contains(character) {
			return None;
		}
	}
	let ecl = map.get("ecl")?;
	let values = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
	if !values.contains(ecl) {
		return None;
	}
	let pid = map.get("pid")?;
	if pid.len() != 9 {
		return None;
	}
	Some(())
}
