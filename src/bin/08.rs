#![feature(binary_heap_into_iter_sorted)]
use std::{
    array,
    collections::{BinaryHeap, VecDeque},
    time::Instant,
};

use aoc_utils::{
    grid::{Grid, Point},
    union_find::UnionFind,
};
use lina::{Point2, Vec2, Vec3};

advent_of_code::solution!(8);

type JBox = Vec3<i64>;

trait Distance{
    fn distance(&self, other: &JBox) -> i64;}

impl Distance for JBox {
    fn distance(&self, other: &JBox) -> i64 {
        (*other - *self).length2()
    }
}

fn parse(input: &str) -> Vec<JBox> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let mut iter = line
                .split(",")
                .map(|x| x.parse().expect("Expect lines to contain numeric strings"));
            let [a, b, c]: [i64; 3] =
                array::from_fn(|_| iter.next().expect("lines should have exactly 3 integers"));
            Vec3::new(a, b, c)
        })
        .collect()
}

type AdjacencyList = Vec<Vec<usize>>;

pub fn part_one(input: &str) -> Option<usize> {
    Some(largest_circuits_from_shortest_connections(input, 1000))
}

pub fn largest_circuits_from_shortest_connections(
    input: &str,
    no_shortest_connections: usize,
) -> usize {
    let now = Instant::now();

    let jboxes = parse(input);
    let n = jboxes.len();

    let mut graph_matrix =
        Grid::new_with_dimensions_uniform(Vec2::new(jboxes.len() as i32, n as i32), false);
    let mut graph_list: AdjacencyList = vec![vec![]; n];

    let distances = find_distances(&jboxes);
    for (_, u, v) in distances.into_iter_sorted().take(no_shortest_connections) {
        graph_matrix[Point2::new(u, v)] = true;
        graph_matrix[Point2::new(u, v)] = true;
        graph_list[u].push(v);
        graph_list[v].push(u);
    }

    let now2 = Instant::now();

    #[cfg(debug_assertions)]
    {
        graph_matrix.map(|&x| if x { '1' } else { ' ' }).print();
        eprintln!(
            "{}",
            graph_list
                .iter()
                .map(|x| x
                    .iter()
                    .map(|y| y.to_string())
                    .collect::<Vec<_>>()
                    .join(" "))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }

    let mut colours = vec![0; n];
    let mut working_colour = 1;
    for uncoloured in 0..n {
        if colours[uncoloured] != 0 {
            continue;
        }
        bfs(uncoloured, &graph_list, working_colour, &mut colours);
        working_colour += 1;
    }

    let now3 = Instant::now();

    let mut counts = vec![0; working_colour as usize];
    for &c in colours.iter() {
        counts[c as usize] += 1;
    }

    let r = BinaryHeap::from(counts)
        .into_iter_sorted()
        .take(3)
        .fold(1, |a, b| a * b);

    let now4 = Instant::now();
    eprintln!("Nearest neighbours: {:#?}", now2 - now);
    eprintln!("BFS:                {:#?}", now3 - now2);
    eprintln!("Colouring Max:      {:#?}", now4 - now3);

    r
}

fn find_distances(jboxes: &[JBox]) -> BinaryHeap<(i64, usize, usize)> {
    let mut distances = BinaryHeap::new();

    for (i, j_i) in jboxes.iter().enumerate() {
        for (j, j_j) in jboxes.iter().enumerate().skip(i + 1) {
            distances.push((-j_i.distance(j_j), i, j));
        }
    }

    distances
}

fn bfs(src: usize, graph: &AdjacencyList, colour: u32, colours: &mut Vec<u32>) {
    let mut queue = VecDeque::new();
    colours[src] = colour;
    queue.push_back(src);

    while let Some(v) = queue.pop_front() {
        for &u in graph[v].iter() {
            debug_assert!(colours[u] == 0 || colours[u] == colour);
            if colours[u] == 0 {
                colours[u] = colour;
                queue.push_back(u);
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let jboxes = parse(input);

    let distances = find_distances(&jboxes);

    let n = jboxes.len();
    let mut union = UnionFind::new(n);
    let mut last = None;
    for (_, u, v) in distances.into_iter_sorted() {
            // eprintln!("Connecting {:#?}, {:#?}", jboxes[u], jboxes[v]);
        union.union(u, v);
        if union.distinct_count() == 1 {
            last = Some((u, v));
            break;
        }
    }
    let last = last.expect("Eventually every junction must be connected");

    Some((jboxes[last.0].x * jboxes[last.1].x) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = largest_circuits_from_shortest_connections(
            &advent_of_code::template::read_file("examples", DAY),
            10,
        );
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
