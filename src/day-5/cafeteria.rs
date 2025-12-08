use std::io;
use std::str::FromStr;

#[derive(Debug)]
struct Range {
    first: u64,
    last: u64,
}

impl FromStr for Range {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (first, last) = input.split_once('-').ok_or("Could not parse range")?;

        let first = first.trim().parse::<u64>().map_err(|_| "Bad first")?;
        let last = last.trim().parse::<u64>().map_err(|_| "Bad last")?;

        Ok(Range { first, last })
    }
}

impl Range {
    fn len(&self) -> u64 {
        self.last - self.first + 1
    }

    fn check(&self, number: u64) -> bool {
        number >= self.first && number <= self.last
    }
}

#[derive(Debug)]
struct Database {
    ranges: Vec<Range>,
    ingredients: Vec<u64>,
}

impl FromStr for Database {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (ranges, ingredients) = input
            .split_once("\n\n")
            .or_else(|| input.split_once("\r\n\r\n"))
            .ok_or("Could not parse delimiter")?;

        // parse ranges
        let ranges = ranges
            .lines()
            .map(|line| line.parse::<Range>())
            .collect::<Result<Vec<_>, _>>()?;

        // parse ingredients
        let ingredients = ingredients
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().parse::<u64>().map_err(|_| "Invalid integer"))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Database::new(ranges, ingredients))
    }
}

impl Database {
    fn new(ranges: Vec<Range>, ingredients: Vec<u64>) -> Self {
        // sort ranges by start (and then by end for stability)
        let mut ranges = ranges;
        ranges.sort_by(|a, b| match a.first.cmp(&b.first) {
            std::cmp::Ordering::Equal => a.last.cmp(&b.last),
            other => other,
        });

        // merge overlapping ranges
        let mut merged: Vec<Range> = Vec::new();
        for range in ranges {
            if let Some(last) = merged.last_mut() {
                // overlap if current.first <= last.last
                if range.first <= last.last {
                    // extend the last range if needed
                    if range.last > last.last {
                        last.last = range.last;
                    }
                } else {
                    // no overlap: start a new merged range
                    merged.push(range);
                }
            } else {
                // first range
                merged.push(range);
            }
        }

        Self {
            ranges: merged,
            ingredients,
        }
    }

    fn count_fresh_ingredients(&self) -> usize {
        self.ingredients
            .iter()
            .copied()
            .filter(|&value| {
                // true if any range contains this value
                self.ranges.iter().any(|range| range.check(value))
            })
            .count()
    }

    fn count_all_fresh_ingredients(&self) -> usize {
        self.ranges.iter().map(|range| range.len()).sum::<u64>() as usize
    }
}

fn main() -> io::Result<()> {
    let input = include_str!("input.data").trim();
    let database = Database::from_str(input).unwrap();
    println!(
        "The answer for part one is {}",
        database.count_fresh_ingredients()
    );
    println!(
        "The answer for part two is {}",
        database.count_all_fresh_ingredients()
    );
    Ok(())
}
