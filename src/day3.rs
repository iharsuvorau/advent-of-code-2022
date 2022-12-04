use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run1() {
    parse_file("/Users/ihar/Projects/advent-of-code-2022/input/day3_test.txt");
}

pub fn run2() {
    parse_file_2("/Users/ihar/Projects/advent-of-code-2022/input/day3.txt");
}

fn determine_priority(a: char) -> i32 {
    1 + "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars()
        .position(|x| x == a)
        .unwrap() as i32
}

fn parse_file(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let result: Vec<i32> = reader.lines().map(|line| {
        let line = line.unwrap();
        let mid = line.len() / 2;
        let a = line.chars().take(mid).collect::<String>();
        let b = line.chars().skip(mid).collect::<String>();
        let same_chars = find_same_chars(&a, &b);
        println!("chars {:?}", same_chars);

        let mut sum = 0;
        same_chars.iter().for_each(|c| {
            sum += determine_priority(*c);
        });
        sum
    }).collect();

    let sum: i32 = result.iter().sum();

    println!("result: {:?}", result);
    println!("sum: {}", sum);
}

fn parse_file_2(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut v: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        v.push(line);
        if v.len() == 3 {
            groups.push(v.clone());
            v.clear();
        }
    }

    let result: i32 = groups.iter().map(|group| {
        let badge = find_same_char_in_group(&group);
        determine_priority(badge)
    }).sum();

    println!("result: {}", result);
}

fn find_same_char_in_group(v: &Vec<String>) -> char {
    for ac in v[0].chars() {
        for bc in v[1].chars() {
            if ac == bc {
                for cc in v[2].chars() {
                    if ac == cc {
                        return ac;
                    }
                }
            }
        }
    }
    ' '
}

fn find_same_chars(a: &str, b: &str) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    for ac in a.chars() {
        for bc in b.chars() {
            if ac == bc {
                result.push(ac);
            }
        }
    }
    result.dedup();
    result
}