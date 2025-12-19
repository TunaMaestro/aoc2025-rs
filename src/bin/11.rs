use std::{collections::HashMap, str::FromStr};

advent_of_code::solution!(11);

fn count_paths(graph: &Graph, src: usize, out: usize) -> usize {
    let mut p = vec![vec![0; graph.size]];

    /// let p(len, i) = p[len-1][i] be the number of ways of reaching node i from src in exactly len steps.

    /// base case of p(1, i)
    for &i in &graph.graph[src] {
        p[0][i] = 1;
    }

    let mut cont = true;

    while cont {
        cont = false;
        let prev_layer = p.last().unwrap();
        let mut working_layer = vec![0; graph.size];
        // eprintln!("{} {}", p.len(), prev_layer.iter().map(|x| format!("{x}")).collect::<Vec<_>>().join(" "));
        for v in 0..graph.size {
            // eprintln!("{} {}", prev_layer[v], working_layer[u]);
            if prev_layer[v] == 0 {
                continue;
            }
            cont = true;

            let edges = &graph.graph[v];
            for &u in edges {
                working_layer[u] += prev_layer[v];
            }
        }
        p.push(working_layer);
    }

    let ans = p.iter().map(|x| x[out]).sum();
    ans
}

pub fn part_one(input: &str) -> Option<usize> {
    let graph: Graph = input.parse().unwrap();

    Some(count_paths(&graph, graph.you, graph.out))
}

pub fn part_two(input: &str) -> Option<usize> {
    let graph: Graph = input.parse().unwrap();

    let dac_fft = count_paths(&graph, graph.dac, graph.fft);
    let fft_dac = count_paths(&graph, graph.fft, graph.dac);

    let (first, second) = if dac_fft != 0 {
        (graph.dac, graph.fft)
    } else {
        (graph.fft, graph.dac)
    };

    let first_leg = count_paths(&graph, graph.svr, first);
    let middle_leg = fft_dac + dac_fft;
    let last_leg =count_paths(&graph, second, graph.out);
    let count = first_leg * middle_leg * last_leg;

    Some(count)
}

#[derive(Debug)]
struct Graph {
    you: usize,
    out: usize,
    dac: usize,
    fft: usize,
    svr: usize,
    size: usize,
    graph: Vec<Vec<usize>>,
}

impl FromStr for Graph {
    type Err = &'static str;

    fn from_str<'a>(s: &'a str) -> Result<Self, Self::Err> {
        let mut map: HashMap<&'a str, usize> = HashMap::from([
            ("you", 0),
            ("out", 1),
            ("dac", 2),
            ("fft", 3),
            ("svr", 4),
        ]);
        let s = s.trim();
        let mut node_idx = 5;

        for row in s.split("\n") {
            let Some((node_label, neighbours)) = row.split_once(": ") else {
                return Err("No colon in line");
            };
            let node = node_idx;
            if map.contains_key(node_label) {
                continue;
            }
            map.insert(node_label, node);
            node_idx += 1;
        }

        let mut graph = vec![vec![]; map.len()];

        for row in s.split("\n") {
            let Some((node_label, neighbours)) = row.split_once(": ") else {
                return Err("No colon in line");
            };
            let node = *map.get(node_label).ok_or("node not already found")?;
            let mut working = &mut graph[node];
            for neighbour in neighbours.split(" ") {
                let u = *map.get(neighbour).ok_or("node not already found")?;
                working.push(u);
            }
        }
        let size = graph.len();
        Ok(Graph {
            graph,
            size,
            you: 0,
            out: 1,
            dac: 2,
            fft: 3,
            svr: 4,
        })
    }
}

fn to_num<'a>(map: &mut HashMap<&'a str, usize>, node_idx: &mut usize, s: &'a str) -> usize {
    match map.get(s) {
        Some(x) => *x,
        None => {
            let out = *node_idx;
            map.insert(s, out);
            *node_idx += 1;
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
