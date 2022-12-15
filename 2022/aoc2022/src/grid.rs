#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn all() -> &'static [Direction] {
        &[
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }

    pub fn from_relative_direction(letter: &str) -> Option<Direction> {
        match letter {
            "U" | "u" => Some(Direction::North),
            "R" | "r" => Some(Direction::East),
            "D" | "d" => Some(Direction::South),
            "L" | "l" => Some(Direction::West),
            _ => None,
        }
    }
}

/**
 * A point in standard coordinates, where Y values grow in the positively in the North direction.
 */
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn zero() -> Point {
        Point::new(0, 0)
    }

    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn distance_to(&self, other: &Point) -> (i32, i32) {
        (other.x - self.x, other.y - self.y)
    }

    pub fn move_by_one_in(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.y += 1,
            Direction::East => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
