use std::io;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Diagram {
    map: Vec<Vec<char>>,
}

impl FromStr for Diagram {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Diagram {
            map: input
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        })
    }
}

impl Diagram {
    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn find_source(&self) -> (usize, usize) {
        let start_col = self.map[0]
            .iter()
            .position(|&symbol| symbol == 'S')
            .unwrap();
        (0, start_col)
    }
}

fn count_tachyon_beam_splits(diagram: &Diagram) -> usize {
    let height = diagram.height();
    let width = diagram.width();
    let (source_row, source_col) = diagram.find_source();

    // active beams per column for current row (as they enter the next row)
    let mut current = vec![false; width];
    current[source_col] = true;

    let mut splits: usize = 0;
    for row in (source_row + 1)..height {
        let mut next = vec![false; width];

        for col in 0..width {
            if !current[col] {
                continue;
            }

            match diagram.map[row][col] {
                '.' | 'S' => {
                    // beam continues straight down
                    next[col] = true;
                }
                '^' => {
                    // beam is stopped, creates new beams to left and right
                    splits += 1;
                    if col > 0 {
                        next[col - 1] = true;
                    }
                    if col + 1 < width {
                        next[col + 1] = true;
                    }
                }
                _ => {
                    // treat any unknown cells like empty space
                    next[col] = true;
                }
            }
        }

        current = next;

        // if no beams remain, we're done
        if !current.iter().any(|&beam| beam) {
            break;
        }
    }

    splits
}

fn count_different_timelines(diagram: &Diagram) -> u64 {
    let height = diagram.height();
    let width = diagram.width();

    let (source_row, source_col) = diagram.find_source();

    // dp[c] = number of timelines currently in column c (just above the next row to be processed)
    let mut dp = vec![0_u64; width];
    dp[source_col] = 1;

    let mut total_timelines: u64 = 0;

    for row in (source_row + 1)..height {
        let mut next = vec![0_u64; width];

        for col in 0..width {
            let ways = dp[col];
            if ways == 0 {
                continue;
            }

            match diagram.map[row][col] {
                '^' => {
                    // left branch
                    if col > 0 {
                        next[col - 1] += ways;
                    } else {
                        // Exits the manifold to the left
                        total_timelines += ways;
                    }

                    // right branch
                    if col + 1 < width {
                        next[col + 1] += ways;
                    } else {
                        // Exits the manifold to the right
                        total_timelines += ways;
                    }
                }

                '.' | 'S' => {
                    // empty space or source: go straight down
                    next[col] += ways;
                }

                _ => {
                    // any other character – treat as empty space
                    next[col] += ways;
                }
            }
        }

        dp = next;
    }

    // any timelines still inside the grid fall out the bottom
    total_timelines + dp.iter().sum::<u64>()
}

fn main() -> io::Result<()> {
    let input = include_str!("input.data").trim();

    let diagram = Diagram::from_str(input).unwrap();
    println!(
        "The answer for part one is {}",
        count_tachyon_beam_splits(&diagram)
    );
    println!(
        "The answer for part two is {}",
        count_different_timelines(&diagram)
    );

    Ok(())
}
