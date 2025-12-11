use std::collections::{HashMap, HashSet, VecDeque};
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
    use std::collections::{HashMap, HashSet};

    let mut devices = HashMap::new();

    // first pass: parse each line
    for line in input.lines() {
        let dev = Device::from_str(line).unwrap();
        let name = dev.name.clone();
        devices.insert(name, dev);
    }

    // second pass: ensure every `out` exists as a Device (even if empty)
    let mut missing = Vec::new();
    for dev in devices.values() {
        for out in &dev.outs {
            if !devices.contains_key(out) {
                missing.push(out.clone());
            }
        }
    }

    for name in missing {
        devices.entry(name.clone()).or_insert(Device {
            name,
            outs: HashSet::new(),
        });
    }

    devices
}

fn collect_all_paths(devices: &HashMap<String, Device>, from: &str, to: &str) -> Vec<Vec<String>> {
    fn dfs(
        devices: &HashMap<String, Device>,
        current: &str,
        target: &str,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
        all_paths: &mut Vec<Vec<String>>,
    ) {
        visited.insert(current.to_string());
        path.push(current.to_string());

        if current == target {
            // found full path, store a copy
            all_paths.push(path.clone());
        } else if let Some(device) = devices.get(current) {
            for next in &device.outs {
                if !visited.contains(next) {
                    dfs(devices, next, target, visited, path, all_paths);
                }
            }
        }

        // backtrack
        path.pop();
        visited.remove(current);
    }

    let mut all_paths = Vec::new();
    let mut visited = HashSet::new();
    let mut path = Vec::new();

    dfs(devices, from, to, &mut visited, &mut path, &mut all_paths);
    all_paths
}

fn count_paths(devices: &HashMap<String, Device>, from: &str, to: &str) -> usize {
    // it's not optimal, but run in a blink under debug, so it meets my personal requirements
    collect_all_paths(&devices, from, to).len()
}

/// Assumes: graph is a DAG
fn count_paths_with_checkpoints(
    devices: &HashMap<String, Device>,
    from: &str,
    to: &str,
    checkpoints: &[&str],
) -> usize {
    // 1) build name -> index mapping (include nodes that appear only in outs)
    let mut name_to_idx = HashMap::new();
    let mut idx_to_name = Vec::new();

    // first, all device keys
    for name in devices.keys() {
        let idx = idx_to_name.len();
        idx_to_name.push(name.as_str());
        name_to_idx.insert(idx_to_name[idx], idx);
    }

    // then, any out-targets not already present
    for dev in devices.values() {
        for out in &dev.outs {
            if !name_to_idx.contains_key(out.as_str()) {
                let idx = idx_to_name.len();
                idx_to_name.push(out.as_str());
                name_to_idx.insert(idx_to_name[idx], idx);
            }
        }
    }

    let get_idx = |name: &str| -> Option<usize> { name_to_idx.get(name).copied() };
    let from_idx = get_idx(from).unwrap_or(0);
    let to_idx = get_idx(to).unwrap_or(0);

    // 2) build adjacency list and indegree for topo sort
    let outs_count = idx_to_name.len();
    let mut adj = vec![Vec::new(); outs_count];
    let mut indegree = vec![0; outs_count];

    for (name, dev) in devices {
        let u = get_idx(name).expect("device key must map to index");
        for out in &dev.outs {
            if let Some(v) = get_idx(out) {
                adj[u].push(v);
                indegree[v] += 1;
            }
        }
    }

    // 3) topological sort (Kahn)
    let mut topo = Vec::with_capacity(outs_count);
    let mut q = VecDeque::new();
    for v in 0..outs_count {
        if indegree[v] == 0 {
            q.push_back(v);
        }
    }
    while let Some(u) = q.pop_front() {
        topo.push(u);
        for &v in &adj[u] {
            indegree[v] -= 1;
            if indegree[v] == 0 {
                q.push_back(v);
            }
        }
    }
    if topo.len() != outs_count {
        panic!("graph is not a DAG (cycle detected)");
    }

    // 4) map checkpoints -> bits and build per-node checkpoint mask
    let mut checkpoint_index = HashMap::new();
    for (bit, &cp) in checkpoints.iter().enumerate() {
        checkpoint_index.insert(cp, bit);
    }

    let mut checkpoint_mask = vec![0; outs_count]; // up to 5 bits -> u8 is enough
    for (name, &idx) in &name_to_idx {
        if let Some(&bit) = checkpoint_index.get(name) {
            checkpoint_mask[idx] = 1u8 << bit;
        }
    }

    // quick check: if any checkpoint name isn't in the graph, answer is 0
    for &checkpoint in checkpoints {
        if get_idx(checkpoint).is_none() {
            return 0;
        }
    }

    // 5) DP over topo order with bitmask of visited checkpoints.
    // dp[v][mask] = number of paths from `from` to `v` that have visited
    // exactly the set of checkpoints indicated by `mask`.
    let mask_count = 1usize << checkpoints.len();
    let mut dp = vec![vec![0; mask_count]; outs_count];

    let start_mask = checkpoint_mask[from_idx] as usize;
    dp[from_idx][start_mask] = 1;

    for &u in &topo {
        for mask in 0..mask_count {
            let ways = dp[u][mask];
            if ways == 0 {
                continue;
            }
            for &v in &adj[u] {
                let new_mask = mask | checkpoint_mask[v] as usize;
                dp[v][new_mask] += ways;
            }
        }
    }

    if checkpoints.is_empty() {
        // no checkpoints: total paths from `from` to `to` is sum over all masks
        return dp[to_idx].iter().sum();
    }

    let full_mask = (1usize << checkpoints.len()) - 1;
    dp[to_idx][full_mask]
}

fn main() -> io::Result<()> {
    let input = include_str!("input.data").trim();

    let devices = load_devices(input);
    println!(
        "The answer for part one is {}",
        count_paths(&devices, "you", "out")
    );

    let checkpoints = vec!["dac", "fft"];
    println!(
        "The answer for part two is {}",
        count_paths_with_checkpoints(&devices, "svr", "out", &checkpoints)
    );

    Ok(())
}
