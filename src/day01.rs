use std::fs;

pub fn print_solution() {
		let input = puzzle_input();
		let indexes = find_first_sum_to(&input, 2020);
		println!("Day 01 Solution Part 1: {}", input[indexes.0] * input[indexes.1]);

		let indexes = find_first_3_sum_to(&input, 2020);
		println!("Day 01 Solution Part 2: {}", input[indexes.0] * input[indexes.1] * input[indexes.2]);
}

pub fn find_first_sum_to(values: &[i64], target: i64) -> (usize, usize) {
		for i in 0..(values.len() - 1) {
				for j in i + 1..values.len() {
						if target == values[i] + values[j] {
								return (i, j);
						}
				}
		}
		(0, 0)
}


pub fn find_first_3_sum_to(values: &[i64], target: i64) -> (usize, usize, usize) {
		for i in 0..values.len() - 2 {
				for j in i + 1..values.len() - 1 {
						for k in j + 1..values.len() {
								if target == values[i] + values[j] + values[k] {
										return (i, j, k);
								}
						}
				}
		}
		(0, 1, 2)
}

fn puzzle_input() -> Vec<i64> {
		let contents = fs::read_to_string("input/day01.txt").expect("Could not read file day01.txt");

		contents.split_whitespace()
						.map(|s| s.parse::<i64>())
						.map(|res| res.unwrap())
						.collect()
}

#[cfg(test)]
mod tests {
		use crate::day01::{find_first_sum_to, find_first_3_sum_to};

		#[test]
		fn values_1_2_3_and_target_2_gives_0_0() {
				let values = [1, 2, 3];
				assert_eq!(find_first_sum_to(&values, 2), (0, 0));
		}

		#[test]
		fn values_1_4_5_and_target_9_gives_1_2() {
				let values = [1, 4, 5];
				assert_eq!(find_first_sum_to(&values, 9), (1, 2))
		}

		#[test]
		fn values_1_4_10_13_sum_to_15_gives_0_1_2() {
				let values = [1, 4, 10, 13];
				assert_eq!(find_first_3_sum_to(&values, 15), (0, 1, 2));
		}

		#[test]
		fn values_1_100_213_14_121_13_11223_5_target_20_gives_0_3_7() {
				let values = [1, 100, 213, 14, 121, 13, 11223, 5];
				assert_eq!(find_first_3_sum_to(&values, 20), (0, 3, 7));
		}
}