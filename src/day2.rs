use std::fs::File;
use std::io::{BufRead, BufReader};

enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Win,
    Lose,
    Draw,
}

fn parse_rounds(filename: &str) -> Vec<(Shape, Shape)> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let result = reader.lines().map(|line| {
        let line = line.unwrap();
        let mut shapes = line.split_whitespace();
        let shape1 = shapes.next().unwrap();
        let shape2 = shapes.next().unwrap();
        let shape1 = match shape1 {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Unknown shape: {}", shape1),
        };
        let shape2 = match shape2 {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => panic!("Unknown shape: {}", shape2),
        };
        (shape1, shape2)
    }).collect();
    result
}

fn score_round(round: &(Shape, Shape)) -> u32 {
    let mut score = 0;

    match round {
        (_, Shape::Rock) => score += 1,
        (_, Shape::Paper) => score += 2,
        (_, Shape::Scissors) => score += 3,
    }

    match process_round(round) {
        Result::Win => score += 6,
        Result::Draw => score += 3,
        Result::Lose => score += 0,
    }

    score
}

fn process_round(round: &(Shape, Shape)) -> Result {
    match round {
        (Shape::Rock, Shape::Rock) => Result::Draw,
        (Shape::Rock, Shape::Paper) => Result::Win,
        (Shape::Rock, Shape::Scissors) => Result::Lose,
        (Shape::Paper, Shape::Rock) => Result::Lose,
        (Shape::Paper, Shape::Paper) => Result::Draw,
        (Shape::Paper, Shape::Scissors) => Result::Win,
        (Shape::Scissors, Shape::Rock) => Result::Win,
        (Shape::Scissors, Shape::Paper) => Result::Lose,
        (Shape::Scissors, Shape::Scissors) => Result::Draw,
    }
}

fn parse_rounds_part_2(filename: &str) -> Vec<(Shape, Shape)> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let result = reader.lines().map(|line| {
        let line = line.unwrap();
        let mut shapes = line.split_whitespace();
        let shape1 = shapes.next().unwrap();
        let result = shapes.next().unwrap();
        let shape1 = match shape1 {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Unknown shape: {}", shape1),
        };
        let result = match result {
            "X" => Result::Lose,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            _ => panic!("Unknown shape: {}", result),
        };
        let pair = convert_result_to_shape(&(shape1, result));
        pair
    }).collect();
    result
}

fn convert_result_to_shape(round: &(Shape, Result)) -> (Shape, Shape) {
    match round {
        (Shape::Rock, Result::Lose) => (Shape::Rock, Shape::Scissors),
        (Shape::Rock, Result::Draw) => (Shape::Rock, Shape::Rock),
        (Shape::Rock, Result::Win) => (Shape::Rock, Shape::Paper),
        (Shape::Paper, Result::Lose) => (Shape::Paper, Shape::Rock),
        (Shape::Paper, Result::Draw) => (Shape::Paper, Shape::Paper),
        (Shape::Paper, Result::Win) => (Shape::Paper, Shape::Scissors),
        (Shape::Scissors, Result::Lose) => (Shape::Scissors, Shape::Paper),
        (Shape::Scissors, Result::Draw) => (Shape::Scissors, Shape::Scissors),
        (Shape::Scissors, Result::Win) => (Shape::Scissors, Shape::Rock),
    }
}

pub fn run() {
    let rounds = parse_rounds("/Users/ihar/Projects/advent-of-code-2022/input/day2.txt");
    let score = rounds.iter()
        .map(|x| score_round(x))
        .sum::<u32>();
    println!("score: {}", score);
}

pub fn run2() {
    let rounds = parse_rounds_part_2("/Users/ihar/Projects/advent-of-code-2022/input/day2.txt");
    let score = rounds.iter()
        .map(|x| score_round(&x))
        .sum::<u32>();
    println!("score: {}", score);
}