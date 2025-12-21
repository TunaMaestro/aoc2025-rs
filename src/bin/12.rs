use std::str::FromStr;

use aoc_utils::grid::{Grid, Point};
use arrayvec::ArrayVec;
use lina::{Matrix, Vec2};

advent_of_code::solution!(12);

struct Shape(Grid<bool>);
struct Space(Grid<bool>);

impl Shape {
    fn copy(&self, m: Matrix<i32, 2, 2>) -> Self {
        assert!(self.0.dimension().x == self.0.dimension().y);

        Shape(Grid::new_with_dimensions(self.0.dimension(), |p| {
            self.0[self.rotate_coordinate(p, m)]
        }))
    }

    fn rotate_coordinate(&self, p: Point, m: Matrix<i32, 2, 2>) -> Point {
        // shape DIM is 3,3. map (1,1) -> (0,0)

        let offset = self.0.dimension() / 2 + Vec2::from([1, 1]);
        let displacement = p - offset;
        m.transform(displacement) + offset
    }
}

const UNIQUE_TRANS: usize = 8;

fn get_transformations() -> [Matrix<i32, 2, 2>; UNIQUE_TRANS] {
    let id = Matrix::identity();
    let rot = Matrix::from_rows([[0, -1], [1, 0]]);
    // let rot_neg = - rot_pos;
    let flip = Matrix::from_rows([[1, 0], [0, -1]]);

    let rot_2 = rot * rot;
    let rot_3 = rot * rot_2;
    [
        id,
        rot,
        rot_2,
        rot_3,
        flip,
        flip * rot,
        flip * rot_2,
        flip * rot_3,
    ]
}

impl FromStr for Shape {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (idx, rest) = s.split_once(":\n").ok_or("Shape contained no :\\n")?;

        Ok(Shape(Grid::read(rest, |c| c == '#')))
    }
}

struct GoalSpace {
    dimension: Vec2<i32>,
    requirements: Vec<usize>,
}

impl FromStr for GoalSpace {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, rest) = s.split_once("x").ok_or("no dimension x")?;
        let (y, rest) = rest.split_once(": ").ok_or("no rest ': '")?;

        let x = x.parse().unwrap();
        let y = y.parse().unwrap();

        let nums = rest.split(" ").map(|x| x.parse().unwrap()).collect();

        let out = GoalSpace {
            dimension: Vec2::new(x, y),
            requirements: nums,
        };
        Ok(out)
    }
}

struct Input {
    shapes: Vec<Shape>,
    spaces: Vec<GoalSpace>,
}

impl Input {
    fn new(shapes: Vec<Shape>, spaces: Vec<GoalSpace>) -> Self {
        Self { shapes, spaces }
    }
}

impl FromStr for Input {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut shapes = vec![];
        let mut spaces = vec![];

        for block in s.split("\n\n") {
            if block.split("\n").next().unwrap().contains(":") {
                shapes.push(block.parse()?);
            } else {
                for line in block.split("\n") {
                    spaces.push(line.parse()?);
                }
            }
        }
        Ok(Input { spaces, shapes })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
