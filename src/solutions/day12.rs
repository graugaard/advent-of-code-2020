use crate::util::puzzle_input;
use std::str::FromStr;

pub fn print_solution() {
    let directions: Vec<Direction> = puzzle_input::read_input("day12")
        .lines()
        .map(|s| s.parse::<Direction>())
        .map(|r| r.expect("All inputs are valid"))
        .collect();

    let mut ship = Ship::default();
    for direction in directions.iter() {
        ship.move_ship(direction);
    }

    println!("Day 12 Solution Part 1: {}", ship.distance());
    println!(
        "Day 12 Solution Part 2: {}",
        waypoint_simulation(&directions)
    );
}

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Right(i64),
    Left(i64),
    Forward(i64),
}

pub fn waypoint_simulation(movement: &[Direction]) -> u64 {
    let mut ship = Ship::default();
    let mut waypoint = Ship::new((10, 1), 0);

    for direction in movement {
        match direction {
            Direction::Forward(steps) => {
                ship.move_towards(waypoint.position, *steps);
            }
            Direction::Left(degree) => {
                waypoint.rotate_around_orig_clockwise(360 - degree);
            }
            Direction::Right(degree) => {
                waypoint.rotate_around_orig_clockwise(*degree);
            }
            _ => {
                waypoint.move_ship(direction);
            }
        }
    }

    ship.distance()
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(());
        }
        let magnitude = match s[1..].parse::<i64>() {
            Ok(n) => n,
            Err(_) => return Err(()),
        };

        let first_char = s.chars().next().expect("At least one char is present");
        let direction = match first_char {
            'N' => Direction::North(magnitude),
            'S' => Direction::South(magnitude),
            'E' => Direction::East(magnitude),
            'W' => Direction::West(magnitude),
            'R' => Direction::Right(magnitude),
            'L' => Direction::Left(magnitude),
            'F' => Direction::Forward(magnitude),
            _ => return Err(()),
        };

        Ok(direction)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Ship {
    position: (i64, i64),
    direction: i64,
}

impl Ship {
    pub fn new(position: (i64, i64), direction: i64) -> Self {
        Ship {
            position,
            direction,
        }
    }

    pub fn distance(&self) -> u64 {
        (self.position.0.abs() + self.position.1.abs()) as u64
    }

    pub fn move_towards(&mut self, step_size: (i64, i64), n_steps: i64) {
        self.position = (
            self.position.0 + step_size.0 * n_steps,
            self.position.1 + step_size.1 * n_steps,
        );
    }

    /// Done clockwise
    pub fn rotate_around_orig_clockwise(&mut self, degree: i64) {
        if degree == 90 {
            self.position = (self.position.1, -self.position.0)
        } else if degree == 180 {
            self.position = (-self.position.0, -self.position.1)
        } else if degree == 270 {
            self.position = (-self.position.1, self.position.0)
        }
    }

    pub fn move_ship(&mut self, direction: &Direction) {
        match direction {
            Direction::North(magnitude) => {
                self.position = (self.position.0, self.position.1 + magnitude)
            }
            Direction::East(magnitude) => {
                self.position = (self.position.0 + magnitude, self.position.1)
            }
            Direction::South(magnitude) => {
                self.position = (self.position.0, self.position.1 - magnitude)
            }
            Direction::West(magnitude) => {
                self.position = (self.position.0 - magnitude, self.position.1)
            }
            Direction::Left(magnitude) => {
                self.direction -= magnitude;
                if self.direction < 0 {
                    self.direction += 360;
                }
            }
            Direction::Right(magnitude) => {
                self.direction += magnitude;
                if self.direction >= 360 {
                    self.direction -= 360;
                }
            }
            Direction::Forward(magnitude) => {
                let pos = if self.direction == 0 {
                    (0, 1)
                } else if self.direction == 90 {
                    (1, 0)
                } else if self.direction == 180 {
                    (0, -1)
                } else {
                    (-1, 0)
                };

                self.position = (
                    self.position.0 + pos.0 * magnitude,
                    self.position.1 + pos.1 * magnitude,
                );
            }
        }
    }
}

impl Default for Ship {
    fn default() -> Self {
        Ship {
            position: (0, 0),
            direction: 90,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day12::{waypoint_simulation, Direction, Ship};

    #[test]
    fn derive_direction_from_string() {
        assert_eq!("W12".parse::<Direction>(), Ok(Direction::West(12)));
    }

    #[test]
    fn move_north() {
        let mut ship = Ship::default();
        ship.move_ship(&Direction::North(12));
        let expected_ship = Ship::new((0, 12), 90);
        assert_eq!(ship, expected_ship);
    }

    #[test]
    fn move_left() {
        let mut ship = Ship::new((0, 0), 0);
        ship.move_ship(&Direction::Left(90));

        let expected = Ship::new((0, 0), 270);
        assert_eq!(ship, expected);
    }

    #[test]
    fn move_right() {
        let mut ship = Ship::new((0, 0), 270);
        ship.move_ship(&Direction::Right(90));

        let expected = Ship::new((0, 0), 0);
        assert_eq!(ship, expected);
    }

    #[test]
    fn move_forward() {
        let mut ship = Ship::new((0, 0), 0);
        ship.move_ship(&Direction::Forward(2));
        let expected = Ship::new((0, 2), 0);
        assert_eq!(ship, expected);

        let mut ship = Ship::new((0, 0), 90);
        ship.move_ship(&Direction::Forward(4));
        let expected = Ship::new((4, 0), 90);
        assert_eq!(ship, expected);

        let mut ship = Ship::new((0, 0), 180);
        ship.move_ship(&Direction::Forward(12));
        let expected = Ship::new((0, -12), 180);
        assert_eq!(ship, expected);

        let mut ship = Ship::new((0, 0), 270);
        ship.move_ship(&Direction::Forward(5));
        let expected = Ship::new((-5, 0), 270);
        assert_eq!(ship, expected);
    }

    #[test]
    fn waypoint_forward() {
        let directions = vec![
            Direction::Forward(10),
            Direction::North(3),
            Direction::Forward(7),
            Direction::Right(90),
            Direction::Forward(11),
        ];

        let distance = waypoint_simulation(&directions);

        assert_eq!(distance, 286)
    }
}
