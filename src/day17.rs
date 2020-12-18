use crate::puzzle_input;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub fn print_solution() {
    let input = puzzle_input::read_input("day17");
    let start = get_space(&input);
    let mut map = Space::new(&start);
    map.advance_time(6);

    println!("Day 15 Solution Part 1: {}", map.active_cells.len());
    let start = append_dimension(&start);
    println!(
        "Day 15 Solution Part 2: {}",
        run_simulation(&start, 6).len()
    )
}

fn append_dimension(start: &Vec<(i64, i64, i64)>) -> Vec<(i64, i64, i64, i64)> {
    start.iter().map(|p| (p.0, p.1, p.2, 0)).collect()
}

fn get_space(input: &str) -> Vec<(i64, i64, i64)> {
    let mut vec = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                vec.push((x as i64, y as i64, 0));
            }
        }
    }

    vec
}

#[derive(Eq, PartialEq, Debug)]
struct Space {
    active_cells: HashSet<(i64, i64, i64)>,
}

impl Space {
    fn new(active_cells: &[(i64, i64, i64)]) -> Self {
        let s = HashSet::from_iter(active_cells.iter().map(|c| *c));

        Self { active_cells: s }
    }

    fn advance_time(&mut self, rounds: usize) {
        let mut cells = HashSet::from_iter(self.active_cells.iter().map(|&c| c));

        for _ in 0..rounds {
            let mut map = HashMap::new();

            for &cell in &cells {
                map.insert(cell, 0);
            }

            for cell in &cells {
                for x in -1..=1 {
                    for y in -1..=1 {
                        for z in -1..=1 {
                            if x == 0 && y == 0 && z == 0 {
                                continue;
                            }
                            let p = (cell.0 + x, cell.1 + y, cell.2 + z);
                            let value = map.get(&p);
                            let new_value = match value {
                                None => {
                                     1
                                },
                                Some(v) => {
                                     v + 1
                                }
                            };

                            map.insert(p, new_value);
                        }
                    }
                }
            }

            let mut active_cells = HashSet::new();

            for (k, &v) in map.iter() {
                if v == 3 {
                    active_cells.insert(*k);
                } else if v == 2 && cells.contains(k) {
                    active_cells.insert(*k);
                }
            }
            cells = active_cells;
        }

        self.active_cells = cells;
    }
}

fn run_simulation(start: &[(i64, i64, i64, i64)], rounds: i64) -> Vec<(i64, i64, i64, i64)> {
    let mut cells = HashSet::from_iter(start.iter().map(|&c| c));

    for _ in 0..rounds {
        let mut map = HashMap::new();

        for &cell in &cells {
            map.insert(cell, 0);
        }

        for cell in &cells {
            for x in -1..=1 {
                for y in -1..=1 {
                    for z in -1..=1 {
                        for w in -1..=1 {
                            if x == 0 && y == 0 && z == 0 && w == 0 {
                                continue;
                            }
                            let p = (cell.0 + x, cell.1 + y, cell.2 + z, cell.3 + w);
                            let value = map.get(&p);
                            let new_value = match value {
                                None => {
                                    1
                                },
                                Some(v) => {
                                    v + 1
                                }
                            };

                            map.insert(p, new_value);
                        }
                    }
                }
            }
        }

        let mut active_cells = HashSet::new();

        for (k, &v) in map.iter() {
            if v == 3 {
                active_cells.insert(*k);
            } else if v == 2 && cells.contains(k) {
                active_cells.insert(*k);
            }
        }
        cells = active_cells;
    }

    cells.iter().map(|c| c.clone()).collect()
}

#[cfg(test)]
mod tests {
    use crate::day17::Space;

    #[test]
    fn first_iteration() {
        let active_cells = [(1, 0, 0), (2, 1, 0), (0, 2, 0), (1, 2, 0), (2, 2, 0)];

        let mut first_space = Space::new(&active_cells);

        first_space.advance_time(1);

        let next_active_cells = [
            (0, 1, 1),
            (2, 2, 1),
            (1, 3, 1),
            (0, 1, 0),
            (2, 1, 0),
            (1, 2, 0),
            (2, 2, 0),
            (1, 3, 0),
            (0, 1, -1),
            (2, 2, -1),
            (1, 3, -1),
        ];
        let secon_space = Space::new(&next_active_cells);

        assert_eq!(first_space, secon_space);
    }
}
