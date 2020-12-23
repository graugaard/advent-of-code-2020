use crate::util::{puzzle_input, GroupIterator};
use std::str::FromStr;

type Ticket = [i64];

pub fn print_solution() {
    let input = puzzle_input::read_input("day16");
    println!("Day 16 Solution Part 1: {}", n_ticket_error_rate(&input));
}

fn n_ticket_error_rate(input: &str) -> i64 {
    let mut iterator = GroupIterator::new(input);
    let next = iterator.next();
    let validator = next
        .expect("first group is ticket")
        .as_str()
        .parse::<TicketValidator>()
        .expect("Valid ticket rules");

    iterator.next();
    let tickets = iterator.next().expect("Tickets").as_str();

    let mut error_rate = 0;
    for ticket in tickets.lines().skip(1) {
        let ticket: Vec<i64> = ticket
            .split(",")
            .map(|s| s.parse::<i64>().expect("An int"))
            .collect();
        error_rate += validator.error_rate(&ticket);
    }

    error_rate
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Range {
    lower: i64,
    upper: i64,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("-");
        let lower = split.next().ok_or_else(|| ())?;
        let lower = lower.parse::<i64>().expect("Is an int");
        let upper = split.next().ok_or_else(|| ())?;
        let upper = upper.parse::<i64>().expect("Is an int");

        Ok(Self { lower, upper })
    }
}

impl Range {
    pub fn is_in_range(&self, val: i64) -> bool {
        self.lower <= val && val <= self.upper
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TicketValidator {
    ranges: Vec<Vec<Range>>,
}

impl FromStr for TicketValidator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranges = Vec::new();
        for line in s.lines() {
            let mut split = line.split(": ");
            let mut line_ranges = Vec::new();
            split.next();
            match split.next() {
                None => return Err(()),
                Some(str) => {
                    for range in str.split(" or ") {
                        let range = range.parse::<Range>()?;
                        line_ranges.push(range);
                    }
                }
            }
            ranges.push(line_ranges);
        }

        Ok(Self { ranges })
    }
}

impl TicketValidator {
    /// The sum of values of a ticket that does not match any
    /// of the ranges of the validator.
    pub fn error_rate(&self, ticket: &Ticket) -> i64 {
        let mut error_rate = 0;
        for &val in ticket {
            let mut in_a_range = false;
            for range in self.ranges.iter().flat_map(|r| r.iter()) {
                in_a_range = in_a_range || range.is_in_range(val);
            }
            if !in_a_range {
                error_rate += val;
            }
        }
        error_rate
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::day16::{n_ticket_error_rate, TicketValidator};
    use std::str::FromStr;

    #[test]
    fn test_validator() {
        let validator = TicketValidator::from_str(
            r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50",
        )
        .expect("Valid ticket validator descriptor");

        assert_eq!(validator.error_rate(&[7, 3, 47]), 0);
        assert_eq!(validator.error_rate(&[40, 4, 50]), 4);
        assert_eq!(validator.error_rate(&[55, 2, 20]), 55);
        assert_eq!(validator.error_rate(&[38, 6, 12]), 12);
    }

    #[test]
    fn test_count_valid_tickets() {
        let error_rate = n_ticket_error_rate(
            r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12",
        );

        assert_eq!(error_rate, 71);
    }
}
