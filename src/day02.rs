use crate::puzzle_input;

pub struct PasswordChecker {
		c: char,
		range: (usize, usize)
}

impl PasswordChecker {
		fn new(config: &str) -> PasswordChecker {
				let conf = to_configuration(config);
				PasswordChecker {
						c: conf.0,
						range: (conf.1, conf.2)
				}
		}

		fn check(&self, password: &str) -> bool {
				let chars = password.chars()
												.filter(|c| self.c == *c)
												.count();

				self.range.0 <= chars && chars <= self.range.1
		}

		fn check_at_positions(&self, password: &str) -> bool {
				let chars: Vec<char> = password.chars().collect();

				chars[self.range.0 - 1] == self.c && chars[self.range.1 - 1] != self.c
				|| chars[self.range.0 - 1] != self.c && chars[self.range.1 - 1] == self.c
		}
}

pub fn to_configuration(config: &str) -> (char, usize, usize) {
		let parts: Vec<&str> = config.split_ascii_whitespace().collect();

		let range: Vec<usize> = parts[0].split("-")
																		.map(|c| c.parse::<usize>())
																		.map(|end| end.unwrap())
																		.collect();

		(parts[1].chars().next().unwrap(), range[0], range[1])
}

pub fn check_password(password: &str) -> bool {
		let password_split: Vec<&str> = password.split(":").collect();

		let checker = PasswordChecker::new(password_split[0]);

		checker.check(password_split[1].trim())
}

pub fn test_with_position(password: &str) -> bool {
		let password_split: Vec<&str> = password.split(":").collect();

		let checker = PasswordChecker::new(password_split[0]);

		checker.check_at_positions(password_split[1].trim())
}

pub fn print_solution() {
		let passwords = puzzle_input::read_input("day02");
		let n_valid = passwords.split("\n")
													 .filter(|password| check_password(password))
													 .count();

		println!("Day 02 Solution Part 1: {}", n_valid);

		let n_valid = passwords.split("\n")
			.filter(|password| test_with_position(password))
			.count();

		println!("Day 02 Solution Part 2: {}", n_valid)
}

#[cfg(test)]
mod tests {
		use crate::day02::{to_configuration, PasswordChecker, check_password, test_with_position};

		#[test]
		fn the_config_a_1_2_gives_a_1_2() {
				assert_eq!(to_configuration("1-2 a"), ('a', 1, 2));
		}

		#[test]
		fn the_config_c_1_2_gives_c_1_2() {
				assert_eq!(to_configuration("1-2 c"), ('c', 1, 2));
		}

		#[test]
		fn the_config_d_11_20_gives_d_11_20() {
				assert_eq!(to_configuration("11-20 d"), ('d', 11, 20));
		}

		#[test]
		fn test_aad_matches_0_1_d() {
				let checker = PasswordChecker::new("0-1 d");
				assert_eq!(checker.check("aad"), true);
		}

		#[test]
		fn test_ccdd_does_not_match_3_4_c() {
				let checker = PasswordChecker::new("3-4 c");
				assert_eq!(checker.check("ccdd"), false);
		}

		#[test]
		fn test_full_config_of_password() {
				assert!(check_password("3-4 c: cccdd"));
		}

		#[test]
		fn test_config_with_position_with_invalid_check() {
				let checker = PasswordChecker::new("1-2 c");
				assert_eq!(checker.check_at_positions("ccc"), false);
		}

		#[test]
		fn test_config_with_position_check_with_valid_password() {
				let checker = PasswordChecker::new("1-4 b");
				assert_eq!(checker.check_at_positions("bbbab"), true);
		}

		#[test]
		fn part2_example1() {
				assert_eq!(test_with_position("1-3 a: abcde"), true);
		}
}