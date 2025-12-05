use aoc2025_rs::{Range, read_lines};

fn parse<S: AsRef<str>>(lines: &[S]) -> (Vec<Range>, Vec<u64>) {
    let split = lines
        .iter()
        .position(|l| l.as_ref().is_empty())
        .unwrap_or(lines.len());

    let ranges = lines[..split]
        .iter()
        .map(|r| r.as_ref().parse::<Range>().unwrap())
        .collect();

    let nums = lines[split + 1..]
        .iter()
        .map(|n| n.as_ref().parse::<u64>().unwrap())
        .collect();

    (ranges, nums)
}

fn part1(ranges: &[Range], nums: &[u64]) -> u64 {
    nums.iter()
        .copied()
        .filter(|&n| ranges.iter().any(|r| r.contains_inclusive(n)))
        .count() as u64
}

fn part2(ranges: &[Range]) -> u64 {
    if ranges.is_empty() {
        return 0;
    }

    let mut sorted: Vec<_> = ranges.to_vec();
    sorted.sort_by_key(|r| r.0);

    let mut merged = vec![ranges[0]];

    for r in sorted.iter().skip(1) {
        let last = merged.last_mut().unwrap();

        if last.overlaps(r) {
            last.merge(r);
        } else {
            merged.push(*r);
        }
    }

    merged.iter().map(|r| r.cardinality()).sum()
}

fn main() {
    let lines = read_lines(5);
    let (ranges, nums) = parse(&lines);
    println!("Part 1: {}", part1(&ranges, &nums));
    println!("Part 2: {}", part2(&ranges));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&[Range(3, 5), Range(10, 14), Range(16, 20), Range(12, 18)]),
            14
        );
    }
}
