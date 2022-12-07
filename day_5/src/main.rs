use std::fs;

fn build_stacks(stack: &str) -> Vec<Vec<char>> {
    stack
        .split("\n")
        .filter(|line| line.contains("["))
        .fold(Vec::new(), |mut acc, level| {
            level.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .enumerate()
                .for_each(|(stack_idx, stack_box)| {
                    if acc.len() <= stack_idx {
                        if stack_box.contains(&'[') {
                            acc.push(vec![stack_box[1]]);
                        } else {
                            acc.push(vec![]);
                        }
                    } else {
                        if stack_box.contains(&'[') {
                            acc[stack_idx].insert(0, stack_box[1]);
                        }
                    }
                });
            acc
        })
}

mod a {
    use super::*;

    pub fn run(input: &str) -> String {
        let mut parts = input.split("\n\n");
        let mut stacks = build_stacks(parts.next().unwrap());
       
        parts.next().unwrap().split("\n").for_each(|mm| {
            let mut move_parts = mm.split(" ");
            let move_count = move_parts.nth(1).unwrap().parse::<usize>().unwrap();
            let move_from = move_parts.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            let move_to = move_parts.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            for _ in 0..move_count {
                let move_box = stacks[move_from].pop().unwrap();
                stacks[move_to].push(move_box);
            }
        });
        return stacks.iter().map(|stack| *stack.last().unwrap()).collect();
    }
}

mod b {
    use super::*;

    pub fn run(input: &str) -> String {
        let mut parts = input.split("\n\n");
        let mut stacks = build_stacks(parts.next().unwrap());

        parts.next().unwrap().split("\n").for_each(|mm| {
            let mut move_parts = mm.split(" ");
            let move_count = move_parts.nth(1).unwrap().parse::<usize>().unwrap();
            let move_from = move_parts.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            let move_to = move_parts.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            let stacke_len = stacks[move_from].len();
            let move_boxes = stacks[move_from].split_off(stacke_len - move_count);
            stacks[move_to].extend_from_slice(&move_boxes[..]);

        });
        return stacks.iter().map(|stack| *stack.last().unwrap()).collect();
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
        assert_eq!(a::run(&input), "CMZ");
        assert_eq!(b::run(&input), "MCD");
    }

}
