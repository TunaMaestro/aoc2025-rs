use std::{
    array,
    collections::{BTreeMap, BinaryHeap, HashMap, HashSet},
    fmt::Display,
};

use aoc_utils::{
    bucket::BucketQueue,
    grid::{Grid, Point},
};
use pheap::PairingHeap;

advent_of_code::solution!(4);

type Tile = bool;

// impl Display for Tile {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

//         write!(f, "{c}")
//     }
// }

struct BumGrid {
    grid: Vec<Tile>,
    width: usize,
    height: usize,
}

impl BumGrid {
    fn get(&self, x: isize, y: isize) -> Tile {
        if (x < 0 || x >= self.width as isize || y < 0) {
            return false;
        }
        let x = x as usize;
        let y = y as usize;
        let idx = x + y * self.width;
        self.grid.get(idx).copied().unwrap_or(false)
    }
}

impl Display for BumGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                let c = match (self.get(j as isize, i as isize)) {
                    false => '.',
                    true => '@',
                };
                write!(f, "{}", c)?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

fn parse(input: &str) -> BumGrid {
    let width = input.find('\n').expect("1 newline");

    let vec: Vec<Tile> = input
        .as_bytes()
        .iter()
        .filter_map(|x| match *x {
            b'.' => Some(false),
            b'@' => Some(true),
            _ => None,
        })
        .collect();

    let height = vec.len() / width;

    BumGrid {
        grid: vec,
        width: width,
        height: height,
    }
}

fn count_rolls(grid: &BumGrid, x: isize, y: isize) -> usize {
    // eprintln!("{}", grid);
    let mut roll_count = 0;
    for x_offset in -1..=1 {
        for y_offset in -1..=1 {
            let x_ = x + x_offset;
            let y_ = y + y_offset;

            if !(y_offset == 0 && x_offset == 0) && grid.get(x_, y_) {
                roll_count += 1;
                // eprintln!("{x_offset} {y_offset} -> {x_} {y_}")
            }
        }
    }
    roll_count
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse(input);
    // println!("{grid}\n{} {}", grid.width, grid.height);
    let mut movable_rolls = 0;
    for i in 0..grid.height as isize {
        for j in 0..grid.width as isize {
            if grid.get(j, i) != true {
                continue;
            }
            if count_rolls(&grid, j, i) <= MAX_SURROUNDS {
                movable_rolls += 1;
            }
        }
    }
    Some(movable_rolls)
}

#[derive(Eq, PartialOrd, PartialEq)]
struct Node {
    x: isize,
    y: isize,
    neighbours: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.neighbours.cmp(&other.neighbours)
    }
}

const MAX_SURROUNDS: usize = 3;

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::read(input, |c| c == '@');
    let mut priorities = HashMap::new();
    for p in grid.iter_coordinates() {
        if grid.get_or_default(p) == false {
            continue;
        }

        let count = grid.neighbours_and_corners(p).iter().filter(|(x, y)| **y).count();
        priorities.insert(p, count);
    }

    // let neighbours_grid = Grid::new_with_dimensions(grid.dimension(), |p| {
    //     priorities
    //         .get(&p)
    //         .map(|x| format!("{x}"))
    //         .unwrap_or(" ".to_owned())
    // });

    // neighbours_grid.print();
    // return None;

    let mut queue: BucketQueue<Point, 9> = BucketQueue::create(priorities);

    let mut removed = 0;

    while let Some(node) = queue.pop_min() {
        // let mut to_print = grid.map(|&x| if x { 'o' } else { ' ' });
        // to_print[node.value] = '@';
        // to_print.print();
        // println!();
        if node.priority > MAX_SURROUNDS {
            break;
        }
        removed += 1;
        grid[node.value] = false;
        let neighbours = grid.neighbours_and_corners(node.value);
        for (p, is_roll) in neighbours {
            if !is_roll {
                continue;
            }
            queue.decrease_key(p, 1);
        }
    }

    Some(removed)
}

pub fn part_two_bad_queue(input: &str) -> Option<u64> {
    let mut grid = Grid::read(input, |c| c == '@');
    let mut queue: PairingHeap<Point, usize> = PairingHeap::new();
    for p in grid.iter_coordinates() {
        if grid.get_or_default(p) == false {
            continue;
        }

        let count = grid.neighbours_and_corners(p).iter().filter(|(x, y)| **y).count();
        queue.insert(p, count);
    }


    let mut removed = 0;

    while let Some(node) = queue.delete_min() {
        // let mut to_print = grid.map(|&x| if x { 'o' } else { ' ' });
        // to_print[node.value] = '@';
        // to_print.print();
        // println!();
        if node.1 > MAX_SURROUNDS {
            break;
        }
        removed += 1;
        grid[node.0] = false;
        let neighbours = grid.neighbours_and_corners(node.0);
        for (p, is_roll) in neighbours {
            if !is_roll {
                continue;
            }
            queue.decrease_prio(&p, 1);
        }
    }

    Some(removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }

    #[test]
    fn test_grid() {
        let grid = parse(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(count_rolls(&grid, 2, 0), 3);
    }
    #[test]
    fn test_grid_get() {
        let grid = parse(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(grid.get(3, 0), true);
    }
}
