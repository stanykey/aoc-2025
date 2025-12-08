use std::io;
use std::str::FromStr;

// directions for 8 neighbors
static DIRECTIONS: &[(isize, isize)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<char>>,
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let grid = input
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();
        Ok(Grid { data: grid })
    }
}

impl Grid {
    fn width(&self) -> usize {
        self.data[0].len()
    }

    fn height(&self) -> usize {
        self.data.len()
    }

    fn validate_position(&self, row: isize, col: isize) -> bool {
        0 <= row && row < self.height() as isize && 0 <= col && col < self.width() as isize
    }
}

fn count_accessible_paper(grid: &Grid) -> usize {
    let mut accessible_count = 0usize;

    for row in 0..grid.height() as isize {
        for col in 0..grid.width() as isize {
            if grid.data[row as usize][col as usize] != '@' {
                continue;
            }

            let mut neighbor_rolls = 0;

            for (row_shift, col_shift) in DIRECTIONS {
                let next_row = row + row_shift;
                let next_col = col + col_shift;

                if grid.validate_position(next_row, next_col) {
                    if grid.data[next_row as usize][next_col as usize] == '@' {
                        neighbor_rolls += 1;
                    }
                }
            }

            if neighbor_rolls < 4 {
                accessible_count += 1;
            }
        }
    }

    accessible_count
}

fn main() -> io::Result<()> {
    let input = include_str!("input.data").trim();

    let grid = Grid::from_str(input).unwrap();
    println!(
        "The answer for part one is {}",
        count_accessible_paper(&grid)
    );

    Ok(())
}
