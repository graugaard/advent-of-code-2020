use crate::puzzle_input;
use std::collections::HashSet;

pub fn print_solution() {
    let puzzle = puzzle_input::read_input("day05");

    let potential_max = puzzle
        .split_ascii_whitespace()
        .map(|s| determine_seat_id(s))
        .max();
    let max_id = match potential_max {
        Some(id) => id,
        None => 0,
    };
    println!("Day 05 Solution Part 1: {}", max_id);
    println!("Day 05 Solution Part 2: {}", get_my_seat(&puzzle));
}

/// my seat is a seat not in the list and such
/// that both seat + 1 and seat - 1 is
fn get_my_seat(str: &str) -> u32 {
    let set: HashSet<u32> = str
        .split_ascii_whitespace()
        .map(|s| determine_seat_id(s))
        .collect();

    for seat in &set {
        if set.contains(&(seat - 2)) && !set.contains(&(seat - 1)) {
            return seat - 1;
        }
        if set.contains(&(seat + 2)) && !set.contains(&(seat + 1)) {
            return seat + 1;
        }
    }
    0
}

pub fn determine_row(row_str: &str) -> u32 {
    let mut row = 0;
    for c in row_str.chars() {
        row = row << 1;
        row += match c {
            'B' => 1,
            _ => 0,
        };
    }

    row
}

pub fn determine_column(column_str: &str) -> u32 {
    let mut column = 0;
    for c in column_str.chars() {
        column = column << 1;
        column += match c {
            'R' => 1,
            _ => 0,
        }
    }

    column
}

pub fn determine_seat_id(boarding_pass: &str) -> u32 {
    8 * determine_row(&boarding_pass[0..7]) + determine_column(&boarding_pass[7..])
}

#[cfg(test)]
mod test {
    use crate::day05::{determine_column, determine_row, determine_seat_id};

    #[test]
    fn row_bbbbffb_is_row_0b111_100_1() {
        assert_eq!(determine_row("BBBBFFB"), 0b111_100_1);
    }

    #[test]
    fn row_bfffbbf_is_0b1000110() {
        assert_eq!(determine_row("BFFFBBF"), 0b1000110);
    }

    #[test]
    fn column_rlr_is_0b101() {
        assert_eq!(determine_column("RLR"), 0b101);
    }

    #[test]
    fn determine_example_seat_ids() {
        assert_eq!(determine_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(determine_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(determine_seat_id("BBFFBBFRLL"), 820);
    }
}
