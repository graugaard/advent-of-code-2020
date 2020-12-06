/// Represent a group of a larger text.
#[derive(PartialOrd, PartialEq, Debug)]
pub struct Group<'a> {
		str: &'a str,
}

impl<'a> Group<'a> {
		pub fn new(str: &'a str) -> Self {
				Group {
						str
				}
		}

		pub fn as_str(&self) -> &'a str {
				self.str
		}
}

/// Allows for the iterations of groups in a larger texts.
pub struct GroupIterator<'a> {
		str: &'a str,
		idx: usize,
}

impl<'a> Iterator for GroupIterator<'a> {
		type Item = Group<'a>;

		fn next(&mut self) -> Option<Self::Item> {
				if self.idx >= self.str.len() {
						return None;
				}
				let start = self.idx;
				let mut offset = 0;
				while GroupIterator::ends_with_double_newline(&self.str[start..start + offset]).is_none()
					&& start + offset < self.str.len() {
						offset += 1;
				}

				let end = match GroupIterator::ends_with_double_newline(&self.str[start..start + offset]) {
						Some(size_of_end) => {
								start + offset - size_of_end
						},
						None => start + offset
				};
				self.idx = start + offset;
				Some(Group::new(&self.str[start..end]))
		}
}

impl<'a> GroupIterator<'a> {
		/// Creates the iterator. Groups are separated by a blank line.
		pub fn new(str: &'a str) -> Self {
				GroupIterator {
						str,
						idx: 0,
				}
		}

		fn ends_with_double_newline(str: &str) -> Option<usize> {
				if str.ends_with("\r\n\r\n") {
						return Some(4);
				} else if str.ends_with("\n\r\n") {
						return Some(3);
				} else if str.ends_with("\r\n\n") {
						return Some(3);
				} else if str.ends_with("\n\n") { // must be last as it covers the previous case
						return Some(2);
				}

				None
		}
}

#[cfg(test)]
mod tests {
		use crate::util::{GroupIterator, Group};


		#[test]
		fn grouping_of_simple_string() {
				let iterator = GroupIterator::new("group1-1\ngroup1-2");
				let collection: Vec<Group> = iterator.collect();
				assert_eq!(collection, vec![Group::new("group1-1\ngroup1-2")]);
		}

		#[test]
		fn grouping_of_two_groups() {
				let iterator = GroupIterator::new("grp1-1\n\ngroup2-1\ngroup2-2");
				let collection: Vec<Group> = iterator.collect();

				assert_eq!(
						collection.len(),
						2);
		}

		#[test]
		fn test_grouping_with_two_groups() {
				let iterator = GroupIterator::new("grp1-1\n\ngroup2-1\ngroup2-2");
				let collection: Vec<Group> = iterator.collect();

				assert_eq!(
						collection,
						vec![
								Group::new("grp1-1"),
								Group::new("group2-1\ngroup2-2")]);
		}

		#[test]
		fn test_with_different_group_delimeter() {
				let itr: Vec<Group> = GroupIterator::new("grp\r\n\ngrp2\n\r\ngrp3\r\n\r\ngrp4\n\ngrp5").collect();
				assert_eq!(
						itr,
						vec![
								Group::new("grp"),
								Group::new("grp2"),
								Group::new("grp3"),
								Group::new("grp4"),
								Group::new("grp5")
						]
				)
		}
}