use std::fs;

fn a(input: &str) -> &str {
    input
}
fn b(input: &str) -> &str {
    input
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input file no worky");
    println!("a: {}", a(&input));
    println!("b: {}", b(&input));
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let input = fs::read_to_string("test.txt").expect("input file no worky");
        assert_eq!(a(&input), "");
        assert_eq!(b(&input), "");
    }
}
