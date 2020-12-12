use crate::day11::Type::*;
use crate::puzzle_input;
use crate::util::map::{Map, Terrain, TerrainErr};

pub fn print_solution() {
    let map = Map::<Type>::configure(&puzzle_input::read_input("day11")).unwrap();

    let end = map.find_end(10_000).unwrap();

    let occupied = end
        .iter()
        .filter(|c| *c.terrain() == Type::Occupied)
        .count();

    println!("Day 11 Solution Part 1: {}", occupied);
}

#[derive(Eq, PartialEq, Debug)]
enum Type {
    Empty,
    Occupied,
    Ground,
}

impl Terrain for Type {
    fn from_char(c: char) -> Result<Self, TerrainErr> {
        match c {
            'L' => Ok(Type::Empty),
            '#' => Ok(Type::Occupied),
            '.' => Ok(Type::Ground),
            _ => Err(TerrainErr::UnknownTerrain(c)),
        }
    }
}

impl Map<Type> {
    pub fn next(&self) -> Self {
        let mut vec = Vec::with_capacity(self.size());

        for cord in self.iter() {
            let neighbours = self.neighbours(cord.x(), cord.y());

            let new_terrain = match cord.terrain() {
                Ground => Ground,
                Occupied => {
                    let count = neighbours
                        .iter()
                        .filter(|c| *c.terrain() == Occupied)
                        .count();

                    if count > 3 {
                        Empty
                    } else {
                        Occupied
                    }
                }
                Empty => {
                    let count = neighbours
                        .iter()
                        .filter(|c| *c.terrain() == Occupied)
                        .count();

                    let new_terrain = if count > 0 { Empty } else { Occupied };

                    new_terrain
                }
            };

            vec.push(new_terrain);
        }

        Map::init(vec, self.width()).unwrap()
    }

    pub fn find_end(&self, round_limit: usize) -> Option<Map<Type>> {
        let mut prev = self.next();
        let mut cur;
        for _ in 1..round_limit {
            cur = prev.next();
            {
                if cur.eq(&prev) {
                    return Some(cur);
                }
            }
            prev = cur;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::day11::{Map, Type};

    #[test]
    fn test_simple_seat() {
        let map: Map<Type> = Map::configure("L.\n#.").unwrap();
        assert_eq!(map.terrain_at(0, 2), None);
        assert_eq!(map.terrain_at(2, 0), None);
        assert_eq!(map.terrain_at(0, 0), Some(&Type::Empty));
    }

    #[test]
    fn test_advancing_map() {
        let map = Map::<Type>::configure("...\n.L.\n...\n#L.\n...").unwrap();

        let expected_map = Map::<Type>::configure("...\n.#.\n...\n#L.\n...").unwrap();

        let next_map = map.next();
        assert_eq!(next_map, expected_map);
    }

    #[test]
    fn test_advancing_with_occupied_seat() {
        let map = Map::<Type>::configure(".#.\n###\n.#.").unwrap();

        let expected_map = Map::<Type>::configure(".#.\n#L#\n.#.").unwrap();

        let next_map = map.next();
        assert_eq!(next_map, expected_map);
    }

    #[test]
    fn test_go_to_end_for_example() {
        let start = Map::<Type>::configure(
            "L.LL.LL.LL\n\
LLLLLLL.LL\n\
L.L.L..L..\n\
LLLL.LL.LL\n\
L.LL.LL.LL\n\
L.LLLLL.LL\n\
..L.L.....\n\
LLLLLLLLLL\n\
L.LLLLLL.L\n\
L.LLLLL.LL",
        )
        .unwrap();

        let end = Map::<Type>::configure(
            "#.#L.L#.##\n\
#LLL#LL.L#\n\
L.#.L..#..\n\
#L##.##.L#\n\
#.#L.LL.LL\n\
#.#L#L#.##\n\
..L.L.....\n\
#L#L##L#L#\n\
#.LLLLLL.L\n\
#.#L#L#.##",
        )
        .unwrap();

        assert_eq!(start.find_end(15), Some(end));
    }
}
