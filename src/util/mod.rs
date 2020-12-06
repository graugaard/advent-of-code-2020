/// Takes the lines of a string and return the lines that between blank lines
/// # Examples
/// ```rust
/// let group = group_lines("cake\nicecream\r\nsugar\r\n\nbook\natuhor");
/// assert_eq!(group, vec![vec!["cake", "icecream", "sugar"], vec!["book", "author"]]);
/// ```
pub fn group_lines(str: &str) -> Vec<Vec<&str>> {
		let mut result = Vec::new();
		let mut current_group = Vec::new();
		for line in str.lines() {
				if line.trim().is_empty() {
						result.push(current_group);
						current_group = Vec::new();
				}
				else {
						current_group.push(line);
				}
		}
		if !current_group.is_empty() {
				result.push(current_group);
		}
		result
}

#[cfg(test)]
mod tests {
		use crate::util::group_lines;

		#[test]
		fn lines_are_grouped_by_blank_lines() {
				let group = group_lines("abcde\nadd\r\ncake\n\nanother\r\ngroup");
				assert_eq!(
						group,
						vec![
								vec!["abcde", "add", "cake"],
								vec!["another", "group"]
						]
				)
		}
}