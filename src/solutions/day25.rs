use crate::util::puzzle_input;

pub fn print_solution() {
    let keys: Vec<u64> = puzzle_input::read_input("day25").lines()
      .map(|s| s.parse::<u64>().expect("Is a number"))
      .collect();

    println!("Day 25 Solution Part 1: {}", find_key(7, keys[0], keys[1], 20201227));
}

/// ```
/// let base = 7;
/// let h = 5764801;
/// let modulo = 20201227;
/// let x = find_discrete_log(base, h, modulo);
/// assert_eq!(base.pow(x) % modulo, h % modulo);
/// ```
fn find_discrete_log(base: u64, h: u64, modulo: u64) -> u64 {
    let h = h % modulo;
    let mut tmp = 1;
    let mut loop_count = 0;
    // invariant: tmp = base.pow(loop_count) % modulo
    while tmp != h {
        tmp *= base;
        tmp = tmp % modulo;

        loop_count += 1;
    }

    loop_count
}

/// When `a_key = base ^ a` and `b_key = base ^ b`, we return
/// `base ^ (a * b)`
fn find_key(base: u64, a_key: u64, b_key: u64, modulo: u64) -> u64 {
    let exp = find_discrete_log(base, a_key, modulo);

    let mut key = 1;
    for _ in 0..exp {
        key *= b_key;
        key %= modulo;
    }

    key
}

#[cfg(test)]
mod tests {
    use crate::solutions::day25::{find_discrete_log, find_key};

    #[test]
    fn test_discrete_log() {
        assert_eq!(find_discrete_log(7, 5764801, 20201227), 8);
        assert_eq!(find_discrete_log(7, 17807724, 20201227), 11)
    }

    #[test]
    fn test_find_key() {
        assert_eq!(find_key(7, 5764801, 17807724, 20201227), 14897079);
    }
}