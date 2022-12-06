//! Day 6 of Advent of Code 2022

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
/// use advent_of_code_2022::day6;
///
/// day6::run1("/Users/ihar/Projects/advent-of-code-2022/input/day6.txt");
/// ```
pub fn run1(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let result = process_message(&input, 4);
    println!("result: {}", result);
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
/// use advent_of_code_2022::day6;
///
/// day6::run2("/Users/ihar/Projects/advent-of-code-2022/input/day6.txt");
/// ```
pub fn run2(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let result = process_message(&input, 14);
    println!("result: {}", result);
}

/// Looks for n consecutive unique characters in the string.
fn process_message(input: &str, window_size: usize) -> usize {
    let mut result = 0;

    let slice = input.chars().collect::<Vec<char>>();

    for (position, window) in slice.windows(window_size).enumerate() {
        let mut copy = window.to_vec();
        copy.sort();
        copy.dedup();
        if copy.len() == window_size {
            // all unique
            result = position + window_size;
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input_1() -> Vec<(String, usize)> {
        vec![
            (String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5),
            (String::from("nppdvjthqldpwncqszvftbrmjlhg"), 6),
            (String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10),
            (String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11),
        ]
    }

    fn input_2() -> Vec<(String, usize)> {
        vec![
            (String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19),
            (String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23),
            (String::from("nppdvjthqldpwncqszvftbrmjlhg"), 23),
            (String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29),
            (String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26),
        ]
    }

    #[test]
    fn test_part_1() {
        for (input, expected) in input_1() {
            assert_eq!(process_message(&input, 4), expected);
        }
    }

    #[test]
    fn test_part_2() {
        for (input, expected) in input_2() {
            assert_eq!(process_message(&input, 14), expected);
        }
    }
}
