use std::io;

#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
}

fn load_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            Point {
                x: x.parse().expect(&format!("Cannot parse {} to i64", x)),
                y: y.parse().expect(&format!("Cannot parse {} to i64", y)),
            }
        })
        .collect::<Vec<_>>()
}

fn find_largest_rectangle(points: &[Point]) -> usize {
    points
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| {
            points.iter().skip(i + 1).map(move |p2| {
                let dx = (p1.x - p2.x).abs() + 1;
                let dy = (p1.y - p2.y).abs() + 1;
                dx * dy // i64 tile count
            })
        })
        .max()
        .unwrap_or(0) as usize
}

fn main() -> io::Result<()> {
    let input = include_str!("input.data").trim();

    let points = load_points(input);
    println!(
        "The answer for part one is {}",
        find_largest_rectangle(&points)
    );
    Ok(())
}
