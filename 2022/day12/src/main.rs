use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{env, fs};

trait Elevation {
    fn elevation(&self) -> Option<u32>;
}

impl Elevation for char {
    fn elevation(&self) -> Option<u32> {
        let value = match self {
            'S' => 'a',
            'E' => 'z',
            _ => *self
        };

        Some((value as u32) - 'a' as u32)
    }
}

#[derive(Debug)]
struct Square {
    symbol: char,
    up: Option<usize>,
    down: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

impl Square {
    fn new(symbol: char) -> Square {
        Square { symbol, up: None, down: None, left: None, right: None }
    }

    fn elevation(&self) -> u32 {
        self.symbol.elevation().unwrap()
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    node: usize,
    cost: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// An implemetation of Dijkstra's algorithm to find the shortest path from start to end and return
/// its length.
fn find_length_of_shortest_path(squares: &Vec<Square>, start: usize, end: usize) -> Option<u32> {
    let mut distances: Vec<_> = (0..squares.len()).map(|_| u32::MAX).collect();
    let mut heap = BinaryHeap::new();

    // Initial state
    distances[start] = 0;
    heap.push(State { node: start, cost: 0 });

    while let Some(State { node, cost }) = heap.pop() {
        if node == end {
            return Some(cost);
        }

        if cost > distances[node] {
            continue;
        }

        let square = &squares[node];
        for edge in &[square.up, square.right, square.down, square.left] {
            if edge.is_none() {
                continue;
            }

            let edge = edge.unwrap();

            let next_cost = cost + 1;
            let next = State { node: edge, cost: next_cost };
            if next.cost < distances[edge] {
                heap.push(next);
                distances[edge] = next_cost;
            }
        }
    }

    None
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("Missing filename argument");

    let file_contents = fs::read_to_string(&filename).expect("Unable to read file");
    let character_grid = file_contents.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    // Assume a square grid
    let width = character_grid[0].len();
    let height = character_grid.len();

    let index_into_squares_array = |x: usize, y: usize| -> usize {
        y * width + x
    };

    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut squares: Vec<Square> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let symbol = character_grid[y][x];

            match symbol {
                'S' => start = index_into_squares_array(x, y),
                'E' => end = index_into_squares_array(x, y),
                _ => {},
            }

            let elevation = symbol.elevation().unwrap();
            let mut square = Square::new(symbol);

            if y > 0 {
                let up_elevation = character_grid[y-1][x].elevation().unwrap();
                if up_elevation <= elevation + 1 {
                    square.up = Some(index_into_squares_array(x, y - 1));
                }
            }

            if y < height - 1 {
                let down_elevation = character_grid[y + 1][x].elevation().unwrap();
                if down_elevation <= elevation + 1 {
                    square.down = Some(index_into_squares_array(x, y + 1));
                }
            }

            if x > 0 {
                let left_elevation = character_grid[y][x - 1].elevation().unwrap();
                if left_elevation <= elevation + 1 {
                    square.left = Some(index_into_squares_array(x - 1, y));
                }
            }

            if x < width - 1 {
                let right_elevation = character_grid[y][x + 1].elevation().unwrap();
                if right_elevation <= elevation + 1 {
                    square.right = Some(index_into_squares_array(x + 1, y));
                }
            }

            squares.push(square);
        }
    }

    let length_of_shortest_path = find_length_of_shortest_path(&squares, start, end).unwrap();
    println!("Part 1: length of shortest path to location with best signal: {length_of_shortest_path}");

    let mut paths_from_all_a_squares: Vec<(usize, u32)> = squares
        .iter()
        .enumerate()
        .filter(|(_, sq)| sq.elevation() == 0)
        .map(|(i, _)| (i, find_length_of_shortest_path(&squares, i, end)))
        .filter(|(_, distance)| distance.is_some())
        .map(|(i, distance)| (i, distance.unwrap()))
        .collect();
    paths_from_all_a_squares.sort_by(|a, b| b.1.cmp(&a.1));

    dbg!(&paths_from_all_a_squares.last()); 
}
