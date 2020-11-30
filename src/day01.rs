pub fn season_greetings() {
	println!("Merry christmas and happy holidays!");
}

#[cfg(test)]
mod tests {
		#[test]
		fn basic_test() {
				assert_eq!(2 + 2, 4);
		}
}