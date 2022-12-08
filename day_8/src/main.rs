use std::fs;

struct Forest {
    trees: Vec<Vec<u32>>,
}

impl Forest {
    fn build(input: &str) -> Self {
        Forest {
            trees: input
                .split("\n")
                .map(|line| {
                    line.chars()
                        .map(|tree| tree.to_digit(10).unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>(),
        }
    }

    fn get_row_col(&self, ii: usize, jj: usize) -> (Vec<u32>, Vec<u32>) {
        (
            self.trees[ii].clone(),
            self.trees[..].iter().map(|row| row[jj]).collect(),
        )
    }
}

mod a {
    use super::*;

    pub fn run(input: &str) -> usize {
        let forest = Forest::build(input);
        // assume grid MxN where M, N > 2, start with all outer trees as visible
        let mut visible: usize = (forest.trees.len() * 2) + (forest.trees.len() * 2) - 4;
        for ii in 1..(forest.trees.len() - 1) {
            for jj in 1..(forest.trees[0].len() - 1) {
                let (row, col) = forest.get_row_col(ii, jj);
                let tree = &forest.trees[ii][jj];

                let bottom_visible = col[(ii + 1)..].iter().max().unwrap() < tree;
                let top_visible = col[..ii].iter().max().unwrap() < tree;
                let left_visible = row[..jj].iter().max().unwrap() < tree;
                let right_visible = row[(jj + 1)..].iter().max().unwrap() < tree;
                if bottom_visible | top_visible | left_visible | right_visible {
                    visible += 1;
                }
            }
        }
        visible
    }
}

mod b {
    use super::*;

    fn find_blocker(trees: &[u32], base_height: u32) -> usize {
        trees
            .iter()
            .enumerate()
            .find(|(_, height)| height >= &&base_height)
            .map(|(index, _)| index + 1)
            .unwrap_or(trees.len())
    }

    pub fn run(input: &str) -> usize {
        let forest = Forest::build(input);

        let mut max_score = 0;

        for ii in 1..(forest.trees.len() - 1) {
            for jj in 1..(forest.trees[0].len() - 1) {
                let (row, col) = forest.get_row_col(ii, jj);
                let tree = forest.trees[ii][jj];

                let bottom_score = find_blocker(&col[(ii + 1)..], tree);
                let top_score = find_blocker(
                    &col[..ii]
                        .into_iter()
                        .map(|t| *t)
                        .rev()
                        .collect::<Vec<u32>>()[..],
                    tree,
                );
                let left_score = find_blocker(
                    &row[..jj]
                        .into_iter()
                        .map(|t| *t)
                        .rev()
                        .collect::<Vec<u32>>()[..],
                    tree,
                );
                let right_score = find_blocker(&row[(jj + 1)..], tree);
                let tree_score = bottom_score * top_score * left_score * right_score;

                if tree_score > max_score {
                    max_score = tree_score;
                }
            }
        }
        max_score
    }
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("input.txt file no worky")
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
        let input = fs::read_to_string("test.txt").expect("test.txt file no worky");
        assert_eq!(a::run(&input), 21);
        assert_eq!(b::run(&input), 8);
    }
}
