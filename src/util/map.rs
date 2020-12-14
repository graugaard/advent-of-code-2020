use core::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Eq, PartialEq)]
pub struct Map<Terrain> {
    map: Vec<Terrain>,
    width: usize,
}
impl<T> Debug for Map<T>
where
    T: Terrain,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::with_capacity(self.map.len() + self.width + 1);
        s.push('\n');
        for h in 0..self.height() {
            for t in &self.map[h * self.width..(h + 1) * self.width] {
                s.push(t.to_char())
            }
            s.push('\n')
        }
        f.write_str(&s)
    }
}

impl<T> Map<T>
where
    T: Terrain,
{
    pub fn configure(config: &str) -> Result<Map<T>, MapError> {
        let mut map = Vec::with_capacity(config.len());

        let mut width = 0;
        for line in config.lines() {
            let chars: Vec<char> = line.chars().collect();
            width = chars.len();
            for c in chars {
                let terrain = match T::from_char(c) {
                    Ok(t) => Ok(t),
                    Err(e) => Err(MapError::FromTerrain(e)),
                }?;
                map.push(terrain);
            }
        }

        map.shrink_to_fit();

        Map::init(map, width)
    }

    /// It must the case that `map[x + y * width]` is the terrain at position `(x, y)`
    pub fn init(map: Vec<T>, width: usize) -> Result<Map<T>, MapError> {
        Ok(Map { map, width })
    }

    pub fn height(&self) -> usize {
        self.map.len() / self.width
    }

    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the terrain at `(x, y)` if it is within
    /// the bounds of the map.
    pub fn terrain_at(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height() {
            None
        } else {
            self.map.get(x + y * self.width)
        }
    }

    pub fn terrain_at_point(&self, p: (usize, usize)) -> Option<&T> {
        Map::terrain_at(self, p.0, p.1)
    }

    /// Get the neighbours of at the coordinate `(x, y)`
    pub fn neighbours(&self, x: usize, y: usize) -> Vec<Coordinate<T>> {
        let mut result = Vec::with_capacity(8);

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

        for &offset in &offsets {
            if let Some(cord) = self.cord_at((x, y), offset) {
                result.push(cord);
            }
        }

        result
    }

    pub fn step_until<F>(
        &self,
        start: (usize, usize),
        step_size: (isize, isize),
        is_end: F,
    ) -> Option<Coordinate<T>>
    where
        F: Fn(&Coordinate<T>) -> bool,
    {
        let mut current_step = step_size;
        while let Some(c) = self.cord_at(start, current_step) {
            if is_end(&c) {
                return Some(c);
            }
            current_step = (current_step.0 + step_size.0, current_step.1 + step_size.1);
        }
        None
    }

    pub fn cord_at(&self, p: (usize, usize), offset: (isize, isize)) -> Option<Coordinate<T>> {
        let p = (p.0 as isize, p.1 as isize);
        let new_point = (p.0 + offset.0, p.1 + offset.1);

        if new_point.0 >= 0 && new_point.1 >= 0 {
            let x = new_point.0 as usize;
            let y = new_point.1 as usize;
            self.terrain_at_point((x, y))
                .map(|t| Coordinate::from_point((x, y), t))
        } else {
            None
        }
    }

    /// Iterate over the coordinates of the map
    pub fn iter(&self) -> CoordinateIter<T> {
        CoordinateIter::new(&self.map, self.width)
    }

    pub fn size(&self) -> usize {
        self.map.len()
    }
}

pub struct CoordinateIter<'a, T>
where
    T: Terrain,
{
    coordinates: &'a [T],
    width: usize,
    idx: usize,
}

impl<'a, T> CoordinateIter<'a, T>
where
    T: Terrain,
{
    fn new(coordinates: &'a [T], width: usize) -> Self {
        CoordinateIter {
            coordinates,
            width,
            idx: 0,
        }
    }
}

impl<'a, T> Iterator for CoordinateIter<'a, T>
where
    T: Terrain,
{
    type Item = Coordinate<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.coordinates.len() {
            return None;
        }

        let (x, y) = (self.idx % self.width, self.idx / self.width);

        let res = Coordinate::new(x, y, &self.coordinates[self.idx]);
        self.idx += 1;

        Some(res)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum MapError {
    FromTerrain(TerrainErr),
}

#[derive(Eq, PartialEq, Debug)]
pub struct Coordinate<'a, T>
where
    T: Terrain,
{
    x: usize,
    y: usize,
    terrain: &'a T,
}

impl<'a, T> Coordinate<'a, T>
where
    T: Terrain,
{
    fn new(x: usize, y: usize, terrain: &'a T) -> Self {
        Coordinate { x, y, terrain }
    }

    fn from_point(p: (usize, usize), terrain: &'a T) -> Self {
        Coordinate::new(p.0, p.1, terrain)
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn terrain(&self) -> &T {
        self.terrain
    }
}

/// The Terrain of a map.
pub trait Terrain: Eq + Sized + PartialEq + Debug {
    /// Given a `char c`, return the terrain if it matches one,
    /// otherwise it returns a `TerrainErr`
    fn from_char(c: char) -> Result<Self, TerrainErr>;

    fn to_char(&self) -> char;
}

#[derive(Eq, PartialEq, Debug)]
pub enum TerrainErr {
    UnknownTerrain(char),
}

#[cfg(test)]
mod tests {
    use crate::util::map::{Coordinate, Map, Terrain, TerrainErr};
    use std::cmp::Ordering;
    use std::cmp::Ordering::{Equal, Greater, Less};

    #[derive(Eq, PartialEq, Debug)]
    enum TestTerrain {
        Zero,
        One,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
    }

    impl Terrain for TestTerrain {
        fn from_char(c: char) -> Result<Self, TerrainErr> {
            match c {
                '0' => Ok(TestTerrain::Zero),
                '1' => Ok(TestTerrain::One),
                '2' => Ok(TestTerrain::Two),
                '3' => Ok(TestTerrain::Three),

                '4' => Ok(TestTerrain::Four),
                '5' => Ok(TestTerrain::Five),
                '6' => Ok(TestTerrain::Six),
                '7' => Ok(TestTerrain::Seven),

                '8' => Ok(TestTerrain::Eight),

                _ => Err(TerrainErr::UnknownTerrain(c)),
            }
        }

        fn to_char(&self) -> char {
            match self {
                TestTerrain::Zero => '0',
                TestTerrain::One => '1',
                TestTerrain::Two => '2',
                TestTerrain::Three => '3',
                TestTerrain::Four => '4',
                TestTerrain::Five => '5',
                TestTerrain::Six => '6',
                TestTerrain::Seven => '7',
                TestTerrain::Eight => '8',
            }
        }
    }

    fn comp(c0: &Coordinate<TestTerrain>, c1: &Coordinate<TestTerrain>) -> Ordering {
        if c0.y() < c1.y() {
            Less
        } else if c0.y() > c1.y() {
            Greater
        } else if c0.x() < c1.x() {
            Less
        } else if c0.x() > c1.x() {
            Greater
        } else {
            Equal
        }
    }

    #[test]
    fn test_simple_config() {
        let map = Map::<TestTerrain>::configure("13\n42").unwrap();
        assert_eq!(map.terrain_at(2, 0), None);
        assert_eq!(map.terrain_at(0, 2), None);
        assert_eq!(map.terrain_at(0, 0), Some(&TestTerrain::One));
        assert_eq!(map.terrain_at(1, 0), Some(&TestTerrain::Three));
    }

    #[test]
    fn test_retrieving_upper_left_neighbours() {
        let map = Map::<TestTerrain>::configure("012\n345\n678").unwrap();
        let mut n = map.neighbours(0, 0);
        n.sort_by(&comp);

        assert_eq!(
            n,
            [
                Coordinate::new(1, 0, &TestTerrain::One),
                Coordinate::new(0, 1, &TestTerrain::Three),
                Coordinate::new(1, 1, &TestTerrain::Four)
            ]
        )
    }

    #[test]
    fn test_retrieve_lower_right_neighbours() {
        let map = Map::<TestTerrain>::configure("012\n345\n678").unwrap();
        let mut n = map.neighbours(2, 2);
        n.sort_by(&comp);

        assert_eq!(
            n,
            [
                Coordinate::new(1, 1, &TestTerrain::Four),
                Coordinate::new(2, 1, &TestTerrain::Five),
                Coordinate::new(1, 2, &TestTerrain::Seven),
            ]
        )
    }

    #[test]
    fn test_retrieve_center() {
        let map = Map::<TestTerrain>::configure("012\n345\n678").unwrap();
        let mut n = map.neighbours(1, 1);
        n.sort_by(&comp);

        assert_eq!(
            n,
            [
                Coordinate::new(0, 0, &TestTerrain::Zero),
                Coordinate::new(1, 0, &TestTerrain::One),
                Coordinate::new(2, 0, &TestTerrain::Two),
                Coordinate::new(0, 1, &TestTerrain::Three),
                Coordinate::new(2, 1, &TestTerrain::Five),
                Coordinate::new(0, 2, &TestTerrain::Six),
                Coordinate::new(1, 2, &TestTerrain::Seven),
                Coordinate::new(2, 2, &TestTerrain::Eight)
            ]
        )
    }

    #[test]
    fn test_equality() {
        let map0 = Map::<TestTerrain>::configure("01\n43").unwrap();
        let map1 = Map::<TestTerrain>::configure("01\n43").unwrap();

        assert_eq!(map0, map1);
    }

    #[test]
    fn test_iter() {
        let map = Map::<TestTerrain>::configure("012\n345").unwrap();

        let cords: Vec<Coordinate<TestTerrain>> = map.iter().collect();

        assert_eq!(
            cords,
            [
                Coordinate::new(0, 0, &TestTerrain::Zero),
                Coordinate::new(1, 0, &TestTerrain::One),
                Coordinate::new(2, 0, &TestTerrain::Two),
                Coordinate::new(0, 1, &TestTerrain::Three),
                Coordinate::new(1, 1, &TestTerrain::Four),
                Coordinate::new(2, 1, &TestTerrain::Five),
            ]
        )
    }

    #[test]
    fn test_cord_at_point() {
        let map = Map::<TestTerrain>::configure(
            "\
            11223344\n\
            55667700\n\
            14234123\
            ",
        )
        .unwrap();

        assert_eq!(map.cord_at((1, 0), (0, -1)), None);
        assert_eq!(map.cord_at((1, 0), (-2, 0)), None);
        assert_eq!(
            map.cord_at((2, 1), (-1, -1)),
            Some(Coordinate::new(1, 0, &TestTerrain::One))
        );

        assert_eq!(map.cord_at((5, 2), (3, 0)), None);
        assert_eq!(map.cord_at((7, 1), (-1, 2)), None);
    }

    #[test]
    fn test_step_until() {
        let map = Map::<TestTerrain>::configure(
            "\
            1333113\n\
            5545777\n\
            1113557\n\
            1111111\
            ",
        )
        .unwrap();

        assert_eq!(
            map.step_until((6, 3), (-2, -1), |c| c.terrain.eq(&TestTerrain::Four)),
            Some(Coordinate::from_point((2, 1), &TestTerrain::Four))
        );
    }
}
