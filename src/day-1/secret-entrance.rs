use std::io;

#[derive(Debug, Copy, Clone)]
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

fn count_zero_hits_during_move(position: i32, direction: Direction, distance: i32) -> u64 {
    let distance = distance.max(0);
    if distance == 0 {
        return 0;
    }

    // First k (1-based click index) at which the dial hits 0.
    // For Right: new_pos = (pos + k) mod 100 == 0 -> k ≡ -pos ≡ 100 - pos (mod 100)
    // For Left:  new_pos = (pos - k) mod 100 == 0 -> k ≡ pos (mod 100)
    let position = position.rem_euclid(100);
    let first_k = match direction {
        Direction::Right => {
            let k0 = (100 - position) % 100;
            if k0 == 0 { 100 } else { k0 }
        }
        Direction::Left => {
            if position == 0 {
                100
            } else {
                position
            }
        }
    };

    if first_k > distance {
        0
    } else {
        // Every additional 100 clicks we hit 0 again.
        let remaining = distance - first_k;
        (remaining / 100 + 1) as u64
    }
}

fn process_input(input: &str) -> u64 {
    let mut position: i32 = 50; // dial starts at 50
    let mut zero_hits: u64 = 0;

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let (direction, distance) = parse_line(line);
        zero_hits += count_zero_hits_during_move(position, direction, distance);
        position = apply_move(position, direction, distance);
    }

    zero_hits
}

fn main() -> io::Result<()> {
    let input = include_str!("input.data");

    let zero_hits = process_input(input);

    println!("The actual password is {zero_hits}");
    Ok(())
}
