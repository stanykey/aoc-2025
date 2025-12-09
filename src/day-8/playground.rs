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

        (dx * dx + dy * dy + dz * dz).sqrt() as i64
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

#[derive(Debug, Clone, Copy)]
struct Edge {
    distance: i64,
    first: usize,
    second: usize,
}

fn build_edges(points: &[Point]) -> Vec<Edge> {
    let count = points.len();
    let mut edges = Vec::with_capacity(count * (count.saturating_sub(1)) / 2);

    for i in 0..count {
        for j in (i + 1)..count {
            edges.push(Edge {
                distance: points[i].euclidean_distance(&points[j]),
                first: i,
                second: j,
            });
        }
    }

    edges.sort_unstable_by_key(|edge| edge.distance);
    edges
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

    fn find(&mut self, value: usize) -> usize {
        if self.parent[value] != value {
            let root = self.find(self.parent[value]);
            self.parent[value] = root;
        }
        self.parent[value]
    }

    fn union(&mut self, first: usize, second: usize) {
        let mut first = self.find(first);
        let mut second = self.find(second);
        if first == second {
            return;
        }

        if self.size[first] < self.size[second] {
            std::mem::swap(&mut first, &mut second);
        }

        self.parent[second] = first;
        self.size[first] += self.size[second];
    }
}

fn solution_for_part_one(points: &Vec<Point>, edges: &Vec<Edge>, connection_count: usize) -> usize {
    let mut dsu = DisjointSet::new(points.len());

    for edge in edges.iter().take(connection_count) {
        dsu.union(edge.first, edge.second);
    }

    // find component sizes: compress paths first.
    for i in 0..points.len() {
        let _ = dsu.find(i);
    }

    let mut circuits_sizes = Vec::new();
    for i in 0..points.len() {
        if dsu.parent[i] == i {
            circuits_sizes.push(dsu.size[i]);
        }
    }

    // sort sizes descending and take the three largest.
    circuits_sizes.sort_unstable_by(|a, b| b.cmp(a));
    circuits_sizes[0] * circuits_sizes[1] * circuits_sizes[2]
}

fn solution_for_part_two(points: &Vec<Point>, edges: &Vec<Edge>) -> usize {
    let mut components = points.len();
    let mut dsu = DisjointSet::new(components);

    for edge in edges {
        let first = dsu.find(edge.first);
        let second = dsu.find(edge.second);
        if first != second {
            dsu.union(edge.first, edge.second);
            components -= 1;

            if components == 1 {
                // this is the final needed connection
                return (points[edge.first].x * points[edge.second].x)
                    .try_into()
                    .unwrap_or_else(|_| panic!("overflow"));
            }
        }
    }

    panic!("Graph never became fully connected");
}

fn main() -> io::Result<()> {
    let file_name = "input.data";
    let input = include_str!("input.data").trim();

    let points = load_points(input);
    let edges = build_edges(&points);
    let connection_count = if file_name == "input.data" { 1000 } else { 10 };
    println!(
        "The answer for part one is {}",
        solution_for_part_one(&points, &edges, connection_count)
    );
    println!(
        "The answer for part two is {}",
        solution_for_part_two(&points, &edges)
    );

    Ok(())
}
