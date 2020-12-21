use crate::util::puzzle_input;

pub fn print_solution() {
    let input: Vec<i64> = puzzle_input::read_input("day09")
        .split_ascii_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    let violation = find_xmas_violation(&input, 25);
    println!("Day 09 Solution Part 1: {:?}", violation);
    println!(
        "Day 09 Solution Part 2: {:?}",
        find_contiguous_min_max_sum(&input, violation.unwrap())
    );
}

pub fn find_xmas_violation(list: &[i64], n_previous: usize) -> Option<i64> {
    for idx in n_previous + 1..list.len() {
        let target = list[idx];
        let mut found = false;
        for x in idx - n_previous..idx {
            for y in x + 1..idx {
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

pub fn find_contiguous_min_max_sum(list: &[i64], target: i64) -> i64 {
    let mut lower = 0;
    let mut higher = 0;
    let mut cur_sum = 0;
    while cur_sum != target || higher < list.len() {
        if cur_sum < target {
            cur_sum += list[higher];
            higher += 1;
        } else if cur_sum > target {
            cur_sum -= list[lower];
            lower += 1;
        } else {
            let mut vec = list[lower..higher].to_vec();
            vec.sort();
            return vec[0] + vec[vec.len() - 1];
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::day09::{find_contiguous_min_max_sum, find_xmas_violation};

    #[test]
    fn test_array_with_no_violation() {
        assert_eq!(find_xmas_violation(&[1, 2, 3, 5, 7, 10], 3), None);
    }

    #[test]
    fn test_with_example_violation() {
        let example = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(find_xmas_violation(&example, 5), Some(127));
    }

    #[test]
    fn test_find_min_max_contiguous_sum() {
        let example = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(find_contiguous_min_max_sum(&example, 127), 62);
    }
}
