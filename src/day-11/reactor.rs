use std::collections::{HashMap, HashSet};
use std::io;
use std::str::FromStr;

#[derive(Debug)]
struct Device {
    name: String,
    outs: HashSet<String>,
}

impl FromStr for Device {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (name, outs) = input.split_once(':').unwrap();
        let name = name.trim().to_string();
        let outs = outs
            .split_whitespace()
            .map(|out| out.trim().to_string())
            .collect::<HashSet<_>>();
        Ok(Device { name, outs })
    }
}

fn load_devices(input: &str) -> HashMap<String, Device> {
    input
        .lines()
        .map(|line| {
            let dev = Device::from_str(line).unwrap();
            (dev.name.clone(), dev)
        })
        .collect::<HashMap<_, _>>()
}

fn count_all_possible_paths(devices: &HashMap<String, Device>, from: &str, to: &str) -> usize {
    // DFS: count all simple paths
    fn dfs<'a>(
        devices: &'a HashMap<String, Device>,
        current: &'a str,
        target: &'a str,
        visited: &mut HashSet<&'a str>,
    ) -> usize {
        if current == target {
            return 1;
        }

        visited.insert(current);

        let device = devices.get(current).unwrap();
        let mut total = 0usize;

        for next in &device.outs {
            let next_str: &str = next.as_str();

            if !visited.contains(next_str) {
                total += dfs(devices, next_str, target, visited);
            }
        }

        visited.remove(current);
        total
    }

    let mut visited = HashSet::new();
    dfs(devices, from, to, &mut visited)
}

fn main() -> io::Result<()> {
    let input = include_str!("input.data").trim();

    let devices = load_devices(input);
    println!(
        "The answer for part one is {}",
        count_all_possible_paths(&devices, "you", "out")
    );

    Ok(())
}
