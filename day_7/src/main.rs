use std::fs;
use std::collections::{HashSet};

#[derive(Debug)]
enum Line<'a> {
    Change(&'a str),
    List,
    Dir(&'a str),
    File(u32, &'a str)
}

#[derive(Debug)]
struct Tree<'a> {
    pub nodes: Vec<Directory<'a>>,
    current_directory: Option<usize>,
}
#[derive(Debug)]
struct Directory<'a> {
    parent: Option<usize>,
    children: HashSet<usize>,
    size: u32,
    name: &'a str
}

fn line_parse(input: &str) -> Line {
    let line_parts = input.split(" ").collect::<Vec<&str>>();
    match (line_parts[0], line_parts[1]) {
        ("$", "ls") => Line::List,
        ("$", "cd") => Line::Change(line_parts[2]),
        ("dir", d) => Line::Dir(d),
        (size, name) => Line::File(size.parse::<u32>().unwrap(), name)
    }
}

impl <'a> Tree<'a> {
    fn new() -> Self {
        Self { nodes: vec!(Directory { parent: None, children: HashSet::new(), size: 0, name: "/" }), current_directory: Some(0) }
    }

    fn add_directory(&mut self, name: &'a str) {

        let dir = Directory {
            parent: self.current_directory,
            children: HashSet::new(),
            size: 0,
            name: name
        };
        self.nodes.push(dir);

        if let Some(curr) = self.current_directory {
            let child_dir = self.nodes.len() - 1;
            self.nodes[curr].children.insert(child_dir);
        }
    }

    fn change_to_parent(&mut self) {
        if let Some(curr) = self.current_directory {
            self.current_directory = self.nodes[curr].parent
        }
    }

    fn change_to_child(&mut self, name: &'a str) {
        let child_index = self.nodes[self.current_directory.unwrap()].children.iter().find(|dir| self.nodes[**dir].name == name).unwrap();
        self.current_directory = Some(*child_index)
    }
    
    fn increase_size(&mut self, index: u32) {
        self.nodes[self.current_directory.unwrap()].size += index;
    }

    fn get_total_size(&self, index: usize) -> u32 {
        self.nodes[index].size + self.nodes[index].children.iter().map(|child_idx| { self.get_total_size(*child_idx) }).sum::<u32>()
    }

}

mod a {
    use super::*;

    pub fn run(input: &str) -> u32 {
        let dir_tree = input.split("\n").skip(1).fold(Tree::new(), |mut acc, line| {
            match line_parse(line) {
                Line::Change(target) => {
                    match target {
                        ".." => { acc.change_to_parent() },
                        m => { acc.change_to_child(m) }
                    }
                }
                Line::Dir(k) => {
                    acc.add_directory(k);
                },
                Line::File(size, _) => {
                    acc.increase_size(size);
                }
                _ => {}
            }
            acc
        });
        dir_tree.nodes.iter().enumerate().filter_map::<u32, _>(|(idx, _)| {
            let size = dir_tree.get_total_size(idx);
            if size <= 100000 {
                return Some(size);
            }
            None
        }).sum::<u32>()

    }
}

mod b {
    use super::*;

    pub fn run(input: &str) -> u32 {
        let dir_tree = input.split("\n").skip(1).fold(Tree::new(), |mut acc, line| {
            match line_parse(line) {
                Line::Change(target) => {
                    match target {
                        ".." => { acc.change_to_parent() },
                        m => { acc.change_to_child(m) }
                    }
                }
                Line::Dir(k) => {
                    acc.add_directory(k);
                },
                Line::File(size, _) => {
                    acc.increase_size(size);
                }
                _ => {}
            }
            acc
        });
        let total_size = dir_tree.get_total_size(0);
        let required_space = total_size - 40000000;

        let mut valid_indexes = Vec::from_iter(0..dir_tree.nodes.len())
            .iter()
            .filter(|idx| dir_tree.get_total_size(**idx) >= required_space)
            .map(|idx| *idx)
            .collect::<Vec<usize>>();
        
        valid_indexes.sort_by(|a, b| dir_tree.get_total_size(*a).partial_cmp(&dir_tree.get_total_size(*b)).unwrap());

        dir_tree.get_total_size(valid_indexes[0])

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
        assert_eq!(a::run(&input), 95437);
        assert_eq!(b::run(&input), 24933642);
    }

}
