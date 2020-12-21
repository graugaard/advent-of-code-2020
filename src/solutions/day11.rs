use crate::day11::Type::*;
use crate::util::puzzle_input;
use crate::util::map::{Map, Terrain, TerrainErr};

pub fn print_solution() {
    let map = Map::<Type>::configure(&puzzle_input::read_input("day11")).unwrap();

    let end = map.find_end(10_000).unwrap();

    let occupied = end
        .iter()
        .filter(|c| *c.terrain() == Type::Occupied)
        .count();

    println!("Day 11 Solution Part 1: {}", occupied);

    let map = Map::<Type>::configure(&puzzle_input::read_input("day11")).unwrap();
    let occupied = map
        .end_star(10_000)
        .unwrap()
        .iter()
        .filter(|c| *c.terrain() == Type::Occupied)
        .count();

    println!("Day 11 Solution Part 2: {}", occupied);
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

    fn to_char(&self) -> char {
        match self {
            Type::Empty => 'L',
            Type::Occupied => '#',
            Type::Ground => '.',
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

    pub fn next_star(&self) -> Self {
        let mut vec = Vec::with_capacity(self.size());
        for point in self.iter() {
            let offsets = [
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ];
            let mut neighbours = Vec::with_capacity(8);
            for offset in &offsets {
                if let Some(c) =
                    self.step_until((point.x(), point.y()), *offset, |c| *c.terrain() != Ground)
                {
                    neighbours.push(c);
                }
            }
            let new_terrain = match point.terrain() {
                Ground => Ground,
                Occupied => {
                    let count = neighbours
                        .iter()
                        .filter(|c| *c.terrain() == Occupied)
                        .count();

                    if count > 4 {
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

    pub fn end_star(&self, round_limit: usize) -> Option<Self> {
        Self::to_end(self, Self::next_star, round_limit)
    }

    fn to_end<F>(&self, next: F, round_limit: usize) -> Option<Self>
    where
        F: Fn(&Self) -> Self,
    {
        let mut prev = next(self);
        let mut cur;
        for _ in 1..round_limit {
            cur = next(&prev);
            {
                if cur.eq(&prev) {
                    return Some(cur);
                }
            }
            prev = cur;
        }

        None
    }

    pub fn find_end(&self, round_limit: usize) -> Option<Map<Type>> {
        Self::to_end(self, Self::next, round_limit)
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

    #[test]
    fn test_multiple_iteration() {
        let m = Map::<Type>::configure(
            "\
        L.LL.LL.LL\n\
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

        let two_it = m.next_star().next_star();
        let expected = Map::<Type>::configure(
            "\
#.LL.LL.L#\n\
#LLLLLL.LL\n\
L.L.L..L..\n\
LLLL.LL.LL\n\
L.LL.LL.LL\n\
L.LLLLL.LL\n\
..L.L.....\n\
LLLLLLLLL#\n\
#.LLLLLL.L\n\
#.LLLLL.L#\n\
          ",
        )
        .unwrap();

        assert_eq!(two_it, expected);

        let expected = Map::<Type>::configure(
            "#.L#.##.L#\n\
#L#####.LL\n\
L.#.#..#..\n\
##L#.##.##\n\
#.##.#L.##\n\
#.#####.#L\n\
..#.#.....\n\
LLL####LL#\n\
#.L#####.L\n\
#.L####.L#",
        )
        .unwrap();

        let three_it = two_it.next_star();
        assert_eq!(three_it, expected);
    }

    #[test]
    fn test_until_end_with_star() {
        let m = Map::<Type>::configure(
            "\
        L.LL.LL.LL\n\
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

        let expected = Map::<Type>::configure(
            "\
            #.L#.L#.L#\n\
#LLLLLL.LL\n\
L.L.L..#..\n\
##L#.#L.L#\n\
L.L#.LL.L#\n\
#.LLLL#.LL\n\
..#.L.....\n\
LLL###LLL#\n\
#.LLLLL#.L\n\
#.L#LL#.L#",
        )
        .unwrap();

        assert_eq!(m.end_star(15), Some(expected));
    }
}
