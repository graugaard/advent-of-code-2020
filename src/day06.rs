use std::collections::HashSet;
use std::iter::FromIterator;
use crate::puzzle_input;

pub fn get_answers_from_group(str: &str) -> Vec<HashSet<char>> {
		let mut vec = Vec::new();
		for answer in str.lines() {
				vec.push(HashSet::from_iter(answer.chars()))
		}

		vec
}

pub fn get_combined_answers_of_groups(str: &str) -> Vec<HashSet<char>> {
		let mut result = Vec::new();
		for group in str.split("\r\n\r\n") {
				let group = group.trim();
				if group.is_empty() {
						continue;
				}
				let answers = get_answers_from_group(group);

				let mut common: HashSet<char> = HashSet::from_iter("".chars());
				for set in answers {
						common = HashSet::from_iter(set.union(&common).map(|c| *c));
				}
				result.push(common)
		}
		result
}

pub fn get_common_answer_of_groups(str: &str) -> Vec<HashSet<char>> {
		let mut result = Vec::new();
		for group in str.split("\r\n\r\n") {
				let group = group.trim();
				if group.is_empty() {
						continue;
				}
				let answers = get_answers_from_group(group);

				let mut common: HashSet<char> = HashSet::from_iter("abcdefghijklmnopqrstuvwxyz".chars());
				for set in answers {
						common = HashSet::from_iter(set.intersection(&common).map(|c| *c));
				}
				result.push(common)
		}
		result
}

pub fn print_solution() {
		let puzzle = puzzle_input::read_input("day06");

		println!("Day 06 Solution Part 1: {}", sum_answers(&get_combined_answers_of_groups(&puzzle)));

		let vec = get_common_answer_of_groups(&puzzle);
		println!("Day 06 Solution Part 2: {}", sum_answers(&vec));
}

pub fn sum_answers(answers: &[HashSet<char>]) -> usize {
		let mut sum = 0;
		for common in answers {
				sum += common.len();
		}

		sum
}

#[cfg(test)]
mod tests {
		use crate::day06::{get_answers_from_group, get_combined_answers_of_groups, sum_answers, get_common_answer_of_groups};
		use std::collections::HashSet;
		use std::iter::FromIterator;

		#[test]
		fn a_group_of_one_member() {
				assert_eq!(get_answers_from_group("abcd"), vec![HashSet::from_iter(vec!['a', 'b', 'c', 'd'])]);
		}

		#[test]
		fn a_group_of_multiple_members() {
				assert_eq!(
						get_answers_from_group("abc\ncb\r\nfge"),
						vec![
								HashSet::from_iter(vec!['a', 'b', 'c']),
								HashSet::from_iter(vec!['c', 'b']),
								HashSet::from_iter(vec!['f', 'g', 'e'])
						]
				)
		}

		#[test]
		fn common_answers_of_single_group() {
				assert_eq!(
						get_combined_answers_of_groups("abcdefg\r\nbef\nbfx"),
						vec![
								HashSet::from_iter(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'x'])
						]
				)
		}

		#[test]
		fn common_answers_of_multiple_groups() {
				assert_eq!(
						get_combined_answers_of_groups("abc\r\nncb\r\n\r\ndefg\r\ngh\r\n\r\nxyz\r\nyz\r\nx"),
						vec![
								HashSet::from_iter(vec!['a', 'c', 'b', 'n']),
								HashSet::from_iter(vec!['d', 'e', 'f', 'g', 'h']),
								HashSet::from_iter(vec!['x', 'y', 'z'])
						]
				)
		}

		#[test]
		fn sum_of_answers() {
				let common = get_combined_answers_of_groups("abc\r\n\r\na\r\nb\r\nc\r\n\r\nab\r\nac\r\n\r\na\r\na\r\na\r\na\r\n\r\nb");
				assert_eq!(
						sum_answers(&common),
						11
				)
		}

		#[test]
		fn sum_common_answers() {
				let common = get_common_answer_of_groups("abc\r\n\r\na\r\nb\r\nc\r\n\r\nab\r\nac\r\n\r\na\r\na\r\na\r\na\r\n\r\nb");
				assert_eq!(
						sum_answers(&common),
						6
				)
		}
}