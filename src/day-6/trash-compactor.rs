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

fn parse_problems(input: &str) -> Vec<Problem> {
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

fn get_answer_for_part_one(problems: &[Problem]) -> u64 {
    problems.iter().map(Problem::solve).sum()
}

fn main() -> io::Result<()> {
    let input = include_str!("input.data").trim();

    let problems = parse_problems(input);
    println!(
        "The answer for part one is {}",
        get_answer_for_part_one(&problems)
    );

    Ok(())
}
