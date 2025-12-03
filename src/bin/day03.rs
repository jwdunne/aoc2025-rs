use aoc2025_rs::parse_input;

fn max_n_digit_sum(line: &str, n: usize) -> u64 {
    let mut stack = Vec::with_capacity(line.len());

    for (i, digit) in line.char_indices() {
        while let Some(&top) = stack.last() {
            let remaining = line.len() - i - 1;
            if digit <= top || remaining + stack.len() < n {
                break;
            }
            stack.pop();
        }

        stack.push(digit);
    }

    stack.truncate(n);
    stack.iter().collect::<String>().parse().unwrap()
}

fn part1(lines: &[String]) -> u64 {
    lines.iter().map(|line| max_n_digit_sum(line, 2)).sum()
}

fn part2(lines: &[String]) -> u64 {
    lines.iter().map(|line| max_n_digit_sum(line, 12)).sum()
}

fn main() {
    let lines = parse_input(3);
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&[
                "987654321111111".to_string(),
                "811111111111119".to_string(),
                "234234234234278".to_string(),
                "818181911112111".to_string()
            ]),
            357
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&[
                "987654321111111".to_string(),
                "811111111111119".to_string(),
                "234234234234278".to_string(),
                "818181911112111".to_string()
            ]),
            3121910778619
        )
    }

    #[test]
    fn test_max_n_digit_sum_2() {
        assert_eq!(max_n_digit_sum("987654321111111", 2), 98);
        assert_eq!(max_n_digit_sum("811111111111119", 2), 89);
        assert_eq!(max_n_digit_sum("234234234234278", 2), 78);
        assert_eq!(max_n_digit_sum("818181911112111", 2), 92);
    }

    #[test]
    fn test_max_n_digit_sum_12() {
        assert_eq!(max_n_digit_sum("987654321111111", 12), 987654321111);
        assert_eq!(max_n_digit_sum("811111111111119", 12), 811111111119);
        assert_eq!(max_n_digit_sum("234234234234278", 12), 434234234278);
        assert_eq!(max_n_digit_sum("818181911112111", 12), 888911112111);
    }
}
