use std::fs;

fn day_1_a(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .filter(|cal| cal != &"")
                .map(|cal| cal.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

fn day_1_b(input: &str) -> u32 {
    let mut sorted_elfs = input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .filter(|cal| cal != &"")
                .map(|cal| cal.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    sorted_elfs.sort();
    sorted_elfs.iter().rev().take(3).sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input file no worky");
    println!("a: {}", day_1_a(&input));
    println!("b: {}", day_1_b(&input));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let input = fs::read_to_string("test.txt").expect("input file no worky");
        assert_eq!(day_1_a(&input), 24000);
        assert_eq!(day_1_b(&input), 45000);
    }
}
