use aoc2025_rs::parse_input;

#[derive(PartialEq, Debug)]
struct Turn(i32);

impl Turn {
    fn from_line(line: &str) -> Self {
        let (dir, num) = line.split_at(1);
        let num: i32 = num.parse().expect("Failed to parse int in line");
        match dir {
            "L" => Self(-num),
            "R" => Self(num),
            _ => panic!("Failed to parse direction"),
        }
    }
}

fn part1(turns: &[Turn]) -> i32 {
    let mut dial = 50;
    let mut zeroes = 0;

    for turn in turns.iter() {
        dial = (dial + turn.0).rem_euclid(100);
        if dial == 0 {
            zeroes += 1;
        }
    }

    zeroes
}

fn part2(turns: &[Turn]) -> i32 {
    let mut dial = 50;
    let mut zeroes = 0;

    for turn in turns.iter() {
        for _ in 0..turn.0.abs() {
            dial = (dial + turn.0.signum()).rem_euclid(100);
            if dial == 0 {
                zeroes += 1;
            }
        }
    }

    zeroes
}

fn main() {
    let turns: Vec<Turn> = parse_input(1)
        .iter()
        .map(|line| Turn::from_line(line))
        .collect();

    println!("Part 1: {}", part1(&turns));
    println!("Part 2: {}", part2(&turns));
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: [Turn; 10] = [
        Turn(-68),
        Turn(-30),
        Turn(48),
        Turn(-5),
        Turn(60),
        Turn(-55),
        Turn(-1),
        Turn(-99),
        Turn(14),
        Turn(-82),
    ];

    #[test]
    fn calculates_part1() {
        assert_eq!(part1(&FIXTURE), 3);
    }

    #[test]
    fn calculates_part2() {
        assert_eq!(part2(&FIXTURE), 6);
    }

    #[test]
    fn parses_left() {
        assert_eq!(Turn::from_line("L50"), Turn(-50))
    }

    #[test]
    fn parses_right() {
        assert_eq!(Turn::from_line("R20"), Turn(20))
    }

    #[test]
    #[should_panic]
    fn invalid_dir_panic() {
        Turn::from_line("W20");
    }

    #[test]
    #[should_panic]
    fn invalid_num_panic() {
        Turn::from_line("RHI");
    }
}
