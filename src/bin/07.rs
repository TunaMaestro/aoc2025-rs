use aoc_utils::grid::{Grid, Point};
use itertools::Itertools;
use lina::{Vec2};

advent_of_code::solution!(7);

type Position = u8;

#[derive(Debug, PartialEq)]
struct Input {
    width: Position,
    start: Position,
    splitters: Vec<Vec<Position>>,
}

fn parse(input: &str) -> Input {
    let start_pos = input.find("S").expect("expect one S") as Position;
    let width = input.find("\n").unwrap() as Position;

    let splitters = input
        .split("\n")
        .map(|line| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter_map(|(i, &c)| if c == b'^' { Some(i as Position) } else { None })
                .collect::<Vec<Position>>()
        })
        .filter(|x| !x.is_empty())
        .collect();

    Input {
        width,
        start: start_pos,
        splitters,
    }
}

struct SplitResult {
    splits: usize,
    final_beams: Vec<u8>,
}

impl SplitResult {
    fn new() -> Self {
        Self {
            splits: 0,
            final_beams: vec![],
        }
    }
}

fn simulate_beams(input: Input) -> SplitResult {
    let mut level = 0;
    let mut beams = vec![input.start];
    let mut next_beams: Vec<u8> = vec![];

    let mut splits = 0;

    for split_level in input.splitters {
        let mut splitter_idx = 0;
        let mut beam_idx = 0;

        while splitter_idx < split_level.len() && beam_idx < beams.len() {
            let splitter_pos = split_level[splitter_idx];
            let beam_pos = beams[beam_idx];

            if splitter_pos == beam_pos {
                splits += 1;
                if beam_pos > 0 && next_beams.last() != Some(&(beam_pos - 1)) {
                    next_beams.push(beam_pos - 1);
                }
                if beam_pos + 1 < input.width {
                    next_beams.push(beam_pos + 1);
                }
                splitter_idx += 1;
                beam_idx += 1;
            } else if splitter_pos < beam_pos {
                splitter_idx += 1;
            } else {
                next_beams.push(beam_pos);
                beam_idx += 1;
            }
        }

        (beams, next_beams) = (next_beams, beams);
        next_beams.clear();
    }

    SplitResult {
        splits,
        final_beams: beams,
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);

    let result = simulate_beams(input);

    Some(result.splits)
}

pub fn part_two(input: &str) -> Option<usize> {
    timelines(input)
}

fn timelines(input_str: &str /* , input: Input, splits: SplitResult */) -> Option<usize> {
    // let P(x, y) be the number of timelines a tachyon can take starting at x, y where y is zero at the last row
    let splitter_grid = Grid::read(input_str, |x| x == '^');

    let width = splitter_grid.dimension().x;
    let height = splitter_grid.dimension().y;
    let mut timelines = Grid::new_with_dimensions_uniform(splitter_grid.dimension(), 0_usize);
    for x in 0..width {
        timelines[Point::new(x, height - 1)] = 1;
    }
    for y in (0..height - 1).rev() {
        for x in 0..width {
            let p = Point::new(x, y);
            let below = Point::new(x, y + 1);
            if splitter_grid.get_or_default(below) {
                // the point below is a splitter, so combine from side two
                timelines[p] += timelines.get_or_default(Point::new(x - 1, y + 1));
                timelines[p] += timelines.get_or_default(Point::new(x + 1, y + 1));
            } else {
                // the point below is empty space so must have propagated from above
                timelines[p] += timelines.get_or_default(below);
            }
        }
    }

    let start = input_str.find('S').expect("Expect 'S' to be in first row");

    Some(timelines[Point::new(start as i32, 0)])

    // Each row `y` are (`x`, no of timelines a tachyon would take starting at (x, y))
    // let result: Vec<Vec<(u8, usize)>> =
    //     vec![splits.final_beams.iter().map(|&x| (x, 1_usize)).collect()];

    // for (y, split_level) in input.splitters.iter().rev().enumerate() {
    //     let mut splitter_idx = 0;
    //     let mut x = 0;

    //     let previous_layer_counts = result.last().unwrap();
    //     let mut layer: Vec<(u8, usize)> = vec![];

    //     while splitter_idx < split_level.len() && x < width {
    //         let splitter_pos = split_level[splitter_idx];

    //         if splitter_pos == x {
    //             if x > 0
    //                 && let Some(last) = layer.last_mut()
    //                 && last.0 == x - 1
    //             {
    //                 last.1 +=
    //             }
    //             if x + 1 < input.width {
    //                 // next_beams.push(beam_pos + 1);
    //             }
    //             splitter_idx += 1;
    //             x += 1;
    //         } else if splitter_pos < x {
    //             splitter_idx += 1;
    //         } else {
    //             // next_beams.push(beam_pos);
    //             x += 1;
    //         }
    //     }

    //     (beams, next_beams) = (next_beams, beams);
    //     next_beams.clear();
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_parse() {
        let map = "\
...S..
......
...^.^
..^.^.
.....
";

        assert_eq!(
            parse(map),
            Input {
                width: 6,
                start: 3,
                splitters: vec![vec![3, 5], vec![2, 4],]
            }
        );
    }
}
