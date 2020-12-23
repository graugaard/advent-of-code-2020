use crate::util::puzzle_input;
use std::collections::HashMap;
use std::str::FromStr;

pub fn print_solution() {
    let input = puzzle_input::read_input("day14");

    let map = run_program(&input);

    let answer: u64 = map.values().sum();

    println!("Day 14 Solution Part 1: {}", answer);

    let state = run_mem_program(&input);
    let answer: u64 = state.values().sum();

    println!("Day 14 Solution Part 2: {}", answer);
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

fn run_mem_program(str: &str) -> HashMap<u64, u64> {
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
            for mask_addresses in current_mask.mask_memory_addresses(address) {
                state.insert(mask_addresses, mem_val);
            }
        }
    }
    state
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MaskBit {
    One,
    Zero,
    WildCard,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Mask {
    mask: Vec<(usize, MaskBit)>,
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
            let n = match self
                .mask
                .iter()
                .find(|&(idx, mask_bit)| *idx == i as usize && *mask_bit != MaskBit::WildCard)
            {
                Some((_, mask_bit)) => {
                    if MaskBit::Zero == *mask_bit {
                        0
                    } else {
                        1
                    }
                }
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

    pub fn mask_memory_addresses(&self, base_address: u64) -> Vec<u64> {
        Mask::rec_address_calc(self, 0, base_address)
    }

    fn rec_address_calc(&self, idx: usize, working_address: u64) -> Vec<u64> {
        if 64 <= idx {
            vec![working_address]
        } else {
            let bit = self.mask.iter().find(|&s| s.0 == idx);

            match bit {
                None => Mask::rec_address_calc(self, idx + 1, working_address),
                Some((_, MaskBit::Zero)) => Mask::rec_address_calc(self, idx + 1, working_address),
                Some((bit_idx, MaskBit::One)) => {
                    let working_address = Mask::set_bit_to_one(working_address, *bit_idx);
                    Mask::rec_address_calc(self, idx + 1, working_address)
                }
                Some((bit_idx, MaskBit::WildCard)) => {
                    let zero_address = Mask::set_bit_to_zero(working_address, *bit_idx);
                    let mut res = Mask::rec_address_calc(self, idx + 1, zero_address);

                    let one_address = Mask::set_bit_to_one(working_address, *bit_idx);
                    let additional_address = Mask::rec_address_calc(self, idx + 1, one_address);

                    res.extend(additional_address.iter());

                    res
                }
            }
        }
    }

    /// most significant bit is at idx 0
    fn set_bit_to_one(bits: u64, idx: usize) -> u64 {
        let bits = bits | (1 << (63 - idx));
        bits
    }

    fn set_bit_to_zero(bits: u64, idx: usize) -> u64 {
        let mask = 1 << (63 - idx);
        let mask = !mask;
        bits & mask
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
                '0' => mask.push((offset + idx, MaskBit::Zero)),
                '1' => mask.push((offset + idx, MaskBit::One)),
                'X' => mask.push((offset + idx, MaskBit::WildCard)),
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

    #[test]
    fn address_masking() {
        let mask = "00000000000000000000000000000000X0XX"
            .parse::<Mask>()
            .expect("Valid mask");
        let mut addresses = mask.mask_memory_addresses(0b000000000000000000000000000000011010);

        addresses.sort();
        assert_eq!(
            addresses,
            vec![
                0b000000000000000000000000000000010000,
                0b000000000000000000000000000000010001,
                0b000000000000000000000000000000010010,
                0b000000000000000000000000000000010011,
                0b000000000000000000000000000000011000,
                0b000000000000000000000000000000011001,
                0b000000000000000000000000000000011010,
                0b000000000000000000000000000000011011,
            ]
        )
    }
}
