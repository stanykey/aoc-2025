use std::collections::VecDeque;
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
        let grid = input.lines().map(|line| line.chars().collect()).collect();
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

fn count_removable_paper(grid: &Grid) -> usize {
    let grid_height = grid.height();
    let grid_width = grid.width();

    // present[r][c] == true iff there is still a roll there (not yet removed)
    let mut present = vec![vec![false; grid_width]; grid_height];
    // degree[r][c] = number of neighboring rolls
    let mut degree = vec![vec![0u8; grid_width]; grid_height];
    for row in 0..grid_height {
        for col in 0..grid_width {
            if grid.data[row][col] == '@' {
                present[row][col] = true;

                let mut neighbors = 0u8;
                for (row_shift, col_shift) in DIRECTIONS {
                    let next_row = row as isize + row_shift;
                    let next_col = col as isize + col_shift;

                    if grid.validate_position(next_row, next_col) {
                        if grid.data[next_row as usize][next_col as usize] == '@' {
                            neighbors += 1;
                        }
                    }
                }
                degree[row][col] = neighbors;
            }
        }
    }

    let mut queue = VecDeque::new();

    // initially, any roll with degree < 4 is removable
    for row in 0..grid_height {
        for col in 0..grid_width {
            if present[row][col] && degree[row][col] < 4 {
                present[row][col] = false;
                queue.push_back((row, col));
            }
        }
    }

    let mut removed = 0usize;
    while let Some((row, col)) = queue.pop_front() {
        removed += 1;

        // removing this roll reduces degree of its neighbors
        for (row_shift, col_shift) in DIRECTIONS {
            let next_row = row as isize + row_shift;
            let next_col = col as isize + col_shift;

            if !grid.validate_position(next_row, next_col) {
                continue;
            }

            let next_row = next_row as usize;
            let next_col = next_col as usize;
            if !present[next_row][next_col] {
                continue;
            }

            if degree[next_row][next_col] > 0 {
                degree[next_row][next_col] -= 1;
            }

            if degree[next_row][next_col] < 4 {
                present[next_row][next_col] = false;
                queue.push_back((next_row, next_col));
            }
        }
    }

    removed
}

fn main() -> io::Result<()> {
    let input = include_str!("input.data").trim();

    let grid = Grid::from_str(input).unwrap();
    println!(
        "The answer for part one is {}",
        count_accessible_paper(&grid)
    );
    println!(
        "The answer for part two is {}",
        count_removable_paper(&grid)
    );

    Ok(())
}
