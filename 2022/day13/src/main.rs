use std::convert::TryFrom;
use std::{cmp, env, fmt, fs};

type Pair = (Packet, Packet);
type Int = u32;

#[derive(Clone, Eq, PartialEq)]
enum Datum {
    Int(Int),
    List(Vec<Datum>),
}

impl Datum {
    fn is_int(&self) -> bool {
        match self {
            Datum::Int(_) => true,
            _ => false,
        }
    }

    fn list_mut<'a>(&'a mut self) -> Option<&'a mut Vec<Datum>> {
        match self {
            Datum::List(ref mut list) => Some(list),
            _ => None,
        }
    }
}

impl cmp::PartialOrd for Datum {
    fn partial_cmp(&self, other: &Datum) -> Option<cmp::Ordering> {
        println!("  Compare {} vs {}", self, other);
        match (self, other) {
            (Datum::Int(self_value), Datum::Int(other_value)) => {
                self_value.partial_cmp(other_value)
            }
            (Datum::Int(_), Datum::List(_)) => {
                let self_list = Datum::List(vec![self.clone()]);
                self_list.partial_cmp(other)
            }
            (Datum::List(_), Datum::Int(_)) => {
                let other_list = Datum::List(vec![other.clone()]);
                self.partial_cmp(&other_list)
            }
            (Datum::List(self_list), Datum::List(other_list)) => self_list.partial_cmp(other_list),
        }
    }
}

impl fmt::Debug for Datum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Datum::Int(value) => write!(f, "Int({})", value),
            Datum::List(list) => {
                let strings: Vec<String> = list.iter().map(|d| format!("{}", d)).collect();
                write!(f, "[{}]", strings.join(","))
            }
        }
    }
}

impl fmt::Display for Datum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Datum::Int(value) => write!(f, "{}", value),
            Datum::List(list) => {
                let strings: Vec<String> = list.iter().map(|d| format!("{}", d)).collect();
                write!(f, "[{}]", strings.join(","))
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Packet(Datum);

impl TryFrom<&str> for Packet {
    type Error = &'static str;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut parsing_stack = Vec::new();
        let mut has_trailing_list_item = false;

        for c in line.chars() {
            match c {
                '[' => parsing_stack.push(Datum::List(Vec::new())),
                ']' => {
                    let top_item = parsing_stack
                        .pop()
                        .expect("Found ']' with no items on the stack");

                    if top_item.is_int() || has_trailing_list_item {
                        //println!("Pushing {:?} onto List", &top_item);
                        parsing_stack
                            .last_mut()
                            .expect("Found ']' and int on the stack, with no other items")
                            .list_mut()
                            .expect("Second item from top wasn't a List")
                            .push(top_item);
                    } else {
                        parsing_stack.push(top_item);
                    }

                    has_trailing_list_item = true;
                }
                c if c.is_ascii_digit() => {
                    let c_digit: Int = c.to_digit(10).unwrap();
                    match parsing_stack.last_mut() {
                        Some(Datum::Int(accumulated_int)) => {
                            *accumulated_int = *accumulated_int * 10 + c_digit
                        }
                        _ => parsing_stack.push(Datum::Int(c_digit)),
                    }
                }
                ',' => {
                    let completed_item = parsing_stack
                        .pop()
                        .expect("Found ',' with no items on the stack");
                    parsing_stack
                        .last_mut()
                        .expect("Found ']' and int on the stack, with no other items")
                        .list_mut()
                        .expect("Second item from top wasn't a List")
                        .push(completed_item);
                    has_trailing_list_item = false;
                }
                _ => assert!(false, "Invalid character"),
            }
        }

        if has_trailing_list_item && parsing_stack.len() > 1 {
            let top_item = parsing_stack
                .pop()
                .expect("Found ']' with no items on the stack");
            parsing_stack
                .last_mut()
                .expect("Found ']' and int on the stack, with no other items")
                .list_mut()
                .expect("Second item from top wasn't a List")
                .push(top_item);
        }

        assert!(parsing_stack.len() == 1);

        let top_item = parsing_stack
            .pop()
            .expect("Missing completed packet after parsing all characters on line");
        let packet = Packet(top_item);

        assert!(format!("{}", packet) == line);

        Ok(packet)
    }
}

impl cmp::PartialOrd for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<cmp::Ordering> {
        println!("Compare {} vs {}", self.0, other.0);

        let ordering = self.0.partial_cmp(&other.0);
        println!("  -> {:?}", ordering);

        ordering
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("Missing filename argument");

    let file_contents = fs::read_to_string(&filename).expect("Unable to read file");
    let lines = file_contents.lines();

    let mut pairs: Vec<Pair> = Vec::new();
    let mut completed_packets: Vec<Packet> = Vec::new();

    for line in lines {
        if line == "" {
            let right = completed_packets.pop().expect("Missing right packet");
            let left = completed_packets.pop().expect("Missing left packet");
            println!("Completed pair!");

            assert!(completed_packets.is_empty());

            pairs.push((left, right));
            continue;
        }

        let packet = Packet::try_from(line).expect("Unable to parse packet!");
        println!("Completed packet! {}", &packet);
        completed_packets.push(packet);
    }

    let right = completed_packets.pop().expect("Missing right packet");
    let left = completed_packets.pop().expect("Missing left packet");
    println!("Completed pair!");
    pairs.push((left, right));

    let pairs_in_right_order = pairs
        .iter()
        .enumerate()
        .map(|(i, p)| (i + 1, p))
        .filter(|(_, pair)| {
            if let Some(ordering) = pair.0.partial_cmp(&pair.1) {
                ordering == cmp::Ordering::Less
            } else {
                false
            }
        })
        .collect::<Vec<(usize, &Pair)>>();

    println!("----- Pairs in the Right Order -----");
    for (i, p) in &pairs_in_right_order {
        println!("{} ->\n  {}\n  {}", i, p.0, p.1);
    }
    println!("{} pairs", pairs.len());
    println!(
        "Sum of indicies of pairs in correct order: {}",
        &pairs_in_right_order.iter().map(|(i, _)| i).sum::<usize>()
    );
}

#[cfg(test)]
mod test {
    use crate::{Datum, Packet};
    use std::cmp::Ordering;

    #[test]
    fn shorter_vecs_are_less() {
        assert!(vec![].partial_cmp(&vec![3]).unwrap() == Ordering::Less);
        assert!(vec![1, 2, 3].partial_cmp(&vec![1, 2, 3, 4]).unwrap() == Ordering::Less);
    }

    #[test]
    fn single_empty_list() {
        let packet = Packet::try_from("[]").unwrap();
        assert!(packet == Packet(Datum::List(vec![])));
    }

    #[test]
    fn three_int_list() {
        let packet = Packet::try_from("[1,2,3]").unwrap();
        assert!(
            packet
                == Packet(Datum::List(vec![
                    Datum::Int(1),
                    Datum::Int(2),
                    Datum::Int(3),
                ])),
        );
    }

    #[test]
    fn one_embedded_empty_list() {
        let packet = Packet::try_from("[1,2,[],3,4]").unwrap();
        assert!(
            packet
                == Packet(Datum::List(vec![
                    Datum::Int(1),
                    Datum::Int(2),
                    Datum::List(vec![]),
                    Datum::Int(3),
                    Datum::Int(4),
                ])),
        );
    }

    #[test]
    fn double_embedded_list() {
        let packet = Packet::try_from("[[9,[8,7,8],[]]]").unwrap();
        assert!(
            packet
                == Packet(Datum::List(vec![Datum::List(vec![
                    Datum::Int(9),
                    Datum::List(vec![Datum::Int(8), Datum::Int(7), Datum::Int(8)]),
                    Datum::List(vec![])
                ])]))
        );
    }

    #[test]
    fn first_input_packet() {
        let packet = Packet::try_from("[[[7,8],5],[[9,[8,7,8],[],[2,4,10,10],[2,10,8,3,3]],[],[[6,1,10],[],3,6],[3]],[],[4],[3,0,1,10]]").unwrap();
        assert!(
            packet
                == Packet(Datum::List(vec![
                    Datum::List(vec![
                        Datum::List(vec![Datum::Int(7), Datum::Int(8)]),
                        Datum::Int(5)
                    ]),
                    Datum::List(vec![
                        Datum::List(vec![
                            Datum::Int(9),
                            Datum::List(vec![Datum::Int(8), Datum::Int(7), Datum::Int(8)]),
                            Datum::List(vec![]),
                            Datum::List(vec![
                                Datum::Int(2),
                                Datum::Int(4),
                                Datum::Int(10),
                                Datum::Int(10)
                            ]),
                            Datum::List(vec![
                                Datum::Int(2),
                                Datum::Int(10),
                                Datum::Int(8),
                                Datum::Int(3),
                                Datum::Int(3)
                            ])
                        ]),
                        Datum::List(vec![]),
                        Datum::List(vec![
                            Datum::List(vec![Datum::Int(6), Datum::Int(1), Datum::Int(10)]),
                            Datum::List(vec![]),
                            Datum::Int(3),
                            Datum::Int(6)
                        ]),
                        Datum::List(vec![Datum::Int(3)])
                    ]),
                    Datum::List(vec![]),
                    Datum::List(vec![Datum::Int(4)]),
                    Datum::List(vec![
                        Datum::Int(3),
                        Datum::Int(0),
                        Datum::Int(1),
                        Datum::Int(10)
                    ])
                ]))
        );
    }
}
