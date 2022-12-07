use std::fs;

mod a {
    pub fn run(input: &str) -> usize {
        input.split("\n").count()
    }
}

mod b {
    pub fn run(input: &str) -> usize {
        input.split("\n").count()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt file no worky").clone();
    println!("a: {}", a::run(&input));
    println!("b: {}", b::run(&input));
}


#[cfg(test)]
mod tests {

    use super::{a,b};
    use std::fs;

    #[test]
    fn test() {
        let input = fs::read_to_string("test.txt").expect("test.txt file no worky");
        assert_eq!(a::run(&input), 1);
        assert_eq!(b::run(&input), 1);
    }

}
