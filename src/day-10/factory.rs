use std::collections::{HashSet, VecDeque};
use std::io;
use std::str::FromStr;

#[derive(Debug)]
struct Machine {
    goal: u64,
    buttons: Vec<Vec<u64>>,
    joltage: Vec<u64>,
}

impl Machine {
    fn find_minimal_toggles(&self) -> usize {
        if self.goal == 0 {
            // trivial case
            return 0;
        }

        // build bitmasks for each button: each i32 index in `buttons` is a light index to toggle.
        let mut button_masks = Vec::with_capacity(self.buttons.len());
        for wiring in &self.buttons {
            let mut mask: u64 = 0;
            for &idx in wiring {
                let idx_usize = idx as usize;
                mask |= 1u64 << idx_usize;
            }
            button_masks.push(mask);
        }

        // BFS over light states; state is a `u64` mask.
        let start: u64 = 0;
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(start);
        queue.push_back((start, 0));
        while let Some((state, steps)) = queue.pop_front() {
            // try pressing each button once from this state
            for &mask in &button_masks {
                let next = state ^ mask; // toggle lights
                if next == self.goal {
                    return steps + 1;
                }

                if visited.insert(next) {
                    queue.push_back((next, steps + 1));
                }
            }
        }

        // the target pattern is unreachable with these buttons.
        usize::MAX
    }
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // extract light diagram: stuff between '[' and ']'
        let start = input.find('[').expect("missing light start marker");
        let end = input.find('}').expect("missing light end marker");
        let goal = input[start + 1..end]
            .char_indices()
            .fold(0, |mask, (bit, value)| match value {
                '#' => mask | (1 << bit),
                _ => mask,
            });

        // helper to parse comma-separated numbers
        fn parse_numbers(data: &str) -> Vec<u64> {
            if data.trim().is_empty() {
                return vec![];
            }

            data.split(',')
                .map(|number| number.trim().parse::<u64>().unwrap())
                .collect()
        }

        // extract buttons data
        let start = input.find('(').expect("missing buttons start marker");
        let end = input.rfind(')').expect("missing buttons end marker");
        let buttons = input[start..end + 1]
            .trim()
            .split_whitespace()
            .map(|part| parse_numbers(&part[1..part.len() - 1]))
            .collect::<Vec<_>>();

        // extract joltage data
        let start = input.find('{').expect("missing joltage start marker");
        let end = input.find('}').expect("missing joltage end marker");
        let joltage = parse_numbers(&input[start + 1..end]);

        Ok(Machine {
            goal,
            buttons,
            joltage,
        })
    }
}

fn parse_machines(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| Machine::from_str(line).expect("Invalid machine data"))
        .collect::<Vec<_>>()
}

fn count_buttons_presses(machines: &[Machine]) -> usize {
    machines
        .iter()
        .map(|machine| machine.find_minimal_toggles())
        .sum()
}

fn main() -> io::Result<()> {
    let input = include_str!("input.data").trim();

    let machines = parse_machines(input);
    println!(
        "The answer for part one is {}",
        count_buttons_presses(&machines)
    );

    Ok(())
}
