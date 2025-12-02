use aoc2025_rs::parse_input;

#[derive(PartialEq, Debug, Clone)]
enum Dir {
    Left(i16),
    Right(i16),
}

impl Dir {
    fn to_i16(&self) -> i16 {
        match self {
            Self::Left(n) => -1 * *n,
            Self::Right(n) => *n,
        }
    }

    fn numeric_dir(&self) -> i16 {
        self.to_i16() / self.to_i16().abs()
    }

    fn from_line(line: &str) -> Self {
        let (dir, num) = line.split_at(1);
        let num = num.parse().expect("Failed to parse u8 in line");
        match dir {
            "L" => Self::Left(num),
            "R" => Self::Right(num),
            _ => panic!("Failed to parse direction. Must be L or R. Got {dir}"),
        }
    }
}

fn part1(dirs: &[Dir]) -> i16 {
    let mut dial = 50;
    let mut zeroes = 0;

    for dir in dirs.iter() {
        dial = (dial + dir.to_i16()).rem_euclid(100);
        if dial == 0 {
            zeroes += 1;
        }
    }

    zeroes
}

fn part2(dirs: &[Dir]) -> i16 {
    let mut dial = 50;
    let mut zeroes = 0;

    for dir in dirs.iter() {
        let mag = dir.to_i16().abs();

        for _ in 0..mag {
            dial = (dial + dir.numeric_dir()).rem_euclid(100);
            if dial == 0 {
                zeroes += 1;
            }
        }
    }

    zeroes
}

fn main() {
    let dirs: Vec<Dir> = parse_input(1)
        .iter()
        .map(|line| Dir::from_line(line))
        .collect();

    println!("Part 1: {}", part1(&dirs));
    println!("Part 2: {}", part2(&dirs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: [Dir; 10] = [
        Dir::Left(68),
        Dir::Left(30),
        Dir::Right(48),
        Dir::Left(5),
        Dir::Right(60),
        Dir::Left(55),
        Dir::Left(1),
        Dir::Left(99),
        Dir::Right(14),
        Dir::Left(82),
    ];

    #[test]
    fn calculates_part1() {
        let fixture: Vec<Dir> = FIXTURE.to_vec();
        assert_eq!(part1(&fixture), 3);
    }

    #[test]
    fn calculates_part2() {
        let fixture: Vec<Dir> = FIXTURE.to_vec();
        assert_eq!(part2(&fixture), 6);
    }

    #[test]
    fn parses_left() {
        assert_eq!(Dir::from_line(&"L50".to_string()), Dir::Left(50))
    }

    #[test]
    fn parses_right() {
        assert_eq!(Dir::from_line(&"R20".to_string()), Dir::Right(20))
    }

    #[test]
    #[should_panic]
    fn invalid_dir_panic() {
        Dir::from_line(&"W20".to_string());
    }

    #[test]
    #[should_panic]
    fn invalid_num_panic() {
        Dir::from_line(&"RHI".to_string());
    }
}
