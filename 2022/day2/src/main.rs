use std::env;
use std::fs;

const SCORE_FOR_ROCK: i32 = 1;
const SCORE_FOR_PAPER: i32 = 2;
const SCORE_FOR_SCISSORS: i32 = 3;

const SCORE_FOR_WIN: i32 = 6;
const SCORE_FOR_DRAW: i32 = 3;
const SCORE_FOR_LOSS: i32 = 0;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn part1_from_string(s: &str) -> Option<Shape> {
        if s == "A" || s == "X" {
            Some(Shape::Rock)
        } else if s == "B" || s == "Y" {
            Some(Shape::Paper)
        } else if s == "C" || s == "Z" {
            Some(Shape::Scissors)
        } else {
            None
        }
    }

    fn part2_from_string(s: &str) -> Option<Shape> {
        if s == "A" {
            Some(Shape::Rock)
        } else if s == "B" {
            Some(Shape::Paper)
        } else if s == "C" {
            Some(Shape::Scissors)
        } else {
            None
        }
    }

    fn part2_from_shape_and_outcome(shape: Shape, outcome: Outcome) -> Shape {
        match outcome {
            Outcome::Win => shape.shape_to_win(),
            Outcome::Draw => shape.shape_to_draw(),
            Outcome::Loss => shape.shape_to_lose(),
        }
    }

    fn shape_to_win(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn shape_to_draw(&self) -> Shape {
        *self
    }

    fn shape_to_lose(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn part2_from_string(s: &str) -> Option<Outcome> {
        match s {
            "X" => Some(Outcome::Loss),
            "Y" => Some(Outcome::Draw),
            "Z" => Some(Outcome::Win),
            _ => None,
        }
    }

    fn with_shapes(opponent: Shape, me: Shape) -> Outcome {
        match (opponent, me) {
            (Shape::Rock, Shape::Rock) => Outcome::Draw,
            (Shape::Rock, Shape::Paper) => Outcome::Win,
            (Shape::Rock, Shape::Scissors) => Outcome::Loss,
            (Shape::Paper, Shape::Rock) => Outcome::Loss,
            (Shape::Paper, Shape::Paper) => Outcome::Draw,
            (Shape::Paper, Shape::Scissors) => Outcome::Win,
            (Shape::Scissors, Shape::Rock) => Outcome::Win,
            (Shape::Scissors, Shape::Paper) => Outcome::Loss,
            (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
        }
    }
}

struct Round {
    me: Shape,
    outcome: Outcome,
}

impl Round {
    fn part1_from_string(s: &str) -> Option<Round> {
        let split: Vec<&str> = s.split(" ").collect();
        match (
            Shape::part1_from_string(split[0]),
            Shape::part1_from_string(split[1]),
        ) {
            (Some(opponents_shape), Some(my_shape)) => Some(Round {
                me: my_shape,
                outcome: Outcome::with_shapes(opponents_shape, my_shape),
            }),
            _ => None,
        }
    }

    fn part2_from_string(s: &str) -> Option<Round> {
        let split: Vec<&str> = s.split(" ").collect();
        match (
            Shape::part2_from_string(split[0]),
            Outcome::part2_from_string(split[1]),
        ) {
            (Some(opponents_shape), Some(expected_outcome)) => Some(Round {
                me: Shape::part2_from_shape_and_outcome(opponents_shape, expected_outcome),
                outcome: expected_outcome,
            }),
            _ => None,
        }
    }

    fn score(&self) -> i32 {
        self.score_for_my_shape() + self.score_for_outcome()
    }

    fn score_for_my_shape(&self) -> i32 {
        match self.me {
            Shape::Rock => SCORE_FOR_ROCK,
            Shape::Paper => SCORE_FOR_PAPER,
            Shape::Scissors => SCORE_FOR_SCISSORS,
        }
    }

    fn score_for_outcome(&self) -> i32 {
        match self.outcome {
            Outcome::Win => SCORE_FOR_WIN,
            Outcome::Draw => SCORE_FOR_DRAW,
            Outcome::Loss => SCORE_FOR_LOSS,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("Missing filename argument");

    let file_contents = fs::read_to_string(&filename).expect("Unable to read file");
    let lines = file_contents.lines();

    let mut number_of_rounds: i32 = 0;
    let mut part1_total_score: i32 = 0;
    let mut part2_total_score: i32 = 0;

    for line in lines {
        if let Some(round) = Round::part1_from_string(&line) {
            part1_total_score += round.score();
        }
        if let Some(round) = Round::part2_from_string(&line) {
            part2_total_score += round.score();
        }
        number_of_rounds += 1;
    }

    println!("Processed {} rounds", number_of_rounds);
    println!("Part 1: total score two shapes: {}", part1_total_score);
    println!(
        "Part 2: total score with expected outcomes: {}",
        part2_total_score
    );
}
