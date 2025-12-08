use std::io;

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn apply(&self, left: u64, right: u64) -> u64 {
        match self {
            Operation::Add => left + right,
            Operation::Multiply => left * right,
        }
    }
}

#[derive(Debug, Clone)]
struct Problem {
    numbers: Vec<u64>,
    operation: Operation,
}

impl Problem {
    fn solve(&self) -> u64 {
        self.numbers.iter().copied().fold(
            match self.operation {
                Operation::Add => 0,
                Operation::Multiply => 1,
            },
            |accumulator, value| self.operation.apply(accumulator, value),
        )
    }
}

fn parse_problems_for_part_one(input: &str) -> Vec<Problem> {
    let mut lines = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    // parse operations
    let operations = lines
        .pop()
        .unwrap()
        .split_whitespace()
        .map(|token| match token {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            other => panic!("Invalid operator: {}", other),
        })
        .collect::<Vec<_>>();

    // prepare problems array
    let mut problems = vec![
        Problem {
            numbers: Vec::new(),
            operation: Operation::Add, // temporary, replaced later
        };
        operations.len() // number of columns = number of operations
    ];

    // fill numbers column-wise
    for line in lines {
        let numbers = line
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        for (idx, number) in numbers.into_iter().enumerate() {
            problems[idx].numbers.push(number);
        }
    }

    // assign operations per column
    for (problem, operation) in problems.iter_mut().zip(operations) {
        problem.operation = operation;
    }

    problems
}

fn parse_problems_for_part_two(input: &str) -> Vec<Problem> {
    // keep lines as-is (no trim per line) to preserve horizontal spacing.
    let lines = input.lines().collect::<Vec<_>>();
    let grid_height = lines.len() - 1;
    let grid_width = lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    // build rectangular grid [row][col] of chars, padding right with spaces.
    let mut grid = Vec::with_capacity(grid_height);
    for line in &lines[..grid_height] {
        let mut row = line.chars().collect::<Vec<_>>();
        if row.len() < grid_width {
            row.resize(grid_width, ' ');
        }
        grid.push(row);
    }

    // get segments info
    let operations_line = lines.last().expect("Input is empty");
    let mut segments_positions = operations_line
        .chars()
        .enumerate()
        .filter_map(|(idx, token)| match token {
            '+' | '*' => Some(idx),
            _ => None,
        })
        .collect::<Vec<_>>();
    segments_positions.push(grid_width + 1); // sentinel at the end to make windows cover [start, end)

    // parse numbers
    let mut problems = Vec::new();

    let height = grid.len();
    let operations_line = operations_line.chars().collect::<Vec<_>>();
    for window in segments_positions.windows(2) {
        let start = window[0];
        let end = window[1] - 1;

        let operation = match operations_line[start] {
            '+' => Operation::Add,
            '*' => Operation::Multiply,
            _ => unreachable!("segment start must be an operator"),
        };

        let mut numbers = Vec::new();
        for x in start..end {
            let mut digits = String::new();
            for y in 0..height {
                let ch = grid[y][x];
                if ch.is_ascii_digit() {
                    digits.push(ch);
                }
            }

            numbers.push(digits.parse::<u64>().unwrap());
        }

        problems.push(Problem { numbers, operation });
    }

    problems
}

fn get_answer(problems: Vec<Problem>) -> u64 {
    problems.iter().map(Problem::solve).sum()
}

fn main() -> io::Result<()> {
    let input = include_str!("input.data").trim();

    println!(
        "The answer for part one is {}",
        get_answer(parse_problems_for_part_one(input))
    );
    println!(
        "The answer for part two is {}",
        get_answer(parse_problems_for_part_two(input))
    );

    Ok(())
}
