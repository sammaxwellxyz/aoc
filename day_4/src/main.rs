use std::collections::HashSet;
use std::fs;

fn parse_range(range: &str) -> HashSet<u32> {
    let [a_0, a_1]: [u32; 2] = range
        .split("-")
        .map(|p| p.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap();
    return HashSet::from_iter(a_0..(a_1 + 1));
}

mod a {
    use super::*;

    pub fn run(input: &str) -> usize {
        input
            .split("\n")
            .filter(|line| {
                let parts = line.split(",").collect::<Vec<&str>>();
                let elf_1 = parse_range(parts[0]);
                let elf_2 = parse_range(parts[1]);

                return elf_1.is_subset(&elf_2) || elf_2.is_subset(&elf_1);
            })
            .count()
    }
}

mod b {
    use super::*;

    pub fn run(input: &str) -> usize {
        input
            .split("\n")
            .filter(|line| {
                let parts = line.split(",").collect::<Vec<&str>>();
                let elf_1 = parse_range(parts[0]);
                let elf_2 = parse_range(parts[1]);
                return elf_1.intersection(&elf_2).count() > 0;
            })
            .count()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("input file no worky")
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
        let input = fs::read_to_string("test.txt").expect("input file no worky");
        assert_eq!(a::run(&input), 2);
        assert_eq!(b::run(&input), 4);
    }
}
