use std::{cmp::Ordering, fmt::Display};

advent_of_code::solution!(5);

#[derive(Clone, Copy, PartialEq, Debug)]
struct Interval {
    low: usize,
    high: usize,
}

impl Interval {
    fn of(low: usize, high: usize) -> Interval {
        debug_assert!(low <= high);
        Interval { low, high }
    }

    fn contains(&self, x: usize) -> Ordering {
        if x < self.low {
            Ordering::Less
        } else if x <= self.high {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.low, self.high)
    }
}

fn parse(input: &str) -> (Vec<Interval>, Vec<usize>) {
    let (first, second) = input.split_once("\n\n").expect("Expect two input portions");

    let intervals = first
        .split_ascii_whitespace()
        .map(|line| {
            let (low, high) = line
                .split_once("-")
                .expect("each line should be split by a -");
            Interval::of(low.parse().unwrap(), high.parse().unwrap())
        })
        .collect();

    let nums = second
        .split_ascii_whitespace()
        .map(|x| x.parse().expect("expect second half numbers"))
        .collect();

    (intervals, nums)
}

// fn merge_ranges(mut intervals: Vec<Interval>) -> Vec<Interval> {
//     intervals.push(Interval::of(usize::MAX, usize::MAX));
//     intervals.sort_unstable_by_key(|x| (x.low, x.high));

//     let mut current = intervals[0];
//     let mut out = vec![];
//     for i in intervals.iter().skip(1) {
//         if current.high + 1 >= i.low {
//             state.high
//         }
//     }

// }

fn merge_ranges(mut intervals: Vec<Interval>) -> Vec<Interval> {
    intervals.sort_by_key(|x| (x.low, (x.high as isize)));
    let mut init = intervals[0];
    intervals.push(Interval::of(usize::MAX, usize::MAX));
    let mut filtered_intervals: Vec<Interval> = intervals
        .iter()
        .skip(1)
        .scan(init, |state, x| {
            if state.high + 1 >= x.low {
                state.high = state.high.max(x.high);
                Some(None)
            } else {
                // new range
                let to_return: Interval = *state;

                *state = *x;
                Some(Some(to_return))
            }
        })
        .filter_map(|x| x)
        .collect();

    filtered_intervals
}

/// intervals are sorted by low and are disjoint
/// nums are sorted
fn count_numbers_in_range(intervals: &Vec<Interval>, nums: &Vec<usize>) -> usize {
    let mut interval_idx = 0;
    let mut num_idx = 0;
    let mut count = 0;

    while num_idx < nums.len() && interval_idx < intervals.len() {
        // dbg!(intervals[interval_idx], nums[num_idx]);
        match intervals[interval_idx].contains(nums[num_idx]) {
            Ordering::Less => num_idx += 1,
            Ordering::Equal => {
                count += 1;
                num_idx += 1;
            }
            Ordering::Greater => interval_idx += 1,
        }
    }

    count
}

fn is_single(intervals: &Vec<Interval>, num: usize) -> bool {
    intervals.iter().any(|x| x.contains(num).is_eq())
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut intervals, mut nums) = parse(input);
    let intervals = merge_ranges(intervals);
    Some(count_numbers_in_range(&intervals, &nums))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut intervals, _) = parse(input);
    let intervals = merge_ranges(intervals);

    Some(intervals.iter().map(|i| i.high + 1 - i.low).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_merge() {
        let of = Interval::of;
        let input = vec![of(5, 10), of(6, 10), of(12, 14), of(13, 16)];

        assert_eq!(merge_ranges(input), vec![of(5, 10), of(12, 16)]);
    }
}
