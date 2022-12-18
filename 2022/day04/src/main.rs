use std::env;
use std::fs;
use std::ops::RangeInclusive;

#[derive(PartialEq, PartialOrd)]
struct RangeString(String);

impl From<&str> for RangeString {
    fn from(s: &str) -> Self {
        RangeString(s.to_string())
    }
}

impl From<RangeString> for RangeInclusive<u32> {
    fn from(s: RangeString) -> Self {
        let mut bounds = s.0.split("-");
        let start = u32::from_str_radix(bounds.next().unwrap(), 10).unwrap();
        let end = u32::from_str_radix(bounds.next().unwrap(), 10).unwrap();
        assert!(bounds.next() == None);
        start..=end
    }
}

struct Assignment {
    left: RangeInclusive<u32>,
    right: RangeInclusive<u32>,
}

impl Assignment {
    fn from_left_and_right_specifier(
        left_specifier: RangeString,
        right_specifier: RangeString,
    ) -> Assignment {
        Assignment {
            left: RangeInclusive::<u32>::from(left_specifier),
            right: RangeInclusive::<u32>::from(right_specifier),
        }
    }

    fn has_range_contained_by_other(&self) -> bool {
        let left_contains_right =
            self.left.contains(self.right.start()) && self.left.contains(self.right.end());
        let right_contains_left =
            self.right.contains(self.left.start()) && self.right.contains(self.left.end());

        left_contains_right || right_contains_left
    }

    fn has_overlapping_range(&self) -> bool {
        self.left.contains(self.right.start())
            || self.left.contains(self.right.end())
            || self.right.contains(self.left.start())
            || self.right.contains(self.left.end())
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("Missing filename argument");

    let file_contents = fs::read_to_string(&filename).expect("Unable to read file");

    let assignment_counts: (u32, u32, u32) = file_contents.lines()
        .map(|line| -> Assignment {
            let mut elves = line.split(",").map(|s| RangeString::from(s));
            let assignment = Assignment::from_left_and_right_specifier(
                elves.next().unwrap(),
                elves.next().unwrap(),
            );

            assert!(elves.next() == None);

            assignment
        })
        .fold((0, 0, 0), |acc, a| {
            let part1_counter = acc.1
                + if a.has_range_contained_by_other() {
                    1
                } else {
                    0
                };
            let part2_counter = acc.2 + if a.has_overlapping_range() { 1 } else { 0 };
            (acc.0 + 1, part1_counter, part2_counter)
        });

    println!("Processed {} assignments", assignment_counts.0);
    println!(
        "Part 1: number of assignments with one range containing the other: {}",
        assignment_counts.1
    );
    println!(
        "Part 2: number of assignments with ranges at least partially overlapping: {}",
        assignment_counts.2
    );
}
