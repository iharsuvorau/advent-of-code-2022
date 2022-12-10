//! Day 7 of Advent of Code 2022

use crate::day7::Command::ChangeDir;
use std::fmt::Display;
use std::fs;

/// Prints the result of part 1.
///
/// # Arguments
///
/// * `path` - Path to the input file.
///
/// # Examples
///
/// ```
/// use advent_of_code_2022::day7;
///
/// day7::run1("/Users/ihar/Projects/advent-of-code-2022/input/day7.txt");
/// ```
pub fn run1(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let root = parse_input(&input);
    let mut dir_sizes = vec![];
    root.get_dir_sizes(&mut dir_sizes);
    let result = dir_sizes
        .iter()
        .filter(|size| **size < 100000)
        .sum::<usize>();
    println!("Day 7, part 1: {}", result);
}

/// Prints the result of part 2.
///
/// # Arguments
///
/// * `path` - Path to the input file.
///
/// # Examples
///
/// ```
/// use advent_of_code_2022::day7;
///
/// day7::run2("/Users/ihar/Projects/advent-of-code-2022/input/day7.txt");
/// ```
pub fn run2(path: &str) {
    let total_size = 70000000usize;
    let least_size = 30000000usize;

    let input = fs::read_to_string(path).unwrap();

    let root = parse_input(&input);

    println!("Root size: {}", root.size());
    println!("Free space: {}", total_size - root.size());
    let target_size = least_size - (total_size - root.size());
    println!("Need to free up: {}", target_size);

    let mut dir_sizes = vec![];
    root.get_dir_sizes(&mut dir_sizes);

    dir_sizes.sort();

    let proper_dir = dir_sizes.iter().filter(|size| **size >= target_size).min();

    println!("Day 7, part 2: {}", proper_dir.unwrap());
}

#[derive(Debug, PartialEq)]
enum FileKind {
    Dir,
    File,
}

impl Display for FileKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileKind::Dir => write!(f, "dir"),
            FileKind::File => write!(f, "file"),
        }
    }
}

#[derive(Debug)]
struct File {
    kind: FileKind,
    name: String,
    size: usize,
    children: Vec<File>,
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl File {
    fn add_child(&mut self, child: File) {
        if !self.children.contains(&child) {
            self.children.push(child);
        }
    }

    fn contains(&self, name: &str) -> bool {
        self.children.iter().any(|child| child.name == name)
    }

    fn get_by_path(&self, path: Vec<String>) -> Option<&File> {
        if path.is_empty() {
            return None;
        }
        if self.name == path[0] {
            if path.len() == 1 {
                return Some(self);
            }
            for child in &self.children {
                if let Some(file) = child.get_by_path(path[1..].to_vec()) {
                    return Some(file);
                }
            }
        } else {
            for child in &self.children {
                if let Some(file) = child.get_by_path(path.clone()) {
                    return Some(file);
                }
            }
        }
        None
    }

    fn add_file_by_path(&mut self, file: File, path: Vec<String>) {
        // println!("Adding file {:?} to path {:?}", file, path);

        if path.len() == 1 {
            if self.name == path[0] {
                self.add_child(file);
            } else if self.contains(&path[0]) {
                let child = self
                    .children
                    .iter_mut()
                    .find(|child| child.name == path[0])
                    .unwrap();
                child.add_child(file);
            } else {
                self.pretty_print(0);
                panic!("Path does not match. Path: {:?}", path);
            }
        } else if path.len() == 2 {
            if self.name == path[0] && self.contains(&path[1]) {
                let child = self
                    .children
                    .iter_mut()
                    .find(|child| child.name == path[1])
                    .unwrap();
                child.add_child(file);
            } else if self.contains(&path[0]) {
                let child = self
                    .children
                    .iter_mut()
                    .find(|child| child.name == path[0])
                    .unwrap();
                child.add_file_by_path(file, path[1..].to_vec());
            } else {
                self.pretty_print(0);
                panic!("Path does not match. Path: {:?}", path);
            }
        } else if self.name == path[0] && self.contains(&path[1]) {
            let child = self
                .children
                .iter_mut()
                .find(|child| child.name == path[1])
                .unwrap();
            child.add_file_by_path(file, path[2..].to_vec());
        } else if self.contains(&path[0]) {
            let child = self
                .children
                .iter_mut()
                .find(|child| child.name == path[0])
                .unwrap();
            child.add_file_by_path(file, path[1..].to_vec());
        } else {
            self.pretty_print(0);
            panic!("Path does not match. Path: {:?}", path);
        }
    }

    fn size(&self) -> usize {
        if self.kind == FileKind::File {
            self.size
        } else {
            self.children.iter().map(|child| child.size()).sum()
        }
    }

    fn get_dir_sizes(&self, dirs: &mut Vec<usize>) {
        if self.kind == FileKind::Dir && self.name != "/" {
            dirs.push(self.size());
        }
        for dir in &self.children {
            dir.get_dir_sizes(dirs);
        }
    }

    fn pretty_print(&self, indent: usize) {
        println!("{}- {} ({})", " ".repeat(indent), self.name, self.kind);
        for child in &self.children {
            child.pretty_print(indent + 2);
        }
    }
}

#[derive(Debug)]
enum Command {
    List,
    ChangeDir(String),
}

fn parse_line(line: &str) -> (Option<Command>, Option<File>) {
    if line.starts_with('$') {
        let command = line[1..].trim();
        if command == "ls" {
            (Some(Command::List), None)
        } else if command.starts_with("cd") {
            let dir = command[2..].trim();
            (Some(ChangeDir(dir.to_string())), None)
        } else {
            panic!("Unknown command: {}", command);
        }
    } else if line.starts_with("dir") {
        let name = line.strip_prefix("dir").unwrap().trim().to_string();
        (
            None,
            Some(File {
                kind: FileKind::Dir,
                name,
                size: 0,
                children: Vec::new(),
            }),
        )
    } else {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let size = parts[0].parse::<usize>().unwrap();
        let name = parts[1].to_string();
        (
            None,
            Some(File {
                kind: FileKind::File,
                name,
                size,
                children: Vec::new(),
            }),
        )
    }
}

fn parse_input(input: &str) -> File {
    let mut root = File {
        kind: FileKind::Dir,
        name: "/".to_string(),
        size: 0,
        children: Vec::new(),
    };

    let mut current_command = ChangeDir("/".to_string());
    let mut path = vec![root.name.clone()];

    for line in input.lines() {
        let (command, file) = parse_line(line);

        if let Some(command) = command {
            current_command = command;
        }

        // println!("Command: {:?}, file: {:?}", current_command, file);

        match &current_command {
            ChangeDir(dir) => {
                if dir == ".." {
                    path.pop();
                } else if dir == "/" {
                    path = vec![dir.to_string()];
                } else {
                    let new_dir = File {
                        kind: FileKind::Dir,
                        name: dir.to_string(),
                        size: 0,
                        children: Vec::new(),
                    };
                    root.add_file_by_path(new_dir, path.clone());
                    path.push(dir.to_string());
                }
            }
            Command::List => {
                if let Some(file) = file {
                    root.add_file_by_path(file, path.clone());
                }
            }
        }

        // println!("Path: {:?}", path);
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> String {
        String::from(
            "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
        )
    }

    #[test]
    fn test_parse_input() {
        let root = parse_input(&input());
        dbg!("{:?}", root);
    }

    #[test]
    fn test_size() {
        let root = parse_input(&input());
        let path = vec!["/".to_string(), "a".to_string(), "e".to_string()];
        let dir = root.get_by_path(path).unwrap();
        assert_eq!(dir.size(), 584);

        let path = vec!["/".to_string(), "a".to_string()];
        let dir = root.get_by_path(path).unwrap();
        assert_eq!(dir.size(), 94853);

        let path = vec!["/".to_string(), "d".to_string()];
        let dir = root.get_by_path(path).unwrap();
        assert_eq!(dir.size(), 24933642);

        let path = vec!["/".to_string()];
        let dir = root.get_by_path(path).unwrap();
        assert_eq!(dir.size(), 48381165);
    }

    #[test]
    fn test_dir_size() {
        let root = parse_input(&input());
        let mut dir_sizes = vec![];
        root.get_dir_sizes(&mut dir_sizes);
        assert_eq!(
            dir_sizes
                .iter()
                .filter(|size| **size < 100000)
                .sum::<usize>(),
            95437
        );
    }

    #[test]
    fn test_pretty_print() {
        let root = parse_input(&input());
        root.pretty_print(0);
    }

    #[test]
    fn test_run1() {
        run1("/Users/ihar/Projects/advent-of-code-2022/input/day7.txt");
    }

    #[test]
    fn test_run2() {
        run2("/Users/ihar/Projects/advent-of-code-2022/input/day7.txt");
    }
}
