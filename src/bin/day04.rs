use aoc2025_rs::read_lines;

const MAX_NEIGHBOURS: usize = 3;

const ADJ: &[(i32, i32)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn find_accessible(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut coords = Vec::new();
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '.' {
                continue;
            }

            let mut adj_count = 0;

            for (dx, dy) in ADJ {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx < 0 || nx >= width {
                    continue;
                }

                if ny < 0 || ny >= height {
                    continue;
                }

                if grid[ny as usize][nx as usize] == '@' {
                    adj_count += 1;
                }
            }

            if adj_count <= MAX_NEIGHBOURS {
                coords.push((x, y));
            }
        }
    }

    coords
}

fn part2(grid: &[Vec<char>]) -> usize {
    let mut grid = grid.to_owned();
    let mut count = 0;

    loop {
        let coords = find_accessible(&grid);

        if coords.is_empty() {
            break;
        }

        count += coords.len();

        for (x, y) in coords {
            grid[y][x] = '.';
        }
    }

    count
}

fn part1(grid: &[Vec<char>]) -> usize {
    find_accessible(grid).len()
}

fn lines_to_grid<S: AsRef<str>>(lines: &[S]) -> Vec<Vec<char>> {
    lines
        .iter()
        .map(|line| line.as_ref().chars().collect())
        .collect()
}

fn main() {
    let input = lines_to_grid(&read_lines(4));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        let input = lines_to_grid(&FIXTURE.lines().collect::<Vec<_>>());
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2() {
        let input = lines_to_grid(&FIXTURE.lines().collect::<Vec<_>>());
        assert_eq!(part2(&input), 43);
    }
}
