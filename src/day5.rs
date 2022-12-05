use std::fs;

pub fn run1(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let _result = part_1(&input);
    // println!("result: {}", result);
}

fn part_1(input: &String) -> &str {
    input
}

fn simplify_instructions(instructions: &Vec<String>) -> Vec<Vec<usize>> {
    instructions.iter().fold(Vec::new(), |acc, instruction| {
        let mut result: Vec<Vec<usize>> = acc.clone();
        let n: Vec<usize> = instruction
            .split(' ')
            .skip(1)
            .step_by(2)
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        result.push(n);
        result
    })
}

fn process_instructions(containers: &Vec<Vec<char>>, instructions: &Vec<String>) -> String {
    let instructions_simplified = simplify_instructions(instructions);
    let mut result = containers.clone();
    instructions_simplified.iter().for_each(|instruction| {
        let qty = instruction[0];
        let from = instruction[1];
        let to = instruction[2];
        dbg!(instruction);

        for _ in 0..qty {
            execute_insutruction(&mut result, from, to);
        }
    });
    dbg!(&result);
    String::from("")
}

fn execute_insutruction(result: &mut Vec<Vec<char>>, from: usize, to: usize) {
    // take cargo from container
    let cargo = result[from][0].clone();
    result[from][0] = ' ';

    // find the last empty slot
    let n: usize;
    result[to].reverse();
    if let Some(i) = result[to].iter().position(|&c| c == ' ') {
        n = result[to].len() - i - 2;
    } else {
        n = result[to].len();
    }
    result[to].reverse();

    // move cargo
    dbg!(to);
    dbg!(n);
    result[to][n] = cargo;
}

fn parse_instructions(input: &str) -> Vec<String> {
    let instructions: String = input.split("\n\n").skip(1).take(1).collect();
    let result: Vec<&str> = instructions.split('\n').collect();
    result.iter().map(|s| s.to_string()).collect()
}

fn parse_layout(input: &str) -> Vec<Vec<char>> {
    let layout: String = input.split("\n\n").take(1).collect();
    let stacks: Vec<&str> = layout.split('\n').collect();
    let num_stacks = stacks[0].len() / 3;
    let stacks: Vec<&&str> = stacks.iter().take(stacks.len() - 1).collect();

    let mut containers: Vec<Vec<char>> = Vec::new();
    for _ in 0..num_stacks {
        containers.push(Vec::new());
    }

    for stack in stacks {
        let mut container: Vec<char> = Vec::new();

        stack
            .chars()
            .skip(1)
            .step_by(4)
            .for_each(|c| container.push(c));

        for i in 0..num_stacks {
            containers[i].push(container[i]);
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
        assert_eq!(
            result,
            vec![
                vec![' ', 'N', 'Z'],
                vec!['D', 'C', 'M'],
                vec![' ', ' ', 'P']
            ]
        );
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
        let result = simplify_instructions(&instructions);
        dbg!(&result);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_process_instructions() {
        let input = input();
        let layout = parse_layout(&input);
        let instructions = parse_instructions(&input);
        let result = process_instructions(&layout, &instructions);
        assert_eq!(result, "CMZ");
    }
}
