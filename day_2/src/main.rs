use std::cmp::Ordering;

#[derive(PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors
}

trait Cyclable {
    fn next(&self) -> Self;
    fn previous(&self) -> Self;
}

impl Cyclable for Hand {
    fn next(&self) -> Self {
        match &self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }
    fn previous(&self) -> Self {
        match &self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
}
trait Score {
    fn score(&self) -> u32;
}

impl Score for Hand {
    fn score(&self) -> u32 {
        (*self as u32) + 1
    }
}

mod a {
    use super::*;

    impl From<&str> for Hand {
        fn from(code: &str) -> Self {
            match code {
                "A" | "X" => Hand::Rock,
                "B" | "Y" => Hand::Paper,
                "C" | "Z" => Hand::Scissors,
                _ => panic!("unrecognised hand symbol")
            }
        }
    }

    pub fn run(input: &str) -> u32 {
        input.split("\n")
            .map(|round| round.split(" ").map(|code| code.into()).collect::<Vec<Hand>>())
            .map(|hands| {
                let opposition = &hands[0];
                let me = &hands[1];
                if me == &opposition.next() {
                    return  me.score() + 6
                } else if me == &opposition.previous() {
                    me.score() + 0
                } else {
                    me.score() + 3
                }
            })
            .sum()
    }
}

mod b {

    use super::*;

    fn code_to_hand(code: &str) -> Hand {
        match code {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!("unrecognised hand symbol")
        }
    }

    fn code_to_outcome(code: &str) -> Ordering {
        match code {
            "X" => Ordering::Less,
            "Y" => Ordering::Equal,
            "Z" => Ordering::Greater,
            _ => panic!("unrecognised hand symbol")
        }
    }

    pub fn run(input: &str) -> u32 {
        input.split("\n")
            .map(|round| round.split(" ").collect::<Vec<&str>>())
            .map(|codes| (code_to_hand(codes[0]), code_to_outcome(codes[1])))
            .map(|(opposition, outcome)| {
                match outcome {
                    Ordering::Less => opposition.previous().score() + 0,
                    Ordering::Greater => opposition.next().score() + 6,
                    Ordering::Equal => opposition.score() + 3,
                }
            })
            .sum()
    }
}

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("input file no worky");
    println!("a: {}", a::run(&input));
    println!("b: {}", b::run(&input));
}


#[cfg(test)]
mod tests {

    use super::{a,b};
    use std::fs;


    #[test]
    fn test() {
        let input = fs::read_to_string("test.txt").expect("input file no worky");
        assert_eq!(a::run(&input), 15);
        assert_eq!(b::run(&input), 12);
    }

}
