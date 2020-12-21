use crate::util::puzzle_input;
use std::collections::HashMap;
use std::str::FromStr;

pub fn print_solution() {
    let input = puzzle_input::read_input("day14");

    let map = run_program(&input);

    let answer: u64 = map.values().sum();

    println!("Day 14 Solution Part 1: {}", answer);
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Mask {
    mask: Vec<(usize, u8)>,
}

fn run_program(str: &str) -> HashMap<u64, u64> {
    let mut current_mask = Mask::default();
    let mut state = HashMap::new();
    for line in str.lines() {
        let split: Vec<&str> = line.split(" = ").collect();
        let (left, right) = (split[0], split[1]);
        if left.starts_with("mask") {
            current_mask = right.parse::<Mask>().expect("Is a mask");
        } else {
            let address = left["mem[".len()..left.len() - 1]
                .parse::<u64>()
                .expect("Is an int");
            let mem_val = right.parse::<u64>().expect("Is an int");
            state.insert(address, current_mask.apply(mem_val));
        }
    }
    state
}

impl Default for Mask {
    fn default() -> Self {
        Self { mask: Vec::new() }
    }
}

impl Mask {
    fn apply(&self, bits: u64) -> u64 {
        let mut result = 0;
        for i in 0..64 {
            result = result << 1;
            let n = match self.mask.iter().find(|&bit| bit.0 == i) {
                Some((_, i)) => *i as u64,
                None => {
                    if bits & (1 << 63 - i) != 0 {
                        1
                    } else {
                        0
                    }
                }
            };
            result += n;
        }

        result
    }
}

impl FromStr for Mask {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mask = Vec::new();
        // assume the string represents the last part of 64 bit number
        // but the mask is for the entire 64 bit.
        let offset = 64 - s.len();

        for (idx, c) in s.chars().enumerate() {
            match c {
                '0' => mask.push((offset + idx, 0)),
                '1' => mask.push((offset + idx, 1)),
                'X' => {}
                _ => return Err(()),
            }
        }

        mask.sort_by(|m0, m1| m0.0.cmp(&m1.0).reverse());

        Ok(Self { mask })
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::day14::Mask;

    #[test]
    fn apply_simple_mask() {
        let mask = "X1X0X0X1".parse::<Mask>().expect("Valid mask");
        let new_number = mask.apply(0b1101_1000);

        assert_eq!(new_number, 0b1100_1001);
    }

    #[test]
    fn first_example() {
        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
            .parse::<Mask>()
            .expect("Valid mask");
        let new_number = mask.apply(0b000000000000000000000000000000001011);

        assert_eq!(new_number, 0b000000000000000000000000000001001001);
    }
}
