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

fn check_id_by_part_one_rules(id: i64) -> bool {
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

fn check_id_by_part_two_rules(id: i64) -> bool {
    let s = id.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();

    // ids are valid if:
    // - length < 2 (can't be "pattern repeated at least twice")
    // - or they start with '0' (we treat those as not-invalid per the puzzle)
    if len < 2 || bytes[0] == b'0' {
        return true;
    }

    // look for a period m such that:
    // - m divides len
    // - the number is (first m digits) repeated len/m times
    for m in 1..=len / 2 {
        if len % m != 0 {
            continue;
        }

        let pattern = &bytes[..m];
        if bytes.chunks(m).all(|chunk| chunk == pattern) {
            // found a repeated pattern (at least twice because m <= len/2)
            return false; // invalid
        }
    }

    // no repeating pattern → valid
    true
}

fn count_invalid_ids(range: &Range, function: fn(i64) -> bool) -> Vec<i64> {
    (range.first..=range.last)
        .filter(|&id| !function(id)) // still: collect invalid IDs
        .collect()
}

fn process_ids_for_part_one(ranges: &[Range], function: fn(i64) -> bool) -> i64 {
    ranges
        .iter()
        .flat_map(|range| count_invalid_ids(range, function))
        .sum()
}

fn main() -> io::Result<()> {
    let input = include_str!("input.data").trim();

    let ranges = process_input(input);
    println!(
        "The result for part one is {}",
        process_ids_for_part_one(&ranges, check_id_by_part_one_rules)
    );
    println!(
        "The result for part two is {}",
        process_ids_for_part_one(&ranges, check_id_by_part_two_rules)
    );

    Ok(())
}
