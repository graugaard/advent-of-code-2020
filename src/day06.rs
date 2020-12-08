use crate::puzzle_input;
use crate::util::{Group, GroupIterator};
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn get_answer_from_group(group: &Group) -> Vec<HashSet<char>> {
    let mut vec = Vec::new();
    for line in group.as_str().split_ascii_whitespace() {
        vec.push(HashSet::from_iter(line.chars()));
    }

    vec
}

pub fn get_combined_answers_of_groups(str: &str) -> Vec<HashSet<char>> {
    let mut result = Vec::new();
    for group in GroupIterator::new(str) {
        let answers = get_answer_from_group(&group);

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
    for group in GroupIterator::new(str) {
        let answers = get_answer_from_group(&group);

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

    println!(
        "Day 06 Solution Part 1: {}",
        sum_answers(&get_combined_answers_of_groups(&puzzle))
    );

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
    use crate::day06::{
        get_answer_from_group, get_combined_answers_of_groups, get_common_answer_of_groups,
        sum_answers,
    };
    use crate::util::Group;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn a_group_of_one_member() {
        assert_eq!(
            get_answer_from_group(&Group::new("abcd")),
            vec![HashSet::from_iter(vec!['a', 'b', 'c', 'd'])]
        );
    }

    #[test]
    fn a_group_of_multiple_members() {
        assert_eq!(
            get_answer_from_group(&Group::new("abc\ncb\nfge")),
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
            vec![HashSet::from_iter(vec![
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'x'
            ])]
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
        let common = get_combined_answers_of_groups(
            "abc\n\r\na\r\nb\r\nc\r\n\r\nab\r\nac\r\n\na\r\na\r\na\r\na\r\n\r\nb",
        );
        assert_eq!(sum_answers(&common), 11)
    }

    #[test]
    fn sum_common_answers() {
        let common = get_common_answer_of_groups(
            "abc\r\n\r\na\r\nb\r\nc\r\n\r\nab\r\nac\r\n\r\na\r\na\r\na\r\na\n\nb",
        );
        assert_eq!(sum_answers(&common), 6)
    }
}
