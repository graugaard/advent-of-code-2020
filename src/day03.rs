use crate::day03::Error::UnknownTerrain;
use crate::day03::Terrain::{Ground, Tree};
use crate::puzzle_input;

pub struct Map {
    map: Vec<Terrain>,
    width: usize,
}

impl Map {
    pub fn configure(config: &str) -> Result<Map, Error> {
        let mut map = Vec::new();

        let mut width = 0;
        for line in config.split_ascii_whitespace() {
            let terrain = Map::convert_line(line)?;
            width = terrain.len();

            for t in terrain {
                map.push(t);
            }
        }

        let map = Map { map, width };

        Ok(map)
    }

    pub fn terrain_at(&self, x: usize, y: usize) -> Terrain {
        self.map[(x % self.width) + self.width * y]
    }

    pub fn count_trees_on_route(&self, slope: &Slope) -> u64 {
        let mut count = 0;
        let mut x = 0;

        for row in (0..self.height()).step_by(slope.down) {
            if self.terrain_at(x, row) == Tree {
                count += 1;
            }
            x += slope.right;
        }
        count
    }

    pub fn height(&self) -> usize {
        self.map.len() / self.width
    }

    pub fn convert_line(str: &str) -> Result<Vec<Terrain>, Error> {
        let mut line = Vec::with_capacity(str.len());
        for c in str.chars() {
            match c {
                '.' => line.push(Ground),
                '#' => line.push(Tree),
                _ => return Err(UnknownTerrain(c)),
            }
        }

        Ok(line)
    }
}

pub fn print_solution() {
    let map = Map::configure(&puzzle_input::read_input("day03")).unwrap();

    println!(
        "Day 03 Solution Part 1: {}",
        map.count_trees_on_route(&Slope { right: 3, down: 1 })
    );

    println!("Day 03 Solution Part 2: {}", day_03_part_2_solution(&map));
}

fn day_03_part_2_solution(map: &Map) -> u64 {
    map.count_trees_on_route(&Slope::new(1, 1))
        * map.count_trees_on_route(&Slope::new(3, 1))
        * map.count_trees_on_route(&Slope::new(5, 1))
        * map.count_trees_on_route(&Slope::new(7, 1))
        * map.count_trees_on_route(&Slope::new(1, 2))
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Terrain {
    Ground,
    Tree,
}

#[derive(Debug)]
pub struct Slope {
    right: usize,
    down: usize,
}

impl Slope {
    fn new(right: usize, down: usize) -> Slope {
        Slope { right, down }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Error {
    UnknownTerrain(char),
}

#[cfg(test)]
mod tests {
    use crate::day03::Error::UnknownTerrain;
    use crate::day03::Terrain::{Ground, Tree};
    use crate::day03::{Map, Slope};

    #[test]
    pub fn configure_map() {
        let map = Map::configure(
            ".\n
						.",
        )
        .unwrap();
        assert_eq!(map.terrain_at(0, 0), Ground);
        assert_eq!(map.terrain_at(0, 1), Ground);
    }

    #[test]
    pub fn test_convert_line() {
        assert_eq!(Map::convert_line("..."), Ok(vec![Ground; 3]));
    }

    #[test]
    pub fn if_line_contains_unknown_terrain_return_error_unknown_terrain() {
        assert_eq!(Map::convert_line("a"), Err(UnknownTerrain('a')));
    }

    #[test]
    pub fn convert_both_ground_and_tree() {
        assert_eq!(
            Map::convert_line("..#.##"),
            Ok(vec![Ground, Ground, Tree, Ground, Tree, Tree])
        );
    }

    #[test]
    pub fn configure_1_line_map_with_tree() {
        let map = Map::configure(".#").unwrap();
        assert_eq!(map.terrain_at(0, 0), Ground);
        assert_eq!(map.terrain_at(1, 0), Tree);
    }

    #[test]
    pub fn when_accessing_terrain_outside_width_wrap_around() {
        let map = Map::configure("#..\n.#.\n###").unwrap();

        assert_eq!(map.terrain_at(3, 0), Tree);
        assert_eq!(map.terrain_at(5, 1), Ground);
    }

    #[test]
    pub fn the_height_is_the_number_of_lines() {
        let map = Map::configure("..\n..").unwrap();
        assert_eq!(map.height(), 2);
    }

    #[test]
    pub fn traverse_this_map_should_encounter_1_tree() {
        let map = Map::configure("#.\n..").unwrap();
        let slope = Slope { right: 1, down: 1 };

        assert_eq!(map.count_trees_on_route(&slope), 1);
    }

    #[test]
    pub fn travese_example_map_should_encounter_7_trees() {
        let map = Map::configure(
            "..##.......\n\
										#...#...#..\n\
										.#....#..#.\n\
										..#.#...#.#\n\
										.#...##..#.\n\
										..#.##.....\n\
										.#.#.#....#\n\
										.#........#\n\
										#.##...#...\n\
										#...##....#\n\
										.#..#...#.#",
        )
        .unwrap();
        let slope = Slope { right: 3, down: 1 };

        assert_eq!(map.count_trees_on_route(&slope), 7);
    }
}
