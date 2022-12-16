use geometry::{Direction, Point};
use std::collections::HashSet;
use std::{env, fs};

type SignedPoint = Point;

struct Rope {
    nodes: Vec<SignedPoint>,
}

impl Rope {
    fn with_length(length: usize) -> Rope {
        Rope {
            nodes: vec![SignedPoint::zero(); length],
        }
    }

    fn move_head(&mut self, direction: Direction) {
        if let Some(head) = self.nodes.first_mut() {
            head.move_by_one_in(direction);
            for i in 1..self.nodes.len() {
                let first = self.nodes[i - 1].clone();
                let mut second = self.nodes.get_mut(i).unwrap();

                let distance = second.distance_to(&first);
                if distance.0 == 2 {
                    second.x += 1;
                    if distance.1 < 0 {
                        second.y -= 1;
                    } else if distance.1 > 0 {
                        second.y += 1;
                    }
                } else if distance.0 == -2 {
                    second.x -= 1;
                    if distance.1 < 0 {
                        second.y -= 1;
                    } else if distance.1 > 0 {
                        second.y += 1;
                    }
                } else if distance.1 == 2 {
                    second.y += 1;
                    if distance.0 < 0 {
                        second.x -= 1;
                    } else if distance.0 > 0 {
                        second.x += 1;
                    }
                } else if distance.1 == -2 {
                    second.y -= 1;
                    if distance.0 < 0 {
                        second.x -= 1;
                    } else if distance.0 > 0 {
                        second.x += 1;
                    }
                }
            }
        }
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("Missing filename argument");

    let file_contents = fs::read_to_string(&filename).expect("Unable to read file");

    let mut visited_points: HashSet<SignedPoint> = HashSet::new();
    let mut long_rope_visited_points: HashSet<SignedPoint> = HashSet::new();

    let mut rope = Rope::with_length(2);
    let mut long_rope = Rope::with_length(10);

    for line in file_contents.lines() {
        let mut split_line = line.split(" ");

        let direction = Direction::from_relative_direction(split_line.next().unwrap()).unwrap();
        let count = u8::from_str_radix(split_line.next().unwrap(), 10).unwrap();

        for _ in 0..count {
            rope.move_head(direction);
            if let Some(last_node) = rope.nodes.last() {
                visited_points.insert(last_node.clone());
            }

            long_rope.move_head(direction);
            if let Some(last_node) = long_rope.nodes.last() {
                long_rope_visited_points.insert(last_node.clone());
            }
        }
    }

    println!(
        "Part 1: number of points tail node visited: {}",
        &visited_points.len()
    );
    println!(
        "Part 2: number of points tail node of long rope visited: {}",
        &long_rope_visited_points.len()
    );
}
