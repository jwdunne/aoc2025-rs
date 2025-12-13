use std::str::FromStr;

use aoc2025_rs::{read_lines, timed};
use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, Variable, constraint, default_solver,
    variable,
};

#[derive(Debug)]
struct GF2System {
    width: u8,
    target: u16,
    buttons: Vec<u16>,
}

impl GF2System {
    fn new(width: u8, target: u16, buttons: Vec<u16>) -> Self {
        Self {
            width,
            target,
            buttons,
        }
    }

    fn solve(&self) -> u16 {
        let num_buttons = self.buttons.len();

        let mut augmented: Vec<u16> = (0..self.width)
            .map(|light| {
                let button_bits: u16 = self
                    .buttons
                    .iter()
                    .enumerate()
                    .filter(|&(_, button)| button & (1 << light) != 0)
                    .map(|(button_idx, _)| 1u16 << button_idx)
                    .fold(0, |acc, n| acc | n);

                let target_bit = ((self.target >> light) & 1) << num_buttons;

                button_bits | target_bit
            })
            .collect();

        let mut pivot_row = 0;

        for col in 0..num_buttons {
            let mask = 1 << col;

            let maybe_pivot = augmented[pivot_row..]
                .iter()
                .position(|&row| row & mask != 0)
                .map(|i| i + pivot_row);

            if let Some(found) = maybe_pivot {
                augmented.swap(pivot_row, found);

                let pivot = augmented[pivot_row];

                for (row_idx, row) in augmented.iter_mut().enumerate() {
                    if row_idx != pivot_row && *row & mask != 0 {
                        *row ^= pivot;
                    }
                }

                pivot_row += 1;
            }
        }

        let button_mask: u16 = (1u16 << num_buttons) - 1;

        let mut free_mask: u16 = 0;
        for col in 0..num_buttons {
            let count = augmented
                .iter()
                .filter(|&row| row & (1 << col) != 0)
                .count();

            if count > 1 {
                free_mask |= 1 << col;
            }
        }

        let mut base_presses: u16 = 0;
        let mut constrained_rows: Vec<u16> = Vec::new();

        for &row in &augmented {
            let button_bits = row & button_mask;
            let target = (row >> num_buttons) & 1;

            if button_bits == 0 {
                continue;
            }

            if (button_bits & free_mask) != 0 {
                constrained_rows.push(row);
            } else {
                base_presses += target;
            }
        }

        let mut min_presses = u16::MAX;
        let mut combo = free_mask;
        loop {
            let free_presses = combo.count_ones() as u16;
            let mut presses = base_presses + free_presses;

            for &row in &constrained_rows {
                let target = (row >> num_buttons) & 1;
                let free_parity = (row & combo).count_ones() as u16 & 1;
                presses += target ^ free_parity;
            }

            min_presses = min_presses.min(presses);

            if combo == 0 {
                break;
            }
            combo = (combo - 1) & free_mask;
        }

        min_presses
    }
}

#[derive(Debug)]
struct ParseGF2SystemError;

impl FromStr for GF2System {
    type Err = ParseGF2SystemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pieces: Vec<_> = s.split_whitespace().collect();

        let target_str = pieces.first().ok_or(ParseGF2SystemError)?;

        let mut width: u8 = 0;
        let mut target: u16 = 0;
        for (i, c) in target_str.trim_matches(['[', ']']).char_indices() {
            if c == '#' {
                target |= 1 << i;
            }

            width += 1;
        }

        let system = pieces
            .iter()
            .skip(1)
            .filter(|&row| row.starts_with('('))
            .map(|row| {
                row.trim_matches(['(', ')'])
                    .split(',')
                    .map(|n| 1 << n.parse::<u8>().unwrap())
                    .fold(0_u16, |equation, bit| equation | bit)
            })
            .collect();

        Ok(Self::new(width, target, system))
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        (b, a) = (a % b, b);
    }

    a
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

#[derive(Debug)]
struct IntSystem {
    target: Vec<u16>,
    buttons: Vec<Vec<u16>>,
}

impl IntSystem {
    fn new(target: Vec<u16>, buttons: Vec<Vec<u16>>) -> Self {
        Self { target, buttons }
    }

    fn solve(&self) -> u64 {
        let num_buttons = self.buttons.len();

        let mut augmented: Vec<Vec<i64>> = (0..self.target.len())
            .map(|n| {
                let mut button_row = self
                    .buttons
                    .iter()
                    .map(move |row| if row.contains(&(n as u16)) { 1 } else { 0 })
                    .collect::<Vec<i64>>();

                button_row.push(self.target[n] as i64);
                button_row
            })
            .collect();

        let mut pivot_row: usize = 0;

        for col in 0..num_buttons {
            let maybe_pivot = augmented[pivot_row..]
                .iter()
                .position(|row| row[col] != 0)
                .map(|i| i + pivot_row);

            if let Some(found) = maybe_pivot {
                augmented.swap(pivot_row, found);

                let pivot = augmented[pivot_row].clone();

                for (row_idx, row) in augmented.iter_mut().enumerate() {
                    if row_idx != pivot_row && row[col] != 0 {
                        let lcm = lcm(pivot[col], row[col]);
                        let scale1 = lcm / pivot[col];
                        let scale2 = lcm / row[col];

                        for (new_col, cell) in row.iter_mut().enumerate() {
                            *cell = scale2 * *cell - scale1 * pivot[new_col];
                        }
                    }
                }

                pivot_row += 1;
            }
        }

        for row in augmented.iter_mut() {
            let maybe_pivot = row[..num_buttons].iter().find(|&&v| v != 0);
            if let Some(&pivot) = maybe_pivot
                && pivot < 0
            {
                for cell in row.iter_mut() {
                    *cell = -*cell;
                }
            }
        }

        let mut vars = ProblemVariables::new();

        let x: Vec<Variable> = (0..num_buttons)
            .map(|i| vars.add(variable().integer().min(0).name(format!("x{}", i))))
            .collect();

        let objective: Expression = x.iter().copied().sum();
        let mut problem = vars.minimise(objective).using(default_solver);

        for (i, _) in augmented.iter().enumerate() {
            let lhs: Expression = augmented[i][..num_buttons]
                .iter()
                .zip(x.iter())
                .map(|(&coef, &var)| (coef as i32) * var)
                .sum();

            let rhs = augmented[i][num_buttons] as i32;

            problem = problem.with(constraint!(lhs.clone() <= rhs));
            problem = problem.with(constraint!(lhs >= rhs));
        }

        let solution = problem.solve().unwrap();

        x.iter()
            .map(|&v| solution.value(v).round() as i64)
            .sum::<i64>() as u64
    }
}

#[derive(Debug)]
struct ParseIntSystemError;

impl FromStr for IntSystem {
    type Err = ParseIntSystemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pieces: Vec<&str> = s.split_whitespace().collect();

        let buttons: Vec<Vec<u16>> = pieces
            .iter()
            .skip(1)
            .filter(|&row| row.starts_with('('))
            .map(|row| {
                row.trim_matches(['(', ')'])
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect();

        if buttons.is_empty() {
            return Err(ParseIntSystemError);
        }

        let target: Vec<u16> = pieces
            .iter()
            .last()
            .ok_or(ParseIntSystemError)?
            .trim_matches(['{', '}'])
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        if target.is_empty() {
            return Err(ParseIntSystemError);
        }

        Ok(IntSystem::new(target, buttons))
    }
}

fn part1(systems: &[GF2System]) -> u16 {
    systems.iter().map(|s| s.solve()).sum()
}

fn part2(systems: &[IntSystem]) -> u64 {
    let mut sum: u64 = 0;
    for sys in systems {
        sum += sys.solve();
    }
    sum
}

fn main() {
    let lines = read_lines(10);
    let gf2_systems: Vec<GF2System> = lines.iter().map(|line| line.parse().unwrap()).collect();
    let int_systems: Vec<IntSystem> = lines.iter().map(|line| line.parse().unwrap()).collect();

    timed!("Part 1", part1(&gf2_systems));
    timed!("Part 2", part2(&int_systems));
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    const MULTI_VARIABLE: &str = "[....] (0,1) (2) (3) (1,3) (0) (0,1,3) {150,22,8,18}";

    #[test]
    fn test_part1() {
        let systems: Vec<GF2System> = FIXTURE.lines().map(|line| line.parse().unwrap()).collect();
        assert_eq!(part1(&systems), 7);
    }

    #[test]
    fn test_part2() {
        let systems: Vec<IntSystem> = FIXTURE.lines().map(|line| line.parse().unwrap()).collect();
        assert_eq!(part2(&systems), 33);
    }

    #[test]
    fn test_part2_multiple_free_variables() {
        let systems: Vec<IntSystem> = MULTI_VARIABLE
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();
        assert_eq!(part2(&systems), 158);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(12, 18), 36);
    }
}
