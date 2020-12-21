use crate::puzzle_input;

pub fn print_solution() {
    let input = puzzle_input::read_input("day13");
    let mut lines = input.lines();

    let arrival = lines
        .next()
        .map(|s| s.parse::<u64>())
        .map(|r| r.expect("Is an int"))
        .unwrap();

    let busses: Vec<(u64, u64)> = lines
        .next()
        .expect("Tere is a second line")
        .split(",")
        .enumerate()
        .filter(|&(_, s)| !s.starts_with("x"))
        .map(|(idx, s)| (idx as u64, s.parse::<u64>().expect("Is int")))
        .collect();

    let departure = earliest_depart(arrival, &busses);
    println!("Day 13 Solution Part 1: {}", departure.0 * departure.1);
    println!(
        "Day 13 Solution Part 2: {}",
        calc_chinese_remainder(&busses)
    );
}

fn earliest_depart(arrival: u64, busses: &[(u64, u64)]) -> (u64, u64) {
    let mut depart_time = u64::MAX;
    let mut used_bus = 0;
    for bus in busses {
        let time_to_next_arrival = bus.1 - (arrival % bus.1);
        if time_to_next_arrival < depart_time {
            depart_time = time_to_next_arrival;
            used_bus = bus.1;
        }
    }

    (used_bus, depart_time)
}

fn calc_chinese_remainder(equations: &[(u64, u64)]) -> u64 {
    let mut m = 1;
    for eq in equations.iter() {
        m *= eq.1 as i64;
    }

    let mut res: i64 = 0;
    for eq in equations.iter() {
        let factor = (m / (eq.1 as i64));
        let (inv, _) = extended_euclid(factor, eq.1 as i64);

        res += ((-(eq.0 as i64)) * (factor * (inv as i64)) % m);

        res = res % m;
    }

    let res = if res < 0 { res + m } else { res };

    res as u64
}

fn extended_euclid(m: i64, n: i64) -> (i64, i64) {
    let mut old_r = m;
    let mut r = n;

    let mut old_s = 1;
    let mut s = 0;
    let mut old_t = 0;
    let mut t = 1;

    while r > 0 {
        let q = old_r / r;
        let tmp = old_r;
        old_r = r;
        r = tmp - q * r;

        let tmp = old_s;
        old_s = s;
        s = tmp - q * s;

        let tmp = old_t;
        old_t = t;
        t = tmp - q * t;
    }

    (old_s, old_t)
}

#[cfg(test)]
mod tests {
    use crate::day13::{calc_chinese_remainder, earliest_depart};

    #[test]
    fn find_earliest_depart() {
        let earliest_depart = earliest_depart(939, &[(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)]);
        assert_eq!(earliest_depart, (59, 5));
    }

    #[test]
    fn test_calc_chinese_remainder() {
        let remainder = calc_chinese_remainder(&[(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)]);
        assert_eq!(remainder, 1068781);
    }
}
