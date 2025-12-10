use std::collections::{HashSet, VecDeque};
use std::io;
use std::str::FromStr;

#[derive(Debug)]
struct Machine {
    indicators: u64,
    buttons: Vec<Vec<usize>>,
    joltage_levels: Vec<usize>,
}

impl Machine {
    fn configure_indicator_lights(&self) -> usize {
        if self.indicators == 0 {
            return 0;
        }

        let button_masks = self
            .buttons
            .iter()
            .map(|wiring| {
                wiring
                    .iter()
                    .fold(0u64, |mask, indicator| mask | (1u64 << indicator))
            })
            .collect::<Vec<_>>();

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
                if next == self.indicators {
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

    // The implementation isn't optimal and very slow with "real" input, but it's easy to read/understand
    // For more effective solution read about
    //     - ILP (Integer Linear Programing)
    //     - Linear Diophantine Systems
    fn configure_joltage_levels(&self) -> usize {
        let levels = self.joltage_levels.len();
        if levels == 0 || self.joltage_levels.iter().all(|&level| level == 0) {
            return 0;
        }

        // state = remaining joltage needed, but we need signed to allow "go below 0" check.
        #[derive(Clone, Eq, PartialEq, Hash)]
        struct State(Vec<i16>);

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        // start from target joltage as the "remaining" vector.
        let start_vec = self
            .joltage_levels
            .iter()
            .map(|&level| level as i16)
            .collect::<Vec<_>>();

        queue.push_back((start_vec, 0usize));

        while let Some((state_vec, pressed)) = queue.pop_front() {
            let state = State(state_vec);

            if !visited.insert(state.clone()) {
                continue; // already processed this vector
            }

            // if all zeros: we've used the minimal number of presses.
            if state.0.iter().all(|&x| x == 0) {
                return pressed;
            }

            // if any level went negative, this path is invalid.
            if state.0.iter().any(|&x| x < 0) {
                continue;
            }

            // try pressing each button once (in backward sense: subtract 1)
            for button in &self.buttons {
                if button.is_empty() {
                    continue;
                }

                let mut next = state.0.clone();
                for &pos in button {
                    next[pos] -= 1;
                }

                queue.push_back((next, pressed + 1));
            }
        }

        // we should never get here.
        panic!("Target joltage configuration is unreachable for this machine");
    }
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // extract light diagram: stuff between '[' and ']'
        let start = input.find('[').expect("missing light start marker");
        let end = input.find(']').expect("missing light end marker");
        let indicators = input[start + 1..end]
            .char_indices()
            .fold(0, |mask, (bit, value)| match value {
                '#' => mask | (1 << bit),
                _ => mask,
            });

        // helper to parse comma-separated numbers
        fn parse_numbers(data: &str) -> Vec<usize> {
            if data.trim().is_empty() {
                return vec![];
            }

            data.split(',')
                .map(|number| number.trim().parse::<usize>().unwrap())
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

        // extract joltage levels
        let start = input.find('{').expect("missing joltage start marker");
        let end = input.find('}').expect("missing joltage end marker");
        let joltage_levels = parse_numbers(&input[start + 1..end]);

        Ok(Machine {
            indicators,
            buttons,
            joltage_levels,
        })
    }
}

fn parse_machines(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| Machine::from_str(line).expect("Invalid machine data"))
        .collect::<Vec<_>>()
}

fn configure_indicator_lights(machines: &[Machine]) -> usize {
    machines
        .iter()
        .map(|machine| machine.configure_indicator_lights())
        .sum()
}

fn configure_joltage_levels(machines: &[Machine]) -> usize {
    machines
        .iter()
        .map(|machine| machine.configure_joltage_levels())
        .sum()
}

fn main() -> io::Result<()> {
    let input = include_str!("input.data").trim();

    let machines = parse_machines(input);
    println!(
        "The answer for part one is {}",
        configure_indicator_lights(&machines)
    );
    println!(
        "The answer for part two is {}",
        configure_joltage_levels(&machines)
    );

    Ok(())
}
