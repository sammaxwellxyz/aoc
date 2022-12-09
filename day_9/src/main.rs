use std::collections::HashSet;
use std::fs;

mod a {

    use super::*;

    pub fn run(input: &str) -> usize {
        input
            .split("\n")
            .flat_map(|line| {
                let mut head_instructions = line.split(" ");
                let direction = head_instructions
                    .next()
                    .expect("Should have a direction in the line");
                let moves = head_instructions
                    .next()
                    .expect("Should have a moves in the line")
                    .parse::<usize>()
                    .unwrap();
                match direction {
                    "R" => vec![(0, 1); moves],
                    "L" => vec![(0, -1); moves],
                    "D" => vec![(1, 0); moves],
                    "U" => vec![(-1, 0); moves],
                    other => panic!("didn't understand that instruction: {:?}", other),
                }
            })
            .fold(
                (HashSet::from([(0, 0)]), (0 as i32, 0 as i32), (0, 0)),
                |(mut tail_history, head, tail), head_move| {
                    let new_head = (head.0 + head_move.0, head.1 + head_move.1);
                    let new_tail: (i32, i32);
                    match (new_head.0 - tail.0, new_head.1 - tail.1) {
                        (m, n) if std::cmp::max(m.abs(), n.abs()) <= 1 => {
                            new_tail = tail;
                        }
                        _ => {
                            new_tail = head;
                            tail_history.insert(new_tail);
                        }
                    }
                    tail_history.insert(new_tail);
                    (tail_history, new_head, new_tail)
                },
            )
            .0
            .len()
    }
}

mod b {
    pub fn run(input: &str) -> usize {
        input.split("\n").count()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("input.txt file no worky")
        .clone();
    println!("a: {}", a::run(&input));
    println!("b: {}", b::run(&input));
}

#[cfg(test)]
mod tests {

    use super::{a, b};
    use std::fs;

    #[test]
    fn test() {
        let input = fs::read_to_string("test.txt").expect("test.txt file no worky");
        assert_eq!(a::run(&input), 13);
        assert_eq!(b::run(&input), 8);
    }
}
