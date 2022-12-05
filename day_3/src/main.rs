use std::fs;

const AZ: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

mod a {
    use crate::AZ;

    fn line_value(line: &str) -> usize {
        let pack_1 = &line[..line.len() / 2];
        let pack_2 = &line[(line.len() / 2)..];
        let match_index = pack_1.find(|c: char| pack_2.contains(c)).unwrap();
        let match_char = pack_1.chars().nth(match_index).unwrap();
        let score = AZ.chars().position(|c| c == match_char).unwrap();
        return score + 1;
    }

    pub fn run(input: &str) -> usize {
        input.split("\n").map(line_value).sum()
    }
}

mod b {
    use crate::AZ;
    use std::collections::HashSet;


    fn group_value(lines: &[&str]) -> usize {
        let pack_1: HashSet<char> = HashSet::from_iter(lines[0].chars());
        let pack_2:HashSet<char> = HashSet::from_iter(lines[1].chars());
        let pack_3:HashSet<char> = HashSet::from_iter(lines[2].chars());

        let match_char: HashSet<char> = pack_1.intersection(&pack_2)
            .cloned()
            .collect::<HashSet<char>>()
            .intersection(&pack_3)
            .cloned()
            .collect();
        let score = AZ.chars().position(|c| &c == match_char.iter().next().unwrap()).unwrap();
        return score + 1;
    }

    pub fn run(input: &str) -> usize {
        input.split("\n").collect::<Vec<&str>>().chunks(3).map(group_value).sum()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input file no worky").clone();
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
        assert_eq!(a::run(&input), 157);
        assert_eq!(b::run(&input), 70);
    }

}
