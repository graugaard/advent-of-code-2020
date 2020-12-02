use std::fs;

pub fn read_input(puzzle: &str) -> String {
		let mut filename = String::from("input/");

		filename.push_str(puzzle);
		filename.push_str(".txt");

		fs::read_to_string(filename).expect("Could not read puzzle file")
}