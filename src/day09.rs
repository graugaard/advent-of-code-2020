use std::process::id;
use crate::puzzle_input;

pub fn print_solution() {
    let input: Vec<i64> = puzzle_input::read_input("day09")
      .split_ascii_whitespace()
      .map(|n| n.parse::<i64>().unwrap())
      .collect();


    println!("Day 09 Solution Part 1: {:?}", find_xmas_violation(&input, 25));
}

pub fn find_xmas_violation(list: &[i64], n_previous: usize) -> Option<i64> {
    for idx in n_previous + 1.. list.len() {
        let target = list[idx];
        let mut found = false;
        for x in idx - n_previous .. idx {
            for y in x + 1 .. idx {
                if target == list[x] + list[y] {
                    found = true;
                }
            }
        }
        if !found {
            return Some(target);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::day09::find_xmas_violation;

    #[test]
    fn test_array_with_no_violation() {
        assert_eq!(find_xmas_violation(&[1, 2, 3, 5, 7, 10], 3), None);
    }

    #[test]
    fn test_with_example_violation() {
        let example = [
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576,
        ];

        assert_eq!(find_xmas_violation(&example, 5), Some(127));
    }
}