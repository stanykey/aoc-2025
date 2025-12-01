use std::io;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn parse_line(line: &str) -> (Direction, i32) {
    let line = line.trim();
    let (direction, distance) = line.split_at(1);

    let distance: i32 = distance.parse().expect("Invalid distance");

    let direction = match direction {
        "R" => Direction::Right,
        "L" => Direction::Left,
        _ => panic!("Invalid direction: {}", direction),
    };

    (direction, distance)
}

fn apply_move(position: i32, direction: Direction, distance: i32) -> i32 {
    match direction {
        Direction::Right => (position + distance).rem_euclid(100),
        Direction::Left => (position - distance).rem_euclid(100),
    }
}

fn process_input(input: &str) -> u64 {
    let mut position = 50;
    let mut zero_hits: u64 = 0;

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let (dir, dist) = parse_line(line);

        position = apply_move(position, dir, dist);

        if position == 0 {
            zero_hits += 1;
        }
    }

    zero_hits
}

fn main() -> io::Result<()> {
    let input = include_str!("input.data");

    let zero_hits = process_input(input);

    println!("The actual password is {zero_hits}");

    Ok(())
}
