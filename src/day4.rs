use std::fs;
use std::ops::RangeInclusive;

pub fn run1(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let result = part_1(&input);
    println!("result: {}", result);
}

pub fn run2(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let result = part_2(&input);
    println!("result: {}", result);
}

fn part_1(input: &String) -> u32 {
    input.lines().fold(0, |acc, line| {
        let pair: Vec<&str> = line.split(',').collect();
        let range_one = str_to_range(pair[0]);
        let range_two = str_to_range(pair[1]);
        if ranges_overlap_fully(&range_one, &range_two) {
            acc + 1
        } else {
            acc
        }
    })
}

fn part_2(input: &String) -> u32 {
    input.lines().fold(0, |acc, line| {
        let pair: Vec<&str> = line.split(',').collect();
        let range_one = str_to_range(pair[0]);
        let range_two = str_to_range(pair[1]);
        if overlap(&range_one, &range_two) {
            acc + 1
        } else {
            acc
        }
    })
}

fn ranges_overlap_fully(range_one: &RangeInclusive<u32>, range_two: &RangeInclusive<u32>) -> bool {
    range_one.contains(&range_two.start()) && range_one.contains(&range_two.end()) ||
        range_two.contains(&range_one.start()) && range_two.contains(&range_one.end())
}

fn overlap(range_one: &RangeInclusive<u32>, range_two: &RangeInclusive<u32>) -> bool {
    range_one.contains(&range_two.start()) || range_one.contains(&range_two.end()) ||
        range_two.contains(&range_one.start()) || range_two.contains(&range_one.end())
}

fn str_to_range(s: &str) -> RangeInclusive<u32> {
    let pair: Vec<&str> = s.split('-').collect();
    let start = pair[0].parse::<u32>().unwrap();
    let end = pair[1].parse::<u32>().unwrap();
    start..=end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = String::from("2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8");

        assert_eq!(part_1(&input), 2);
    }

    #[test]
    fn test_str_to_range() {
        let range = str_to_range("2-4");
        assert_eq!(range, 2..=4);
    }

    #[test]
    fn test_ranges_overlap_fully() {
        let range_one = str_to_range("2-4");
        let range_two = str_to_range("4-5");
        assert_eq!(ranges_overlap_fully(&range_one, &range_two), false);

        let range_one = str_to_range("2-4");
        let range_two = str_to_range("3-3");
        assert_eq!(ranges_overlap_fully(&range_one, &range_two), true);
    }

    #[test]
    fn test_overlap() {
        let range_one = str_to_range("2-4");
        let range_two = str_to_range("4-5");
        assert_eq!(overlap(&range_one, &range_two), true);

        let range_one = str_to_range("2-4");
        let range_two = str_to_range("3-3");
        assert_eq!(overlap(&range_one, &range_two), true);

        let range_one = str_to_range("2-4");
        let range_two = str_to_range("5-7");
        assert_eq!(overlap(&range_one, &range_two), false);
    }
}