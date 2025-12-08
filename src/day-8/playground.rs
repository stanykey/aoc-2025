use std::io;

#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn euclidean_distance(&self, other: &Point) -> i64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;

        ((dx * dx + dy * dy + dz * dz).sqrt()) as i64
    }
}

fn load_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let (x, rest) = line.split_once(",").unwrap();
            let (y, z) = rest.split_once(",").unwrap();
            Point {
                x: x.parse().expect(&format!("Cannot parse {} to i64", x)),
                y: y.parse().expect(&format!("Cannot parse {} to i64", y)),
                z: z.parse().expect(&format!("Cannot parse {} to i64", z)),
            }
        })
        .collect::<Vec<_>>()
}

struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return;
        }

        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
    }
}

fn solution_for_part_one(points: &Vec<Point>) -> usize {
    let points_count = points.len();

    // build all unordered pairs (i < j) with their squared distance.
    let mut edges = Vec::new();
    edges.reserve(points_count * (points_count.saturating_sub(1)) / 2);
    for i in 0..points_count {
        for j in (i + 1)..points_count {
            let distance = points[i].euclidean_distance(&points[j]);
            edges.push((distance, i, j));
        }
    }
    edges.sort_unstable_by_key(|&(distance, _, _)| distance);

    let mut dsu = DisjointSet::new(points_count);

    for &(_, a, b) in edges.iter().take(1000) {
        dsu.union(a, b);
    }

    // find component sizes: compress paths first.
    for i in 0..points_count {
        let _ = dsu.find(i);
    }

    let mut circuits_sizes = Vec::new();
    for i in 0..points_count {
        if dsu.parent[i] == i {
            circuits_sizes.push(dsu.size[i]);
        }
    }

    // sort sizes descending and take the three largest.
    circuits_sizes.sort_unstable_by(|a, b| b.cmp(a));
    circuits_sizes[0] * circuits_sizes[1] * circuits_sizes[2]
}

fn main() -> io::Result<()> {
    let input = include_str!("input.data").trim();

    let points = load_points(input);
    println!(
        "The answer for part one is {}",
        solution_for_part_one(&points)
    );

    Ok(())
}
