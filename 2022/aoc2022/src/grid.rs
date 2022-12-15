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

#[derive(Debug)]
struct Grid<T: Clone + Copy> {
    grid: Vec<Vec<T>>,
}

impl<T> Grid<T>
where
    T: Clone + Copy,
{
    pub fn new(size: usize, item: T) -> Grid<T> {
        let vectors = (0..size).map(|_| vec![item; size]).collect::<Vec<Vec<T>>>();
        Grid { grid: vectors }
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }

    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn get_at(&self, pt: &Point) -> Option<&T> {
        if let Some(row) = self.grid.get(pt.y as usize) {
            row.get(pt.x as usize)
        } else {
            None
        }
    }

    pub fn set_at(&mut self, pt: &Point, value: T) -> Result<(), &str> {
        if let Some(row) = self.grid.get_mut(pt.y as usize) {
            if pt.x < row.len() as i32 {
                row[pt.x as usize] = value;
                Ok(())
            } else {
                Err("Unable to set")
            }
        } else {
            Err("Unable to set")
        }
    }

    pub fn iter_points(&self) -> Box<dyn Iterator<Item = Point>> {
        let width = self.width();
        Box::new(
            (0..self.height())
                .map(move |y| (0..width).map(move |x| Point::new(x as i32, y as i32)))
                .flatten(),
        )
    }
}

impl<T: Clone + Copy> From<Vec<Vec<T>>> for Grid<T> {
    fn from(grid: Vec<Vec<T>>) -> Grid<T> {
        Grid { grid }
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
