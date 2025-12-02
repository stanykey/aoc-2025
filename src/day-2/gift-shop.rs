use std::io;
use std::str::FromStr;

#[derive(Debug)]
struct Range {
    first: i64,
    last: i64,
}

impl FromStr for Range {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (first, last) = input.split_once('-').ok_or("Could not parse range")?;

        let first = first.trim().parse::<i64>().map_err(|_| "Bad first")?;
        let last = last.trim().parse::<i64>().map_err(|_| "Bad last")?;

        Ok(Range { first, last })
    }
}

fn process_input(input: &str) -> Vec<Range> {
    input
        .split(',')
        .map(|part| Range::from_str(part).expect("Invalid range"))
        .collect::<Vec<_>>()
}

fn check_id(id: i64) -> bool {
    // convert to decimal string
    let s = id.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();

    // must have even number of digits
    if len % 2 != 0 {
        return true;
    }

    // no leading zeros (the puzzle also says inputs don't have them,
    // but this keeps the function self-contained)
    if bytes[0] == b'0' {
        return true;
    }

    let mid = len / 2;
    &bytes[..mid] != &bytes[mid..]
}

fn count_invalid_ids(range: &Range) -> Vec<i64> {
    (range.first..=range.last)
        .filter(|&id| !check_id(id))
        .map(|id| id as i64)
        .collect()
}

fn process_ids(ranges: &[Range]) -> i64 {
    ranges.iter().flat_map(count_invalid_ids).sum()
}

fn main() -> io::Result<()> {
    let input = include_str!("input.data").trim();

    let ranges = process_input(input);
    let result = process_ids(&ranges);
    println!("The result is {}", result);

    Ok(())
}
