use std::fs;

mod a {
    use std::{collections::HashSet};
    pub fn run(input: &str) -> usize {
        input.bytes().collect::<Vec<u8>>()
            .windows(4)
            .position(|window| {
                HashSet::<u8>::from_iter(window.iter().cloned()).len() == 4
            }).unwrap() + 4
    }
}

mod b {
    use std::{collections::HashSet};
    pub fn run(input: &str) -> usize {
        input.bytes().collect::<Vec<u8>>()
            .windows(14)
            .position(|window| {
                println!("{:?}", window);
                HashSet::<u8>::from_iter(window.iter().cloned()).len() == 14
            }).unwrap() + 14
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input file no worky").clone();
    println!("a: {}", a::run(&input));
    println!("b: {}", b::run(&input));
}


#[cfg(test)]
mod tests {

    use super::*;

    macro_rules! eq_a_tests {
        ($($name:ident: $value:expr)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, a::run(input));
            }
        )*
        }
    }

    macro_rules! eq_b_tests {
        ($($name:ident: $value:expr)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, b::run(input));
            }
        )*
        }
    }
    
    eq_a_tests! {
        eq_a_0: ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)
        eq_a_1: ("nppdvjthqldpwncqszvftbrmjlhg", 6)
        eq_a_2: ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)
        eq_a_3: ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)
    }

    eq_b_tests! {
        eq_b_0: ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)
        eq_b_1: ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)
        eq_b_2: ("nppdvjthqldpwncqszvftbrmjlhg", 23)
        eq_b_3: ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)
        eq_b_4: ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)
    }
}
