use std::str::FromStr;

use aoc2025_rs::{read_lines, timed};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq)]
enum Side {
    Left,
    On,
    Right,
}

impl Point {
    #[inline]
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    #[inline]
    fn rect_area(&self, other: &Self) -> i64 {
        let dx = (other.x - self.x).abs() + 1;
        let dy = (other.y - self.y).abs() + 1;
        dx * dy
    }

    #[inline]
    fn side_of(&self, p1: &Self, p2: &Self) -> Side {
        let side = ((p2.x - p1.x) * (self.y - p1.y)) - ((self.x - p1.x) * (p2.y - p1.y));
        match side.signum() {
            1 => Side::Left,
            -1 => Side::Right,
            _ => Side::On,
        }
    }

    fn within(&self, polygon: &[Point]) -> bool {
        let mut wn = 0;

        for i in 0..polygon.len() {
            let p1 = &polygon[i];
            let p2 = &polygon[(i + 1) % polygon.len()];
            match self.side_of(p1, p2) {
                Side::On => {
                    let in_x = self.x >= p1.x.min(p2.x) && self.x <= p1.x.max(p2.x);
                    let in_y = self.y >= p1.y.min(p2.y) && self.y <= p1.y.max(p2.y);
                    if in_x && in_y {
                        return true;
                    }
                }
                Side::Left if p1.y <= self.y && p2.y > self.y => wn += 1,
                Side::Right if p1.y > self.y && p2.y <= self.y => wn -= 1,
                _ => {}
            }
        }

        wn != 0
    }
}

#[derive(Debug)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s.split_once(',').ok_or(ParsePointError)?;

        let x: i64 = x_str.parse().map_err(|_| ParsePointError)?;
        let y: i64 = y_str.parse().map_err(|_| ParsePointError)?;

        Ok(Self::new(x, y))
    }
}

fn part1(points: &[Point]) -> i64 {
    let len = points.len();
    (0..len)
        .flat_map(|p1| (p1 + 1..len).map(move |p2| points[p1].rect_area(&points[p2])))
        .max()
        .unwrap()
}

fn part2(points: &[Point]) -> usize {
    let mut xs: Vec<i64> = points.iter().map(|p| p.x).collect();
    let mut ys: Vec<i64> = points.iter().map(|p| p.y).collect();
    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();

    let cw = xs.len();
    let ch = ys.len();
    let mut p = Point::new(0, 0);

    let mut grid = vec![0usize; ch * cw];
    for cy in 0..ch {
        for cx in 0..cw {
            p.x = xs[cx];
            p.y = ys[cy];
            grid[cy * cw + cx] = usize::from(p.within(points));
        }
    }

    let pw = cw + 1;
    let mut prefix = vec![0usize; (ch + 1) * pw];

    for cy in 1..=ch {
        for cx in 1..=cw {
            prefix[cy * pw + cx] = grid[(cy - 1) * cw + (cx - 1)]
                + prefix[(cy - 1) * pw + cx]
                + prefix[cy * pw + (cx - 1)]
                - prefix[(cy - 1) * pw + (cx - 1)];
        }
    }

    let len = points.len();
    let mut max_area: usize = 0;

    for i in 0..len {
        for j in (i + 1)..len {
            let px1 = points[i].x.min(points[j].x);
            let py1 = points[i].y.min(points[j].y);
            let px2 = points[i].x.max(points[j].x);
            let py2 = points[i].y.max(points[j].y);

            let cx1 = xs.binary_search(&px1).unwrap();
            let cy1 = ys.binary_search(&py1).unwrap();
            let cx2 = xs.binary_search(&px2).unwrap() + 1;
            let cy2 = ys.binary_search(&py2).unwrap() + 1;

            let actual_count =
                prefix[cy2 * pw + cx2] - prefix[cy1 * pw + cx2] - prefix[cy2 * pw + cx1]
                    + prefix[cy1 * pw + cx1];
            let expected_count = (cx2 - cx1) * (cy2 - cy1);

            if actual_count == expected_count {
                let area = ((px2 - px1 + 1) * (py2 - py1 + 1)) as usize;
                max_area = max_area.max(area);
            }
        }
    }

    max_area
}

fn part2_raycast(points: &[Point]) -> i64 {
    let mut xs: Vec<i64> = points.iter().map(|p| p.x).collect();
    let mut ys: Vec<i64> = points.iter().map(|p| p.y).collect();
    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();

    let cw = xs.len();
    let ch = ys.len();
    let n = points.len();

    let v_edges: Vec<(i64, i64, i64)> = (0..n)
        .filter_map(|i| {
            let p1 = points[i];
            let p2 = points[(i + 1) % n];
            (p1.x == p2.x).then(|| (p1.x, p1.y.min(p2.y), p1.y.max(p2.y)))
        })
        .collect();

    let h_edges: Vec<(i64, i64, i64)> = (0..n)
        .filter_map(|i| {
            let p1 = points[i];
            let p2 = points[(i + 1) % n];
            (p1.y == p2.y).then(|| (p1.y, p1.x.min(p2.x), p1.x.max(p2.x)))
        })
        .collect();

    let grid: Vec<u8> = ys
        .par_iter()
        .flat_map(|&y| {
            let mut crossings: Vec<i64> = v_edges
                .iter()
                .filter(|&&(_, y_min, y_max)| y_min <= y && y < y_max)
                .map(|&(x, _, _)| x)
                .collect();
            crossings.sort_unstable();

            let v_edge_xs: Vec<i64> = v_edges
                .iter()
                .filter(|&&(_, y_min, y_max)| y_min <= y && y <= y_max)
                .map(|&(x, _, _)| x)
                .collect();

            let h_at_y: Vec<(i64, i64)> = h_edges
                .iter()
                .filter(|&&(ey, _, _)| ey == y)
                .map(|&(_, x_min, x_max)| (x_min, x_max))
                .collect();

            xs.iter()
                .map(|&x| {
                    if v_edge_xs.contains(&x) {
                        return 1;
                    }

                    if h_at_y
                        .iter()
                        .any(|&(x_min, x_max)| x_min <= x && x <= x_max)
                    {
                        return 1;
                    }

                    (crossings.partition_point(|&cx| cx < x) & 1) as u8
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let pw = cw + 1;
    let mut prefix = vec![0u32; (ch + 1) * pw];

    for cy in 1..=ch {
        for cx in 1..=cw {
            let idx = cy * pw + cx;
            prefix[idx] =
                grid[(cy - 1) * cw + (cx - 1)] as u32 + prefix[idx - pw] + prefix[idx - 1]
                    - prefix[idx - pw - 1];
        }
    }

    let len = points.len();
    (0..len)
        .into_par_iter()
        .flat_map(|i| (i + 1..len).into_par_iter().map(move |j| (i, j)))
        .filter_map(|(i, j)| {
            let p1 = points[i];
            let p2 = points[j];

            let px1 = xs.binary_search(&p1.x).unwrap();
            let py1 = ys.binary_search(&p1.y).unwrap();
            let px2 = xs.binary_search(&p2.x).unwrap();
            let py2 = ys.binary_search(&p2.y).unwrap();

            let (cx1, cx2) = (px1.min(px2), px1.max(px2) + 1);
            let (cy1, cy2) = (py1.min(py2), py1.max(py2) + 1);

            let actual = prefix[cy2 * pw + cx2] - prefix[cy1 * pw + cx2] - prefix[cy2 * pw + cx1]
                + prefix[cy1 * pw + cx1];

            let expected = ((cx2 - cx1) * (cy2 - cy1)) as u32;

            (actual == expected).then(|| p1.rect_area(&p2))
        })
        .max()
        .unwrap_or(0)
}

fn main() {
    let points: Vec<Point> = read_lines(9)
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();

    timed!("Part 1", part1(&points));
    timed!("Part 2", part2(&points));
    timed!("Part 2 [ray cast]", part2_raycast(&points));
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1() {
        let points: Vec<Point> = FIXTURE.lines().map(|line| line.parse().unwrap()).collect();
        assert_eq!(part1(&points), 50);
    }

    #[test]
    fn test_part2() {
        let points: Vec<Point> = FIXTURE.lines().map(|line| line.parse().unwrap()).collect();
        assert_eq!(part2(&points), 24);
    }

    #[test]
    fn test_part2_raycast() {
        let points: Vec<Point> = FIXTURE.lines().map(|line| line.parse().unwrap()).collect();
        assert_eq!(part2_raycast(&points), 24);
    }

    #[test]
    fn test_side_of() {
        let p1 = Point::new(3, 0);
        let p2 = Point::new(3, 3);

        assert_eq!(Point::new(0, 0).side_of(&p1, &p2), Side::Left);
        assert_eq!(Point::new(3, 1).side_of(&p1, &p2), Side::On);
        assert_eq!(Point::new(4, 1).side_of(&p1, &p2), Side::Right);
    }

    #[test]
    fn test_within() {
        let points = &[
            Point::new(2, 2),
            Point::new(4, 2),
            Point::new(4, 4),
            Point::new(2, 4),
        ];

        assert!(!Point::new(0, 0).within(points));
        assert!(!Point::new(5, 5).within(points));
        assert!(Point::new(3, 3).within(points));
        assert!(Point::new(3, 2).within(points));
    }
}
