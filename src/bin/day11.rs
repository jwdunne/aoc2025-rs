use std::collections::VecDeque;

use aoc2025_rs::{read_lines, timed};

#[derive(Debug)]
struct Graph<'a> {
    keys: Vec<&'a str>,
    adj: Vec<Vec<usize>>,
    inbound: Vec<usize>,
}

impl<'a> Graph<'a> {
    fn count_paths(&self, src: &str, dest: &str) -> usize {
        let mut inbound = self.inbound.to_vec();
        let mut queue = VecDeque::new();

        for (i, &degree) in self.inbound.iter().enumerate() {
            if degree == 0 {
                queue.push_back(i);
            }
        }

        let mut order: Vec<usize> = Vec::new();

        while let Some(node) = queue.pop_front() {
            order.push(node);

            for &neighbor in &self.adj[node] {
                inbound[neighbor] -= 1;

                if inbound[neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }

        let src_i = self.keys.iter().position(|&key| key == src).unwrap();
        let dest_i = self.keys.iter().position(|&key| key == dest).unwrap();

        let mut paths: Vec<usize> = vec![0; self.keys.len()];
        paths[src_i] = 1;

        for &node in &order {
            for &neighbor in &self.adj[node] {
                paths[neighbor] += paths[node];
            }
        }

        paths[dest_i]
    }
}

fn part2(graph: &Graph) -> usize {
    (graph.count_paths("svr", "fft")
        * graph.count_paths("fft", "dac")
        * graph.count_paths("dac", "out"))
        + (graph.count_paths("svr", "dac")
            * graph.count_paths("dac", "fft")
            * graph.count_paths("fft", "out"))
}

impl<'a> FromIterator<&'a str> for Graph<'a> {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut keys: Vec<&'a str> = Vec::new();
        let mut edges: Vec<&'a str> = Vec::new();
        let mut adj: Vec<Vec<usize>> = Vec::new();

        for line in iter {
            if let Some((k, es)) = line.split_once(": ") {
                keys.push(k);
                edges.push(es);
            }
        }

        keys.push("out");

        let mut inbound: Vec<usize> = vec![0; keys.len()];

        for node_edges in edges {
            let edges = node_edges
                .split(' ')
                .filter_map(|edge| keys.iter().position(|key| key == &edge))
                .collect();

            for &edge in &edges {
                inbound[edge] += 1;
            }

            adj.push(edges);
        }

        adj.push(vec![]);

        Graph { keys, adj, inbound }
    }
}

fn main() {
    let lines = read_lines(11);
    let graph = lines.iter().map(|s| s.as_str()).collect::<Graph>();

    timed!("Part 1", graph.count_paths("you", "out"));
    timed!("Part 2", part2(&graph));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_paths() {
        let input: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        let graph = input.lines().collect::<Graph>();
        assert_eq!(graph.count_paths("you", "out"), 5);
    }

    #[test]
    fn test_part2() {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        let graph = input.lines().collect::<Graph>();
        assert_eq!(part2(&graph), 2);
    }
}
