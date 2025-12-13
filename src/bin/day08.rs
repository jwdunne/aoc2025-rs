use std::str::FromStr;

use aoc2025_rs::{read_lines, timed};
use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl Point {
    fn distance(&self, other: &Self) -> u64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;
        dx.pow(2) + dy.pow(2) + dz.pow(2)
    }
}

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, rest) = s.split_once(',').ok_or(ParsePointError)?;
        let (y_str, z_str) = rest.split_once(',').ok_or(ParsePointError)?;

        let x = x_str.parse::<u64>().map_err(|_| ParsePointError)?;
        let y = y_str.parse::<u64>().map_err(|_| ParsePointError)?;
        let z = z_str.parse::<u64>().map_err(|_| ParsePointError)?;

        Ok(Self { x, y, z })
    }
}

struct DisjointSet {
    parent: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let mut root = self.parent[x];

        while root != self.parent[root] {
            root = self.parent[root];
        }

        root
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let mut root_x = self.parent[x];
        let mut root_y = self.parent[y];

        while root_x != self.parent[root_x] {
            root_x = self.parent[root_x];
        }

        while root_y != self.parent[root_y] {
            root_y = self.parent[root_y];
        }

        if root_x == root_y {
            return false;
        }

        self.parent[root_x] = root_y;
        true
    }
}

fn distances(points: &[Point]) -> Vec<(u64, usize, usize)> {
    let len = points.len();

    let mut distances: Vec<_> = (0..len)
        .into_par_iter()
        .flat_map_iter(|p1| {
            ((p1 + 1)..len).map(move |p2| (points[p1].distance(&points[p2]), p1, p2))
        })
        .collect();

    distances.par_sort_unstable_by_key(|d| d.0);

    distances
}

fn part1(points: &[Point], max_shortest: usize) -> usize {
    let len = points.len();
    let distances = distances(points);

    let mut disjoint_set = DisjointSet::new(len);

    for &(_, p1, p2) in distances.iter().take(max_shortest) {
        disjoint_set.union(p1, p2);
    }

    let mut root_sizes: Vec<usize> = vec![0; len];

    for i in 0..len {
        let root = disjoint_set.find(i);
        root_sizes[root] += 1;
    }

    root_sizes.par_sort_unstable_by(|a, b| b.cmp(a));

    root_sizes.iter().take(3).product()
}

fn part2(points: &[Point]) -> u64 {
    let len = points.len();
    let distances = distances(points);

    let mut disjoint_set = DisjointSet::new(len);
    let mut last_union: Option<(usize, usize)> = None;
    let mut merge_count = 0;
    for &(_, p1, p2) in distances.iter() {
        if disjoint_set.union(p1, p2) {
            last_union = Some((p1, p2));

            merge_count += 1;
            if merge_count == len - 1 {
                break;
            }
        }
    }

    let (p1, p2) = last_union.unwrap();
    points[p1].x * points[p2].x
}

fn main() {
    let points = read_lines(8)
        .iter()
        .map(|line| line.parse::<Point>().expect("Could not parse point"))
        .collect::<Vec<_>>();

    timed!("Part 1", part1(&points, 1000));
    timed!("Part 2", part2(&points));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_point() {
        assert_eq!(
            "1,1,1".parse::<Point>().unwrap(),
            Point { x: 1, y: 1, z: 1 }
        );
    }

    const FIXTURE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1() {
        let lines = FIXTURE
            .lines()
            .map(|line| line.parse::<Point>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(part1(&lines, 10), 40);
    }

    #[test]
    fn test_part2() {
        let lines = FIXTURE
            .lines()
            .map(|line| line.parse::<Point>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(part2(&lines), 25272);
    }
}
