use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use arrayvec::ArrayVec;
use itertools::Itertools;

use nalgebra::{DMatrix as Matrix, DVector};
use num::{Signed, integer::lcm};

advent_of_code::solution!(10);

fn parse(input: &str) -> Vec<Machine> {
    input
        .split("\n")
        .filter(|x| x.len() != 0)
        .flat_map(|line| parse_machine(line))
        .collect()
}

fn parse_machine(input: &str) -> Option<Machine> {
    let mut iter = input.split(" ");

    let indicator_part = iter.next()?;
    let indicator_slice = &indicator_part[1..indicator_part.len() - 1];

    let goal = parse_indicator(indicator_slice);

    let mut buttons = ArrayVec::new();
    let mut joltage = ArrayVec::new();
    for part in iter {
        let slice = &part[1..part.len() - 1];
        // eprintln!("{part} {slice}");
        if part.as_bytes()[0] != b'(' {
            joltage = parse_joltage(slice);
            break;
        }
        buttons.push(parse_button(slice));
    }

    Some(Machine {
        goal_size: indicator_slice.len(),
        goal,
        buttons,
        joltage,
    })
}

fn parse_indicator(lights: &str) -> usize {
    let mut required = 0;
    for &c in lights.as_bytes().iter().rev() {
        required <<= 1;
        required |= ((c == b'#') as usize);
    }
    required
}

fn parse_button(button: &str) -> usize {
    let mut out = 0;
    for light in button.split(",") {
        out |= 1 << (light.parse::<u32>().expect("All buttons must be integers"));
    }
    out
}

fn parse_joltage(joltage: &str) -> ArrayVec<usize, 10> {
    joltage
        .split(",")
        .map(|x| x.parse().expect("All joltages must be integers"))
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let machines = parse(input);

    Some(machines.iter().map(|x| x.optimise_steps()).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let machines = parse(input);
    machines
        .iter()
        .for_each((|x| eprintln!("{} {}", x.buttons.len(), x.joltage.len())));
    todo!();
}

#[derive(Debug)]
struct Machine {
    goal_size: usize,
    goal: usize,
    buttons: ArrayVec<usize, 20>,
    joltage: ArrayVec<usize, 10>,
}

struct Equation {
    a: Matrix<f64>,
    rows: usize,
    // m: usize, // rows
    // n: usize, // cols
}

const C: usize = 11;
const R: usize = 12;

trait MyMatrix {
    fn swap_rows(&mut self, row_a: usize, row_b: usize);
}

impl<T: Debug + Copy> MyMatrix for Matrix<T> {
    fn swap_rows(&mut self, row_a: usize, row_b: usize) {
        for i in 0..C {
            let a = *self.index((row_a, i));
            let b = *self.index((row_b, i));

            self[(row_a, i)] = b;
            self[(row_b, i)] = a;
        }
    }
}

impl Equation {
    fn from_machine(machine: &Machine) -> Self {
        // a column corresponds to a variable; a the number of times button i is pressed
        let ncols = machine.buttons.len();
        // a row corresponds to one joltage output value
        let nrows = machine.joltage.len();

        let mut matrix = Matrix::zeros(nrows, ncols);
        for (i, &button) in machine.buttons.iter().enumerate() {
            for j in 0..machine.joltage.len() {
                let mask = 1 << j;
                if mask & button != 0 {
                    matrix[(j, i)] = 1.0;
                }
            }
        }

        let y = machine.joltage.iter().map(|&x| x as isize).collect();
        let y = DVector::from_vec(y);

        assert!(matrix.rank(0.01) == matrix.nrows());

        Equation {
            a: matrix,
            rows: machine.buttons.len(),
        }
    }

    fn row_echelon_form(&mut self) {
        // gaussian elimination
        let a = &mut self.a;

        let mut h = 0; // Initialization of the pivot row
        let mut k = 0; // Initialization of the pivot column

        while h < a.nrows() && k < a.ncols() {
            let col = a.column(k);
            // let i_max = argmax(&col.as_slice()[h..]) + h;
            let i_max = col.as_slice()[h..]
                .iter()
                .enumerate()
                .filter(|x| *x.1 > 0.0)
                .map(|x| x.0)
                .next()
                .unwrap_or(0)
                + h;

            if a[(i_max, k)] == 0.0 {
                k += 1;
                continue;
            }

            dbg!(h, i_max);
            a.swap_rows(h, i_max);

            for i in h + 1..a.nrows() {
                let leading_of_row = a[(i, k)];
                let leading_of_pivot = a[(h, k)];

                a[(i, k)] = 0.0;

                if leading_of_row == 0.0 {
                    continue;
                }
                // let lcm = lcm(leading_of_row, leading_of_pivot);
                // let factor_pivot = lcm / leading_of_pivot;
                // let factor_row = lcm / leading_of_row;

                let f = leading_of_row / leading_of_pivot;

                for j in k + 1..self.rows {
                    // let new = factor_row * a[(i, j)] - factor_pivot * a[(h, j)];
                    let new = a[(i, j)] - f * a[(h, j)];
                    a[(i, j)] = new;
                }
            }
            h += 1;
            k += 1;
        }
    }
}

fn argmax<T: Ord>(v: &[T]) -> usize {
    v.iter()
        .enumerate()
        .max_by_key(|x| x.1)
        .iter()
        .next()
        .map(|x| x.0)
        .unwrap_or(0)
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut indicator = String::new();
        for i in 0..self.goal_size {
            let m = 1 << i;
            let c = if self.goal & m != 0 { '#' } else { '.' };
            indicator.push(c);
        }
        let mut button_str = vec![];
        for &button in &self.buttons {
            let mut lights = vec![];
            for i in 0..self.goal_size {
                let m = 1 << i;
                if m & button != 0 {
                    lights.push(i.to_string());
                }
            }
            button_str.push(format!("({})", lights.join(",")));
        }
        let button_str = button_str.join(" ");
        let joltage_str = &self.joltage.iter().map(|x| x.to_string()).join(",");
        write!(f, "[{indicator}] {button_str} {{{joltage_str}}}")
    }
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_machine(s).ok_or(())
    }
}

impl Machine {
    fn optimise_steps(&self) -> usize {
        let mut best = usize::MAX;
        let combinations = 1 << self.buttons.len();
        for setting in 0..combinations {
            let mut option = 0;
            let mut number_set = 0;
            for (i, &button) in self.buttons.iter().enumerate() {
                let m = 1 << i;
                if m & setting == 0 {
                    continue;
                }
                option ^= button;
                number_set += 1;
            }

            if option == self.goal && number_set < best {
                best = number_set;
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_machine() {
        let m: Machine = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
            .parse()
            .unwrap();

        assert_eq!(m.optimise_steps(), 2);
    }

    #[test]
    fn test_equation() {
        let m: Machine = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
            .parse()
            .unwrap();
        let mut e = Equation::from_machine(&m);
        dbg!(m);

        eprintln!("{}", e.a);
        e.row_echelon_form();
        eprintln!("{}", e.a);

        let qr = e.a.qr();
        eprintln!("QR:{}{}", qr.q(), qr.r());

    }
}
