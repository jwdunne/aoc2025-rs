use aoc2025_rs::read_lines;

fn part1<S: AsRef<str>>(lines: &[S]) -> u64 {
    let len = lines.len();

    let pieces: Vec<_> = lines
        .iter()
        .map(|line| line.as_ref().split_whitespace().collect::<Vec<_>>())
        .collect();

    let ops = &pieces[len - 1];

    let nums: Vec<Vec<u64>> = pieces[..len - 1]
        .iter()
        .map(|ns| ns.iter().map(|n| n.parse::<u64>().unwrap()).collect())
        .collect();

    let mut sum = 0;
    for (i, &op) in ops.iter().enumerate() {
        let col = nums.iter().map(|line| line[i]);

        match op {
            "+" => sum += col.sum::<u64>(),
            "*" => sum += col.product::<u64>(),
            _ => panic!("Unknown operator"),
        }
    }

    sum
}

fn part2<S: AsRef<str>>(lines: &[S]) -> u64 {
    let rows = lines.len();
    let n = rows - 1;
    let cols = lines[0].as_ref().len();

    let ops = lines[n].as_ref().split_whitespace().collect::<Vec<_>>();
    let chars = lines[..n]
        .iter()
        .map(|line| line.as_ref().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let transposed: Vec<_> = (0..cols)
        .map(|col| (0..n).map(|row| chars[row][col]).collect::<Vec<_>>())
        .map(|num| num.iter().collect::<String>())
        .collect();

    let grouped: Vec<Vec<u64>> = transposed
        .split(|s| s.trim().is_empty())
        .map(|group| group.iter().map(|s| s.trim().parse().unwrap()).collect())
        .collect();

    ops.iter()
        .zip(grouped.iter())
        .map(|(&op, group)| match op {
            "+" => group.iter().sum::<u64>(),
            "*" => group.iter().product::<u64>(),
            _ => panic!("Unknown operator"),
        })
        .sum()
}

fn main() {
    let lines = read_lines(6);
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod test {
    use super::*;

    const FIXTURE: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[test]
    fn test_part1() {
        let lines = FIXTURE.lines().collect::<Vec<_>>();
        assert_eq!(part1(&lines), 4277556);
    }

    #[test]
    fn test_part2() {
        let lines = FIXTURE.lines().collect::<Vec<_>>();
        assert_eq!(part2(&lines), 3263827);
    }
}
