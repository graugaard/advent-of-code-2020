use std::fs;

pub fn season_greetings() {
		println!("Merry christmas and happy holidays!");
}

pub fn print_solution() {
		let input = puzzle_input();
		let indexes = find_first_sum_to(&input, 2020);
		println!("Day 01 Solution: {}", input[indexes.0] * input[indexes.1]);
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

fn puzzle_input() -> Vec<i64> {
		let contents = fs::read_to_string("input/day01.txt").expect("Could not read file day01.txt");

		contents.split_whitespace()
						.map(|s| s.parse::<i64>())
						.map(|res| res.unwrap())
						.collect()
}

#[cfg(test)]
mod tests {
		use crate::day01::{find_first_sum_to, increment_indicies};

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
}