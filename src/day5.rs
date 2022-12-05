use std::collections::VecDeque;
use std::fs;

pub fn run1(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let result = part_1(&input);
    println!("result: {}", result);
}

fn part_1(input: &String) -> String {
    let layout = parse_layout(&input);
    let instructions = parse_instructions(&input);
    let instructions = convert_instructions(&instructions);
    process_instructions(&layout, &instructions)
}

/// Converts instructions to machine format.
fn convert_instructions(instructions: &Vec<String>) -> Vec<Vec<usize>> {
    instructions.iter().fold(Vec::new(), |acc, instruction| {
        let mut result: Vec<Vec<usize>> = acc.clone();
        let mut n: Vec<usize> = instruction
            .split(' ')
            .skip(1)
            .step_by(2)
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        n[1] = n[1] - 1;
        n[2] = n[2] - 1;
        result.push(n);
        result
    })
}

/// Executes instructions on the containers.
fn process_instructions(
    containers: &Vec<VecDeque<char>>,
    instructions: &Vec<Vec<usize>>,
) -> String {
    let mut result = containers.clone();

    instructions.iter().for_each(|instruction| {
        let qty = instruction[0];
        let from = instruction[1];
        let to = instruction[2];

        for _ in 0..qty {
            execute_instruction(&mut result, from, to);
        }
    });

    result.iter().fold(String::new(), |acc, container| {
        acc + container[0].to_string().as_str()
    })
}

/// Executes a single instruction.
fn execute_instruction(result: &mut Vec<VecDeque<char>>, from: usize, to: usize) {
    let cargo = result[from].pop_front().unwrap();
    result[to].push_front(cargo);
}

/// Parses instructions from input.
fn parse_instructions(input: &str) -> Vec<String> {
    let instructions: String = input.split("\n\n").skip(1).take(1).collect();
    let result: Vec<&str> = instructions.split('\n').collect();
    result
        .iter()
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Returns an amount of stacks from the last line of the stacks layout.
fn num_stacks(line: &str) -> usize {
    let mut result = 0;
    for s in line.split(' ') {
        if !s.is_empty() {
            result = s.parse::<usize>().unwrap();
        }
    }
    result
}

/// Parses the stacks layout and containers' positions from input.
fn parse_layout(input: &str) -> Vec<VecDeque<char>> {
    let layout: String = input.split("\n\n").take(1).collect();
    let rows: Vec<&str> = layout.split('\n').collect();
    let num_stacks = num_stacks(rows[rows.len() - 1]);
    let rows_without_last_line: Vec<&&str> = rows.iter().take(rows.len() - 1).collect();

    let mut containers: Vec<VecDeque<char>> = Vec::new();
    for _ in 0..num_stacks {
        containers.push(VecDeque::new());
    }

    for stack in rows_without_last_line {
        let mut container: Vec<char> = Vec::new();

        stack
            .chars()
            .skip(1)
            .step_by(4)
            .for_each(|c| container.push(c));

        for i in 0..num_stacks {
            if container[i] != ' ' {
                containers[i].push_back(container[i]);
            }
        }
    }

    containers
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> String {
        String::from(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        )
    }

    #[test]
    fn test_part_1() {
        let input = input();
        assert_eq!(part_1(&input), "CMZ");
    }

    #[test]
    fn test_parse_layout() {
        let input = input();
        let result = parse_layout(&input);
        dbg!(&result);
        assert_eq!(result, vec![vec!['N', 'Z'], vec!['D', 'C', 'M'], vec!['P']]);
    }

    #[test]
    fn test_parse_instructions() {
        let input = input();
        let result = parse_instructions(&input);
        dbg!(&result);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_simplify_instructions() {
        let input = input();
        let instructions = parse_instructions(&input);
        let result = convert_instructions(&instructions);
        dbg!(&result);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_process_instructions() {
        let input = input();
        let layout = parse_layout(&input);
        let instructions = parse_instructions(&input);
        let instructions = convert_instructions(&instructions);
        let result = process_instructions(&layout, &instructions);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_num_stacks() {
        let input = " 1   2   3   4   5   6   7   8   9 ";
        let result = num_stacks(&input);
        assert_eq!(result, 9);
    }
}
