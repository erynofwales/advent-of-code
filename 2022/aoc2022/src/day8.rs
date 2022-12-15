use crate::grid::{Direction, Point};
use std::collections::HashSet;

/*
type UnsignedPoint = Point;

const INPUT: &'static str = include_str!("../../Data/day8-input.txt");

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<i8>>,
}

impl Grid {
    fn new(grid: Vec<Vec<i8>>) -> Grid {
        Grid { grid }
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn tree_height_at(&self, at: UnsignedPoint) -> Option<i8> {
        Some(self.grid[at.y as usize][at.x as usize].clone())
    }

    fn scenic_score_at(&self, at: UnsignedPoint) -> i32 {
        let height = self.tree_height_at(at);

        let mut scores = [0, 0, 0, 0];
        for (i, d) in Direction::all().iter().cloned().enumerate() {
            for pt in self.iter_points_from_point_to_edge_in_direction(at, d) {
                if pt == at {
                    continue;
                }

                scores[i] += 1;

                if self.tree_height_at(pt) >= height {
                    break;
                }
            }
        }

        let total_score = scores.iter().product();
        total_score
    }

    fn iter_points(&self) -> Box<dyn Iterator<Item = UnsignedPoint>> {
        let width = self.width() as i32;
        Box::new(
            (0..self.height())
                .map(move |y| (0..width).map(move |x| UnsignedPoint::new(x as i32, y as i32)))
                .flatten(),
        )
    }

    fn iter_points_from_edge_to_point_in_direction(
        &self,
        point: UnsignedPoint,
        direction: Direction,
    ) -> Box<dyn Iterator<Item = UnsignedPoint>> {
        let width = self.width() as i32;
        let height = self.height() as i32;

        let closure: Box<dyn Fn(i32) -> UnsignedPoint> = match direction {
            Direction::North | Direction::South => {
                Box::new(move |y| UnsignedPoint::new(point.x, y as i32))
            }
            Direction::East | Direction::West => {
                Box::new(move |x| UnsignedPoint::new(x as i32, point.y))
            }
        };

        match direction {
            Direction::North => Box::new((0..point.y).map(closure)),
            Direction::East => Box::new((point.x..width).rev().map(closure)),
            Direction::South => Box::new((point.y..height).rev().map(closure)),
            Direction::West => Box::new((0..point.x).map(closure)),
        }
    }

    fn iter_points_from_point_to_edge_in_direction(
        &self,
        point: UnsignedPoint,
        direction: Direction,
    ) -> Box<dyn Iterator<Item = UnsignedPoint>> {
        let width = self.width() as i32;
        let height = self.height() as i32;

        let closure: Box<dyn Fn(i32) -> UnsignedPoint> = match direction {
            Direction::North | Direction::South => {
                Box::new(move |y| UnsignedPoint::new(point.x, y as i32))
            }
            Direction::East | Direction::West => Box::new(move |x| UnsignedPoint::new(x, point.y)),
        };

        match direction {
            Direction::North => Box::new((0..point.y).rev().map(closure)),
            Direction::East => Box::new((point.x..width).map(closure)),
            Direction::South => Box::new((point.y..height).map(closure)),
            Direction::West => Box::new((0..point.x).rev().map(closure)),
        }
    }

    fn print_with_visible_set(&self, visible_trees: &HashSet<UnsignedPoint>) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pt = UnsignedPoint::new(x as i32, y as i32);
                let height = self.tree_height_at(pt).unwrap();
                if visible_trees.contains(&pt) {
                    print!("\x1B[32m{}\x1B[0m", height);
                } else {
                    print!("{}", height);
                }
            }
            print!("\n");
        }
    }
}
*/

pub fn main(_filename: &str) -> std::io::Result<()> {
    /*
    let grid = Grid::new(
        INPUT
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| i8::from_str_radix(&c.to_string(), 10).unwrap())
                    .collect::<Vec<i8>>()
            })
            .collect(),
    );

    let mut visible_trees: HashSet<UnsignedPoint> = HashSet::new();
    let mut highest_scenic_score: i32 = -1;

    for grid_pt in grid.iter_points() {
        let mut tallest_tree_height: i8 = -1;

        for pt in grid.iter_points_from_edge_to_point_in_direction(grid_pt, Direction::North) {
            let tree_height = grid.tree_height_at(pt).unwrap();
            if tree_height > tallest_tree_height {
                tallest_tree_height = tree_height;
                visible_trees.insert(pt);
            }
        }

        tallest_tree_height = -1;
        for pt in grid.iter_points_from_edge_to_point_in_direction(grid_pt, Direction::East) {
            let tree_height = grid.tree_height_at(pt).unwrap();
            if tree_height > tallest_tree_height {
                tallest_tree_height = tree_height;
                visible_trees.insert(pt);
            }
        }

        tallest_tree_height = -1;
        for pt in grid.iter_points_from_edge_to_point_in_direction(grid_pt, Direction::South) {
            let tree_height = grid.tree_height_at(pt).unwrap();
            if tree_height > tallest_tree_height {
                tallest_tree_height = tree_height;
                visible_trees.insert(pt);
            }
        }

        tallest_tree_height = -1;
        for pt in grid.iter_points_from_edge_to_point_in_direction(grid_pt, Direction::West) {
            let tree_height = grid.tree_height_at(pt).unwrap();
            if tree_height > tallest_tree_height {
                tallest_tree_height = tree_height;
                visible_trees.insert(pt);
            }
        }

        let scenic_score = grid.scenic_score_at(grid_pt);
        if scenic_score > highest_scenic_score {
            highest_scenic_score = scenic_score;
        }
    }

    println!("Part 1: Number of visible trees: {}", &visible_trees.len());
    println!("Part 2: Highest scenic score: {}", highest_scenic_score);
    */

    Ok(())
}
