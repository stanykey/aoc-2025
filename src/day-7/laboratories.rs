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

    fn find_start_position(&self) -> (usize, usize) {
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
    let (start_row, start_col) = diagram.find_start_position();

    // active beams per column for current row (as they enter the next row)
    let mut current = vec![false; width];
    current[start_col] = true;

    let mut splits: usize = 0;
    for row in (start_row + 1)..height {
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

fn main() -> io::Result<()> {
    let input = include_str!("input.data").trim();

    let diagram = Diagram::from_str(input).unwrap();
    let splits = count_tachyon_beam_splits(&diagram);
    println!("{}", splits);

    Ok(())
}
