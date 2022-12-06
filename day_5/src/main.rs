use std::fs;


mod a {
    pub fn run(input: &str) -> String {
        let mut parts = input.split("\n\n");
        let mut stacks = parts.next().unwrap()
            .split("\n")
            .fold(Vec::new(), |mut acc, level| {
                if level.contains(" 1 ") {
                    return acc;
                }
                level.chars().iter().chunks(4).enumerate().for_each(|(stack_idx, stack_box)| {
                    println!("{:?} {:?}", stack_idx, stack_box);
                    stack_box.filter();
                    if acc.len() <= stack_idx {
                        if stack_box.contains(&"[") {
                            acc.push(vec![stack_box[1]]);
                        } else {
                            acc.push(vec![]);
                        }
                    } else {
                        if stack_box.contains(&"[") {
                            acc[stack_idx].insert(0, stack_box[1]);
                        }
                    }
                });
                acc
            });
        println!("shhaa {:?}", stacks);
        parts.next().unwrap().split("\n").for_each(|mm| {
            let mut move_parts = mm.split(" ");
            let move_count = move_parts.nth(1).unwrap().parse::<usize>().unwrap();
            let move_from = move_parts.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            let move_to = move_parts.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            for _ in 0..move_count {
                println!("{:?} {:?}", stacks,move_from );
                let move_box = stacks[move_from].pop().unwrap();
                stacks[move_to].push(move_box);
            }
        });
        println!("{:?}", stacks);
        return stacks.iter().map(|stack| *stack.last().unwrap()).collect();
    }
}

mod b {
    pub fn run(input: &str) -> &str {
        println!("{:?}", input);
        return "CMZ"
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
        assert_eq!(b::run(&input), "CMZ");
    }

}
