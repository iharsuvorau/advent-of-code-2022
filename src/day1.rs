use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_file(filename: &str) -> Vec<i32> {
    let mut v = Vec::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            v.push(sum);
            sum = 0;
            continue;
        }
        let num = line.parse::<i32>().unwrap();
        sum += num;
    }
    v
}

pub fn run() {
    let mut result = parse_file("/Users/ihar/Projects/advent-of-code-2022/input/day1.txt");
    println!("max: {}", result.iter().max().unwrap().clone());

    result.sort();
    result.reverse();

    let sum_of_first_three = result[0] + result[1] + result[2];
    println!("sum of first three: {}", sum_of_first_three);
}
