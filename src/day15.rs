use crate::puzzle_input;
use std::collections::HashMap;

pub fn print_solution() {
    let input = puzzle_input::read_input("day15");
    let input: Vec<usize> = input
        .split(",")
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect();

    println!("Day 15 Solution Part 1: {}", n_th_number(&input, 2020));
    println!("Day 15 Solution Part 2: {}", n_th_number(&input, 30000000));
}

fn n_th_number(start: &[usize], turn: usize) -> usize {
    //let  turn = turn - 1; // it is specified we start at turn 1
    if turn <= start.len() {
        return start[turn - 1];
    }

    let mut val_to_round: HashMap<usize, usize> = HashMap::with_capacity(turn);

    for (idx, &val) in start.iter().enumerate() {
        val_to_round.insert(val, idx + 1);
    }

    let mut last_number = *start.last().expect("Start is not empty");
    for current_round in start.len()..turn {
        match val_to_round.get(&last_number) {
            None => {
                val_to_round.insert(last_number, current_round);
                last_number = 0;
            }
            Some(&number) => {
                let tmp = current_round - number;
                val_to_round.insert(last_number, current_round);
                last_number = tmp;
            }
        }
    }

    last_number
}

#[cfg(test)]
mod tests {
    use crate::day15::n_th_number;

    #[test]
    fn start_0_3_6_first_3_rounds() {
        let start = [0, 3, 6];
        assert_eq!(n_th_number(&start, 1), 0);
        assert_eq!(n_th_number(&start, 2), 3);
        assert_eq!(n_th_number(&start, 3), 6);
    }

    #[test]
    fn start_0_3_6_first_4_rounds() {
        let start = [0, 3, 6];
        assert_eq!(n_th_number(&start, 4), 0);
    }

    #[test]
    fn start_0_3_6_first_5_rounds() {
        let start = [0, 3, 6];
        assert_eq!(n_th_number(&start, 5), 3);
    }

    #[test]
    fn start_0_3_6_first_6_rounds() {
        let start = [0, 3, 6];
        assert_eq!(n_th_number(&start, 6), 3);
    }

    #[test]
    fn start_0_3_6_first_7_rounds() {
        let start = [0, 3, 6];
        assert_eq!(n_th_number(&start, 7), 1);
    }

    #[test]
    fn start_0_3_6_first_8_rounds() {
        let start = [0, 3, 6];
        assert_eq!(n_th_number(&start, 8), 0);
    }

    #[test]
    fn start_0_3_6_first_9_rounds() {
        let start = [0, 3, 6];
        assert_eq!(n_th_number(&start, 9), 4);
    }

    #[test]
    fn start_0_3_6_first_10_rounds() {
        let start = [0, 3, 6];
        assert_eq!(n_th_number(&start, 10), 0);
    }

    #[test]
    fn first_example() {
        let start = [0, 3, 6];
        assert_eq!(n_th_number(&start, 2020), 436);
    }

    #[test]
    fn second_example() {
        assert_eq!(n_th_number(&[1, 3, 2], 2020), 1);
    }
}
