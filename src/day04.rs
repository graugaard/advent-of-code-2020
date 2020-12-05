use std::collections::{HashMap, HashSet};
use crate::puzzle_input;

const VALID_EYE_COLORS: [&str; 7] = [
		"amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

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

fn get_mandatory_fields() -> HashSet<String> {
		vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].into_iter()
																												 .map(|s| s.to_string())
																												 .collect()
}

pub fn is_valid_passport(passport: &HashMap<String, String>) -> bool {
		has_valid_birth_year(passport)
			&& has_valid_issue_year(passport)
			&& has_valid_expiration_year(passport)
			&& has_valid_height(passport)
			&& has_valid_hair_color(passport)
			&& has_valid_eye_color(passport)
			&& has_valid_passport_id(passport)
}

pub fn has_mandatory_fields(passport: &HashMap<String, String>) -> bool {
		for field in get_mandatory_fields() {
				if !passport.contains_key(&field) {
						return false;
				}
		}
		true
}

fn has_valid_birth_year(passport: &HashMap<String, String>) -> bool {
		match passport.get("byr") {
				Some(year) => {
						let year = match year.parse::<i32>() {
								Ok(year) => year,
								Err(_) => 0,
						};
						1920 <= year && year <= 2002
				},
				None => false,
		}
}

fn has_valid_issue_year(passport: &HashMap<String, String>) -> bool {
		match passport.get("iyr") {
				Some(year) => {
						let year = match year.parse::<i32>() {
								Ok(year) => year,
								Err(_) => 0,
						};

						2010 <= year && year <= 2020
				}
				None => false
		}
}

fn has_valid_expiration_year(passport: &HashMap<String, String>) -> bool {
		match passport.get("eyr") {
				Some(year) => {
						let year = match year.parse::<i32>() {
								Ok(year) => year,
								Err(_) => 0,
						};
						2020 <= year && year <= 2030
				},
				None => false,
		}
}

fn has_valid_height(passport: &HashMap<String, String>) -> bool {
		match passport.get("hgt") {
				Some(hgt) => {
						let height = match hgt[0..hgt.len() - 2].parse::<i32>() {
								Ok(height) => height,
								Err(_) => 0,
						};
						if hgt[hgt.len() - 2..].starts_with("cm") {
								150 <= height && height <= 193
						} else if hgt[hgt.len() - 2..].starts_with("in") {
								59 <= height && height <= 76
						} else {
								false
						}
				},
				None => false,
		}
}

fn has_valid_hair_color(passport: &HashMap<String, String>) -> bool {
		match passport.get("hcl") {
				Some(hcl) => {
						if hcl.len() != 7 {
								return false;
						}
						if !hcl.starts_with('#') {
								return false;
						}
						for char in hcl[1..].chars() {
								if !char.is_ascii_hexdigit() {
										return false
								}
						}
						true
				},
				None => false
		}
}

fn has_valid_eye_color(passport: &HashMap<String, String>) -> bool {
		match passport.get("ecl") {
				Some(color) => {
						VALID_EYE_COLORS.contains(&color.as_str())
				},
				None => false,
		}
}

fn has_valid_passport_id(passport: &HashMap<String, String>) -> bool {
		match passport.get("pid") {
				Some(pid) => {
						if pid.len() != 9 {
								return false;
						}
						match pid.parse::<u64>() {
								Ok(_) => true,
								Err(_) => false,
						}
				},
				None => false
		}
}

pub fn print_solution() {
		let puzzle = puzzle_input::read_input("day04");

		println!("Day 04 Solution Part 1: {}", count_with_filter(&puzzle, |passport| has_mandatory_fields(passport)));
		println!("Day 04 Solution Part 2: {}", count_with_filter(&puzzle, |passport| is_valid_passport(passport)));
}

pub fn count_with_filter<T>(input: &str, f: T) -> usize
		where T: Fn(&HashMap<String, String>) -> bool
{
		input.split("\r\n\r\n")
				 .map(|line| process_line(line))
				 .filter(f)
				 .count()
}

#[cfg(test)]
mod tests {
		use crate::day04::{process_line, is_valid_passport, has_mandatory_fields, count_with_filter};
		use std::collections::HashMap;

		#[test]
		fn test_validating_byr() {
				let mut passport = HashMap::new();
				passport.insert("iyr".to_string(), "2010".to_string());
				passport.insert("eyr".to_string(), "2020".to_string());
				passport.insert("hgt".to_string(), "150cm".to_string());
				passport.insert("hcl".to_string(), "#ffffff".to_string());
				passport.insert("ecl".to_string(), "amb".to_string());
				passport.insert("pid".to_string(), "000111222".to_string());
				assert_eq!(is_valid_passport(&passport), false);

				passport.insert("byr".to_string(), "1920".to_string());

				assert_eq!(is_valid_passport(&passport), true);

				passport.insert("byr".to_string(), "1919".to_string());
				assert_eq!(is_valid_passport(&passport), false);

				passport.insert("byr".to_string(), "2003".to_string());
				assert_eq!(is_valid_passport(&passport), false);
		}

		#[test]
		fn validating_iyr() {
				let mut passport = HashMap::new();
				passport.insert("byr".to_string(), "1920".to_string());
				passport.insert("eyr".to_string(), "2020".to_string());
				passport.insert("hgt".to_string(), "150cm".to_string());
				passport.insert("hcl".to_string(), "#ffffff".to_string());
				passport.insert("ecl".to_string(), "amb".to_string());
				passport.insert("pid".to_string(), "000111222".to_string());

				assert_eq!(is_valid_passport(&passport), false);


				passport.insert("iyr".to_string(), "2010".to_string());
				assert_eq!(is_valid_passport(&passport), true);

				passport.insert("iyr".to_string(), "2009".to_string());
				assert_eq!(is_valid_passport(&passport), false);

				passport.insert("iyr".to_string(), "2021".to_string());
				assert_eq!(is_valid_passport(&passport), false);
		}

		#[test]
		fn validating_eyr() {
				let mut passport = HashMap::new();
				passport.insert("byr".to_string(), "1920".to_string());
				passport.insert("iyr".to_string(), "2010".to_string());
				passport.insert("hgt".to_string(), "150cm".to_string());
				passport.insert("hcl".to_string(), "#ffffff".to_string());
				passport.insert("ecl".to_string(), "amb".to_string());
				passport.insert("pid".to_string(), "000111222".to_string());

				assert_eq!(is_valid_passport(&passport), false);

				passport.insert("eyr".to_string(), "2020".to_string());
				assert_eq!(is_valid_passport(&passport), true);

				passport.insert("eyr".to_string(), "2019".to_string());
				assert_eq!(is_valid_passport(&passport), false);

				passport.insert("eyr".to_string(), "2031".to_string());
				assert_eq!(is_valid_passport(&passport), false);
		}

		#[test]
		fn validating_hgt() {
				let mut passport = HashMap::new();
				passport.insert("byr".to_string(), "1920".to_string());
				passport.insert("iyr".to_string(), "2010".to_string());
				passport.insert("eyr".to_string(), "2020".to_string());
				passport.insert("hcl".to_string(), "#ffffff".to_string());
				passport.insert("ecl".to_string(), "amb".to_string());
				passport.insert("pid".to_string(), "000111222".to_string());
				assert_eq!(is_valid_passport(&passport), false);


				passport.insert("hgt".to_string(), "150km".to_string());
				assert_eq!(is_valid_passport(&passport), false);

				passport.insert("hgt".to_string(), "150cm".to_string());
				assert_eq!(is_valid_passport(&passport), true);

				passport.insert("hgt".to_string(), "149cm".to_string());
				assert_eq!(is_valid_passport(&passport), false);

				passport.insert("hgt".to_string(), "194cm".to_string());
				assert_eq!(is_valid_passport(&passport), false);

				passport.insert("hgt".to_string(), "59in".to_string());
				assert_eq!(is_valid_passport(&passport), true);

				passport.insert("hgt".to_string(), "58in".to_string());
				assert_eq!(is_valid_passport(&passport), false);

				passport.insert("hgt".to_string(), "77in".to_string());
				assert_eq!(is_valid_passport(&passport), false);
		}

		#[test]
		fn validate_hcl() {
				let mut passport = HashMap::new();
				passport.insert("byr".to_string(), "1920".to_string());
				passport.insert("iyr".to_string(), "2010".to_string());
				passport.insert("eyr".to_string(), "2020".to_string());
				passport.insert("hgt".to_string(), "150cm".to_string());

				passport.insert("ecl".to_string(), "amb".to_string());
				passport.insert("pid".to_string(), "000111222".to_string());
				assert_eq!(is_valid_passport(&passport), false);

				passport.insert("hcl".to_string(), "#ffffff".to_string());
				assert_eq!(is_valid_passport(&passport), true);

				passport.insert("hcl".to_string(), "#12345".to_string());
				assert_eq!(is_valid_passport(&passport), false);

				passport.insert("hcl".to_string(), "tffffff".to_string());
				assert_eq!(is_valid_passport(&passport), false);

				passport.insert("hcl".to_string(), "#12321g".to_string());
				assert_eq!(is_valid_passport(&passport), false);
		}

		#[test]
		fn validate_ecl() {
				let mut passport = HashMap::new();
				passport.insert("byr".to_string(), "1920".to_string());
				passport.insert("iyr".to_string(), "2010".to_string());
				passport.insert("eyr".to_string(), "2020".to_string());
				passport.insert("hgt".to_string(), "150cm".to_string());
				passport.insert("hcl".to_string(), "#ffffff".to_string());
				passport.insert("pid".to_string(), "000111222".to_string());

				assert_eq!(is_valid_passport(&passport), false);

				passport.insert("ecl".to_string(), "amb".to_string());
				assert_eq!(is_valid_passport(&passport), true);

				passport.insert("ecl".to_string(), "red".to_string());
				assert_eq!(is_valid_passport(&passport), false);
		}

		#[test]
		fn test_validate_pid() {
				let mut passport = HashMap::new();
				passport.insert("byr".to_string(), "1920".to_string());
				passport.insert("iyr".to_string(), "2010".to_string());
				passport.insert("eyr".to_string(), "2020".to_string());
				passport.insert("hgt".to_string(), "150cm".to_string());
				passport.insert("hcl".to_string(), "#ffffff".to_string());
				passport.insert("ecl".to_string(), "amb".to_string());

				assert_eq!(is_valid_passport(&passport), false);

				passport.insert("pid".to_string(), "000111222".to_string());
				assert_eq!(is_valid_passport(&passport), true);

				passport.insert("pid".to_string(),"12345678".to_string());
				assert_eq!(is_valid_passport(&passport), false);

				passport.insert("pid".to_string(), "aaabbbccc".to_string());
				assert_eq!(is_valid_passport(&passport), false);
		}

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
				valid.insert("byr".to_string(), "121".to_string());
				valid.insert("iyr".to_string(), "1".to_string());
				valid.insert("eyr".to_string(), "12".to_string());
				valid.insert("hgt".to_string(), "1231".to_string());
				valid.insert("hcl".to_string(), "red".to_string());
				valid.insert("ecl".to_string(), "blue".to_string());
				valid.insert("pid".to_string(), "11-22-4".to_string());
				assert_eq!(has_mandatory_fields(&valid), true);
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

				assert_eq!(has_mandatory_fields(&invalid), false);
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
				assert_eq!(count_with_filter(input, |passport| has_mandatory_fields(passport)), 2);
		}
}