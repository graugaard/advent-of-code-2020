use crate::util::puzzle_input;
use puzzle_input::read_input;

pub fn print_solution() {
    let mut input: Vec<u64> = read_input("day10")
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    input.sort();

    let jolts = n_jolt_leaps(&input);

    println!("Day 10 Solution Part 1: {}", jolts.0 * jolts.1);
    println!("Day 10 Solution Part 2: {}", count_arrangements(&input));
}

/// Assumes the adapters are in sorted order
pub fn count_arrangements(adapters: &[u64]) -> u64 {
    let mut cache = [0; 3];
    cache[0] = 1;

    if adapters[1] <= 3 {
        cache[1] += 1;
    }

    if adapters[1] - adapters[0] <= 3 {
        cache[1] += cache[0];
    }

    if adapters[2] <= 3 {
        cache[2] += 1;
    }
    if adapters[2] - adapters[1] <= 3 {
        cache[2] += cache[1];
    }
    if adapters[2] - adapters[0] <= 3 {
        cache[2] += cache[0];
    }

    for idx in 3..adapters.len() {
        let mut tmp = 0;
        for offset in 1..4 {
            if adapters[idx] - adapters[idx - offset] <= 3 {
                tmp += cache[3 - offset];
            }
        }
        cache[0] = cache[1];
        cache[1] = cache[2];
        cache[2] = tmp;
    }

    cache[2]
}

/// Assumes that the adapters are in sorted order
fn n_jolt_leaps(adapters: &[u64]) -> (u64, u64) {
    let mut one_leaps = match adapters[0] {
        1 => 1,
        _ => 0,
    };

    let mut three_leaps = match adapters[0] {
        3 => 2,
        _ => 1,
    };

    for idx in 0..adapters.len() - 1 {
        match adapters[idx + 1] - adapters[idx] {
            1 => {
                one_leaps += 1;
            }
            3 => {
                three_leaps += 1;
            }
            _ => {}
        }
    }

    (one_leaps, three_leaps)
}

#[cfg(test)]
mod tests {
    use crate::day10::{count_arrangements, n_jolt_leaps};

    #[test]
    fn test_voltage() {
        let mut adapters = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        adapters.sort();

        assert_eq!(n_jolt_leaps(&adapters), (7, 5));
    }

    #[test]
    fn test_count_arrangements_small_examples() {
        let mut adapters = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        adapters.sort();

        assert_eq!(count_arrangements(&adapters), 8);
    }

    #[test]
    fn test_count_arrangements_large_example() {
        let mut adapters = [
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        adapters.sort();

        assert_eq!(count_arrangements(&adapters), 19208);
    }
}
