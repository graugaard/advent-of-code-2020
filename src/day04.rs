use std::collections::{HashMap, HashSet};
use crate::puzzle_input;

pub fn process_line(line: &str) -> HashMap<String, String> {
		let mut map = HashMap::new();

		for pair in line.split_ascii_whitespace() {
				let split: Vec<String> = pair.split(":")
					.map(|s| s.to_string())
					.collect();

				map.insert(split[0].to_string(), split[1].to_string());
		}

		map
}

pub fn get_mandatory_fields() -> HashSet<String> {
		vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].into_iter()
			.map(|s| s.to_string())
			.collect()
}

pub fn is_valid_map<T>(map: &HashMap<String, T>) ->bool {
		for field in get_mandatory_fields() {
				if !map.contains_key(&field) {
						return false;
				}
		}
		true
}

pub fn print_solution() {
		let puzzle = puzzle_input::read_input("day04");
		println!("Maps: {}", puzzle.split("\n\n")
															.map(|line| process_line(line)).count());
		println!("Day 04 Solution Part 1: {}", count_valid_passports(&puzzle));
}

pub fn count_valid_passports(input: &str) -> usize {
		input.split("\r\n\r\n")
			.map(|line| process_line(line))
			.filter(|passport| is_valid_map(passport))
			.count()
}
#[cfg(test)]
mod tests {
		use crate::day04::{process_line, is_valid_map, count_valid_passports};
		use std::collections::HashMap;

		#[test]
		fn test_proccess_line() {
				let map = process_line("abc:efg key1:123\n        key:space");
				assert_eq!(map.get("abc"), Some(&"efg".to_string()));
				assert_eq!(map.get("key1"), Some(&"123".to_string()));
				assert_eq!(map.get("key"), Some(&"space".to_string()));
		}

		#[test]
		fn test_is_valid() {
				let mut valid = HashMap::new();
				valid.insert("byr".to_string(), "1".to_string());
				valid.insert("iyr".to_string(), "1".to_string());
				valid.insert("eyr".to_string(), "12".to_string());
				valid.insert("hgt".to_string(), "1231".to_string());
				valid.insert("hcl".to_string(), "red".to_string());
				valid.insert("ecl".to_string(), "blue".to_string());
				valid.insert("pid".to_string(), "11-22-4".to_string());

				assert_eq!(is_valid_map(&valid), true);
		}

		#[test]
		fn test_is_invalid() {
				let mut invalid = HashMap::new();
				invalid.insert("iyr".to_string(), "1".to_string());
				invalid.insert("eyr".to_string(), "12".to_string());
				invalid.insert("hgt".to_string(), "1231".to_string());
				invalid.insert("hcl".to_string(), "red".to_string());
				invalid.insert("ecl".to_string(), "blue".to_string());
				invalid.insert("pid".to_string(), "11-22-4".to_string());

				assert_eq!(is_valid_map(&invalid), false);
		}

		#[test]
		fn test_count_valid_passports() {
				let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\r\n\
byr:1937 iyr:2017 cid:147 hgt:183cm\r\n\
\r\n\
iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\r\n\
hcl:#cfa07d byr:1929\r\n\
\r\n\
hcl:#ae17e1 iyr:2013\r\n\
eyr:2024\r\n\
ecl:brn pid:760753108 byr:1931\r\n\
hgt:179cm\r\n\
\r\n\
hcl:#cfa07d eyr:2025 pid:166559648\r\n\
iyr:2011 ecl:brn hgt:59in";
				assert_eq!(count_valid_passports(input), 2);
		}
}