use std::fs;

mod a {
    use std::collections::HashSet;

    pub fn run(input: &str) -> usize {
        input.split("\n")
            .filter(|line| {
                let mut parts = line.split(",");
                let mut a_range = parts.next().unwrap().split("-");
                let elf_1: HashSet<i32> = HashSet::from_iter((a_range.next().unwrap().parse::<i32>().unwrap())..(a_range.next().unwrap().parse::<i32>().unwrap() + 1));
                let mut b_range = parts.next().unwrap().split("-");
                let elf_2: HashSet<i32> = HashSet::from_iter((b_range.next().unwrap().parse::<i32>().unwrap())..(b_range.next().unwrap().parse::<i32>().unwrap() + 1));
                return elf_1.is_subset(&elf_2) || elf_2.is_subset(&elf_1)
            })
            .count()
    }
}

mod b {
    use std::collections::HashSet;

    pub fn run(input: &str) -> usize {
        input.split("\n")
            .filter(|line| {
                let mut parts = line.split(",");
                let mut a_range = parts.next().unwrap().split("-");
                let elf_1: HashSet<i32> = HashSet::from_iter((a_range.next().unwrap().parse::<i32>().unwrap())..(a_range.next().unwrap().parse::<i32>().unwrap() + 1));
                let mut b_range = parts.next().unwrap().split("-");
                let elf_2: HashSet<i32> = HashSet::from_iter((b_range.next().unwrap().parse::<i32>().unwrap())..(b_range.next().unwrap().parse::<i32>().unwrap() + 1));
                return elf_1.intersection(&elf_2).count() > 0

            })
            .count()
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
        assert_eq!(a::run(&input), 2);
        assert_eq!(b::run(&input), 4);
    }

}
