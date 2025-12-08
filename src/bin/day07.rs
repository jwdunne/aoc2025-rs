use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use aoc2025_rs::read_lines;

fn parse<S: AsRef<str>>(lines: &[S]) -> Vec<Vec<usize>> {
    lines
        .iter()
        .map(|line| {
            line.as_ref()
                .char_indices()
                .filter(|(_, c)| *c == 'S' || *c == '^')
                .map(|(i, _)| i)
                .collect()
        })
        .collect()
}

fn part1(grid: &[Vec<usize>]) -> usize {
    let mut active = HashSet::from([grid[0][0]]);
    let mut splits = 0;

    for splitters in grid[1..].iter() {
        for splitter in splitters.iter() {
            if active.contains(splitter) {
                active.remove(splitter);
                active.insert(splitter - 1);
                active.insert(splitter + 1);

                splits += 1;
            }
        }
    }

    splits
}

fn part2(grid: &[Vec<usize>]) -> usize {
    let mut active = HashMap::from([(grid[0][0], 1)]);

    for splitters in grid[1..].iter() {
        let mut next_state: HashMap<usize, usize> = HashMap::new();

        for (&col, &count) in &active {
            if splitters.contains(&col) {
                *next_state.entry(col - 1).or_default() += count;
                *next_state.entry(col + 1).or_default() += count;
            } else {
                *next_state.entry(col).or_default() += count;
            }
        }

        active = next_state;
    }

    active.values().sum()
}

fn main() {
    let grid = parse(&read_lines(7));
    println!("Part 1: {}", part1(&grid));

    let now = Instant::now();
    let p2 = part2(&grid);
    let elapsed = now.elapsed();

    println!(
        "Part 2: {} ({}ms)",
        p2,
        (elapsed.as_micros() as f64) / 1000.0
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        let grid = parse(&FIXTURE.lines().collect::<Vec<_>>());
        assert_eq!(part1(&grid), 21);
    }

    #[test]
    fn test_part2() {
        let grid = parse(&FIXTURE.lines().collect::<Vec<_>>());
        assert_eq!(part2(&grid), 40);
    }
}
