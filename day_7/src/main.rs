use std::collections::HashSet;
use std::fs;
use std::num::ParseIntError;

enum Line<'a> {
    Change(&'a str),
    List,
    Dir(&'a str),
    File(u32, &'a str),
}

struct Tree<'a> {
    pub nodes: Vec<Directory<'a>>,
    current_directory: Option<usize>,
}
struct Directory<'a> {
    parent: Option<usize>,
    children: HashSet<usize>,
    size: u32,
    name: &'a str,
}

fn line_parse(input: &str) -> Result<Line, ParseIntError> {
    let line_parts = input.split(" ").collect::<Vec<&str>>();
    match (line_parts[0], line_parts[1]) {
        ("$", "ls") => Ok(Line::List),
        ("$", "cd") => Ok(Line::Change(line_parts[2])),
        ("dir", d) => Ok(Line::Dir(d)),
        (size, name) => Ok(Line::File(size.parse::<u32>()?, name)),
    }
}

impl<'a> Tree<'a> {
    fn new() -> Self {
        Self {
            nodes: vec![Directory {
                parent: None,
                children: HashSet::new(),
                size: 0,
                name: "/",
            }],
            current_directory: Some(0),
        }
    }

    fn build(input: &'a str) -> Self {
        input
            .split("\n")
            .skip(1)
            .fold(Tree::new(), |acc, line| match line_parse(line).unwrap() {
                Line::Change("..") => acc.change_to_parent(),
                Line::Change(child) => acc.change_to_child(child),
                Line::Dir(k) => acc.add_directory(k),
                Line::File(size, _) => acc.increase_size(size),
                _ => acc,
            })
    }

    fn add_directory(mut self, name: &'a str) -> Self {
        self.nodes.push(Directory {
            parent: self.current_directory,
            children: HashSet::new(),
            size: 0,
            name: name,
        });

        if let Some(curr) = self.current_directory {
            let added_node = self.nodes.len() - 1;
            self.nodes[curr].children.insert(added_node);
        }
        self
    }

    fn change_to_parent(mut self) -> Self {
        if let Some(curr) = self.current_directory {
            self.current_directory = self.nodes[curr].parent;
        }
        self
    }

    fn change_to_child(mut self, name: &'a str) -> Self {
        let child_index = self.nodes[self.current_directory.unwrap()]
            .children
            .iter()
            .find(|dir| self.nodes[**dir].name == name)
            .unwrap();
        self.current_directory = Some(*child_index);
        self
    }

    fn increase_size(mut self, index: u32) -> Self {
        self.nodes[self.current_directory.unwrap()].size += index;
        self
    }

    fn get_total_size(&self, index: usize) -> u32 {
        self.nodes[index].size
            + self.nodes[index]
                .children
                .iter()
                .map(|child_idx| self.get_total_size(*child_idx))
                .sum::<u32>()
    }
}

mod a {
    use super::*;

    pub fn run(input: &str) -> u32 {
        let dir_tree = Tree::build(input);
        dir_tree
            .nodes
            .iter()
            .enumerate()
            .filter_map::<u32, _>(|(idx, _)| {
                let size = dir_tree.get_total_size(idx);
                if size <= 100000 {
                    return Some(size);
                }
                None
            })
            .sum::<u32>()
    }
}

mod b {
    use super::*;

    pub fn run(input: &str) -> u32 {
        let dir_tree = Tree::build(input);
        let required_space = dir_tree.get_total_size(0) - 40000000;

        dir_tree
            .nodes
            .iter()
            .enumerate()
            .fold(None, |mut acc, (idx, _)| {
                let size = dir_tree.get_total_size(idx);
                if (size >= required_space) & (size < acc.unwrap_or(u32::MAX)) {
                    acc = Some(size);
                }
                acc
            })
            .unwrap()
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
        assert_eq!(a::run(&input), 95437);
        assert_eq!(b::run(&input), 24933642);
    }
}
