use std::io;
use std::str::FromStr;

#[derive(Debug)]
struct Bank {
    batteries: Vec<u8>,
}

impl FromStr for Bank {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let batteries = line
            .chars()
            .map(|ch| {
                ch.to_digit(10)
                    .ok_or_else(|| format!("Invalid digit: '{}'", ch))
                    .map(|d| d as u8)
            })
            .collect::<Result<Vec<u8>, _>>()?;

        Ok(Bank { batteries })
    }
}

impl Bank {
    fn max_joltage(&self, k: usize) -> u64 {
        let n = self.batteries.len();
        if k == 0 || k > n {
            return 0;
        }

        let mut result: u64 = 0;
        let mut start = 0;
        for remaining in (0..k).rev() {
            // search range end: we must leave `remaining` digits after the chosen one
            let end = n - remaining;

            // choose max digit in batteries[start..end)
            let mut max_digit = 0u8;
            let mut max_pos = start;

            for i in start..end {
                let d = self.batteries[i];
                if d > max_digit {
                    max_digit = d;
                    max_pos = i;
                }
            }

            // append digit to result
            result = result * 10 + max_digit as u64;

            // next search starts after this position
            start = max_pos + 1;
        }

        result
    }
}

fn process_banks(banks: &Vec<Bank>, batteries_count: usize) -> u64 {
    banks
        .iter()
        .map(|bank| bank.max_joltage(batteries_count))
        .sum()
}

fn process_input(input: &str) -> Vec<Bank> {
    input
        .split_whitespace()
        .map(|data| Bank::from_str(data).unwrap())
        .collect::<Vec<_>>()
}

fn main() -> io::Result<()> {
    let input = include_str!("input.data").trim();

    let banks = process_input(input);
    println!(
        "The answer for part one is : {:?}",
        process_banks(&banks, 2)
    );
    println!(
        "The answer for part two is : {:?}",
        process_banks(&banks, 12)
    );

    Ok(())
}
