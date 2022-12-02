mod a {
    use std::cmp::Ordering;

    #[derive(PartialEq)]
    enum Hand {
        Rock,
        Paper,
        Scissors
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Hand::Rock, Hand::Rock) => Some(Ordering::Equal),
            (Hand::Rock, Hand::Paper) => Some(Ordering::Less),
            (Hand::Rock, Hand::Scissors) => Some(Ordering::Greater),
            (Hand::Paper, Hand::Rock) => Some(Ordering::Greater),
            (Hand::Paper, Hand::Paper) => Some(Ordering::Equal),
            (Hand::Paper, Hand::Scissors) => Some(Ordering::Less),
            (Hand::Scissors, Hand::Rock) => Some(Ordering::Less),
            (Hand::Scissors, Hand::Paper) => Some(Ordering::Greater),
            (Hand::Scissors, Hand::Scissors) => Some(Ordering::Equal)
        }
    }
    }

    trait Score {
        fn score(&self) -> u32;
    }

    impl Score for Hand {
        fn score(&self) -> u32 {
            match self {
                Hand::Rock => 1,
                Hand::Paper => 2,
                Hand::Scissors => 3
            }
        }
    }

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
                match opposition.partial_cmp(&me).unwrap() {
                    Ordering::Less => me.score() + 6,
                    Ordering::Equal => me.score() + 3,
                    Ordering::Greater => me.score() + 0,
                }
            })
            .sum()
    }
}

mod b {
    use std::cmp::Ordering;

    #[derive(PartialEq)]
    enum Hand {
        Rock,
        Paper,
        Scissors
    }

    trait Battle {
        fn beats(&self) -> Self;
        fn loses_to(&self) -> Self;
    }

    impl Battle for Hand {
        fn beats(&self) -> Self {
            match &self {
                Hand::Rock => Hand::Scissors,
                Hand::Paper => Hand::Rock,
                Hand::Scissors => Hand::Paper,
            }
        }
        fn loses_to(&self) -> Self {
            match &self {
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissors,
                Hand::Scissors => Hand::Rock,
            }
        }
    }
    trait Score {
        fn score(&self) -> u32;
    }

    impl Score for Hand {
        fn score(&self) -> u32 {
            match self {
                Hand::Rock => 1,
                Hand::Paper => 2,
                Hand::Scissors => 3
            }
        }
    }

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
                match (opposition, outcome) {
                    (op, Ordering::Less) => op.beats().score() + 0,
                    (op, Ordering::Greater) => op.loses_to().score() + 6,
                    (op, Ordering::Equal) => op.score() + 3,
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
