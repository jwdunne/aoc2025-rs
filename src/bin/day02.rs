use aoc2025_rs::Range;
use std::fs;

fn is_double_repeat(n: u64) -> bool {
    let digits = n.ilog10() + 1;

    if !digits.is_multiple_of(2) {
        return false;
    }

    let d = digits / 2;
    let divisor = 10_u64.pow(d) + 1;

    n.is_multiple_of(divisor)
}

fn is_repeating(n: u64) -> bool {
    let digits = n.ilog10() + 1;

    for d in 1..=digits / 2 {
        if !digits.is_multiple_of(d) {
            continue;
        }

        let base = 10_u64.pow(d);
        let k = n / 10_u64.pow(digits - d);
        let multiplier = (10_u64.pow(digits) - 1) / (base - 1);

        if k * multiplier == n {
            return true;
        }
    }

    false
}

fn part1(ranges: &[Range]) -> u64 {
    ranges.iter().map(|r| r.sum_repeats(is_double_repeat)).sum()
}

fn part2(ranges: &[Range]) -> u64 {
    ranges.iter().map(|r| r.sum_repeats(is_repeating)).sum()
}

fn main() {
    let input = fs::read_to_string("input/day02.txt").expect("Failed to read input/day02.txt");
    let ranges: Vec<_> = input.split(',').map(|s| s.parse().unwrap()).collect();
    println!("Part 1: {}", part1(&ranges));
    println!("Part 2: {}", part2(&ranges));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_double_repeat_true() {
        assert!(is_double_repeat(11));
        assert!(is_double_repeat(1111));
        assert!(is_double_repeat(2222));
        assert!(is_double_repeat(1010));
        assert!(is_double_repeat(446446));
    }

    #[test]
    fn test_is_double_repeat_false() {
        assert!(!is_double_repeat(1));
        assert!(!is_double_repeat(111));
        assert!(!is_double_repeat(11111));
        assert!(!is_double_repeat(12321));
        assert!(!is_double_repeat(123321));
    }

    #[test]
    fn test_sum_double_repeats() {
        assert_eq!(Range(11, 22).sum_repeats(is_double_repeat), 11 + 22);
        assert_eq!(Range(95, 115).sum_repeats(is_double_repeat), 99);
        assert_eq!(Range(998, 1012).sum_repeats(is_double_repeat), 1010);
    }

    #[test]
    fn test_sum_any_repeats() {
        assert_eq!(Range(11, 22).sum_repeats(is_repeating), 11 + 22);
        assert_eq!(Range(95, 115).sum_repeats(is_repeating), 99 + 111);
        assert_eq!(Range(998, 1012).sum_repeats(is_repeating), 999 + 1010);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&[
                Range(11, 22),
                Range(95, 115),
                Range(998, 1012),
                Range(1188511880, 1188511890),
                Range(222220, 222224),
                Range(1698522, 1698528),
                Range(446443, 446449),
                Range(38593856, 38593862)
            ]),
            1227775554
        );
    }
}
