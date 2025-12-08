use std::array::repeat;

use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Op {
    Plus,
    Times,
}

impl Op {
    fn eval(&self, a: usize, b: usize) -> usize {
        match self {
            Op::Plus => a + b,
            Op::Times => a * b,
        }
    }
}
impl TryFrom<&str> for Op {
    type Error = ();

    fn try_from(value: &str) -> Result<Op, Self::Error> {
        match value.trim() {
            "*" => Ok(Self::Times),
            "+" => Ok(Self::Plus),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for Op {
    type Error = ();

    fn try_from(value: u8) -> Result<Op, Self::Error> {
        match value {
            b'*' => Ok(Self::Times),
            b'+' => Ok(Self::Plus),
            _ => Err(()),
        }
    }
}

fn parse(input: &str) -> (usize, Vec<usize>, Vec<Op>) {
    let mut nums = vec![];
    let mut ops = vec![];

    for s in input.split_ascii_whitespace() {
        if let Ok(num) = s.parse() {
            nums.push(num);
        }
        if let Ok(op) = s.try_into() {
            ops.push(op);
        }
    }

    assert_eq!(nums.len() % ops.len(), 0);
    let operands = nums.len() / ops.len();
    (operands, nums, ops)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (operands, nums, ops) = parse(input);
    let equations = nums.len() / operands;
    Some(
        (0..equations)
            .map(|i| {
                (0..operands)
                    .map(|arg_idx| nums[i + arg_idx * equations])
                    .reduce(|a, b| ops[i].eval(a, b))
                    .expect("expected there to be at least 2 operands")
            })
            .sum(),
    )
}

fn index(width: usize, x: usize, y: usize) -> usize {
    x + width * y
}

pub fn part_two(input: &str) -> Option<usize> {
    let width = input.find('\n').unwrap();
    let depth = input.len() / width;
    // dbg!(width, depth);
    let mut transposed: Vec<u8> =
        Vec::from_iter(std::iter::repeat(b'\n').take(input.len() + width - depth));

    for row_idx in 0..depth {
        for col_idx in 0..width {
            let c = input.as_bytes()[index(width + 1, col_idx, row_idx)];
            // eprintln!("{} {} {}", row_idx, col_idx, char::from(c));
            transposed[index(depth + 1, row_idx, col_idx)] = c;
        }
    }

    let transposed_str = str::from_utf8(&transposed).unwrap();
    // println!("{}", transposed_str);

    // let equation_count = input
    //     .split("\n")
    //     .next()
    //     .expect("input should have at least one line")
    //     .split_ascii_whitespace()
    //     .count();

    // let number_width = width / (equation_count + 1);
    let number_width = depth - 1;

    let mut sum = 0;

    // for eq_block in transposed_str.split("\n\n") {
    //     let operator = Op::try_from(eq_block.as_bytes()[number_width])
    //         .expect("Expect the byte at number_width to be * or + at the start of each block");
    //     let product = eq_block
    //         .split("\n")
    //         .inspect(|&s| {
    //             eprintln!("'{}' '{}'", s, (s[..number_width]).to_owned());
    //         })
    //         .filter_map(|s| {
    //             s[..number_width].trim().parse::<usize>().ok()

    //         })
    //         .chunk_by(key)
    //         .inspect(|u| {
    //             eprintln!("{}", u);
    //         })
    //         .reduce(|a, b| operator.eval(a, b))
    //         .unwrap();
    //     eprintln!();
    //     sum += product;
    // }

    let mut iter = transposed_str.split("\n");
    let line = iter.next().unwrap();
    let num_part = &line[..line.len() - 1].trim();
    let op_part = &line[line.len() - 1..];

    let mut accumulator: usize = num_part.parse().unwrap();
    let mut op = Op::try_from(op_part).unwrap();

    for line in iter {
        if (line.trim() == "") {
            continue;
        }
        let num_part = &line[..line.len() - 1].trim();
        let op_part = &line[line.len() - 1..];
        let num: usize = num_part.parse().unwrap();
        if let Ok(new_op) = Op::try_from(op_part) {
            op = new_op;
            sum += accumulator;
            // eprintln!("+ <-- {accumulator}");
            accumulator = num;
        } else {
            accumulator = op.eval(accumulator, num);
        }
    }

    sum += accumulator;

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_parse() {
        let parse = parse(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            parse,
            (
                3,
                vec![123, 328, 51, 64, 45, 64, 387, 23, 6, 98, 215, 314,],
                vec![Op::Times, Op::Plus, Op::Times, Op::Plus,]
            )
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
