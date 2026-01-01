use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    num::Saturating,
    str::FromStr,
};

use arrayvec::ArrayVec;
use bit_iter::BitIter;
use itertools::Itertools;

use microlp::{LinearExpr, OptimizationDirection, Problem};
use nalgebra::{DMatrix as Matrix, DVector};
use num::{Integer, Signed, integer::lcm};

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

fn generate_sequences(max_len: usize) -> Vec<Vec<usize>> {
    (0..=max_len)
        .map(|x| {
            let mut v: Vec<usize> = (0..(1 << x)).collect();
            v.sort_by_key(|f| f.count_ones());
            v
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let machines = parse(input);
    let max_buttons = machines.iter().map(|x| x.buttons.len()).max().unwrap_or(0);
    let ordered_iterations = generate_sequences(max_buttons);
    Some(
        machines
            .iter()
            .map(|x| x.optimise_steps(&ordered_iterations))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let machines = parse(input);
    let max_buttons = machines.iter().map(|x| x.buttons.len()).max().unwrap_or(0);
    let ordered_iterations = generate_sequences(max_buttons);

    let ans = machines
        .iter()
        .map(|x| x.solve_by_halving(&ordered_iterations))
        .sum();

    Some(ans)
}

pub fn part_two_ilp(input: &str) -> Option<usize> {
    let machines = parse(input);
    let sum = machines
        .iter()
        .map(|x| Equation::from_machine(x))
        .map(|x| x.solve())
        .sum();
    Some(sum)
}

type Joltage = ArrayVec<usize, 10>;

#[derive(Debug)]
struct Machine {
    goal_size: usize,
    goal: usize,
    buttons: ArrayVec<usize, 20>,
    joltage: Joltage,
}

struct Equation {
    a: Matrix<i64>,
    y: DVector<i64>,
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
        let ncols = machine.buttons.len();
        let nrows = machine.joltage.len();

        let mut matrix = Matrix::zeros(nrows, ncols);
        for (i, &button) in machine.buttons.iter().enumerate() {
            for j in 0..machine.joltage.len() {
                let mask = 1 << j;
                if mask & button != 0 {
                    matrix[(j, i)] = 1;
                }
            }
        }

        let y = machine.joltage.iter().map(|&x| x as i64).collect();
        let y = DVector::from_vec(y);

        Equation { a: matrix, y }
    }

    fn row_echelon_form(&mut self) {
        let a = &mut self.a;

        let mut h = 0;
        let mut k = 0;

        while h < a.nrows() && k < a.ncols() {
            let col = a.column(k);
            let i_max = col.as_slice()[h..]
                .iter()
                .enumerate()
                .filter(|x| *x.1 > 0)
                .map(|x| x.0)
                .next()
                .unwrap_or(0)
                + h;

            if a[(i_max, k)] == 0 {
                k += 1;
                continue;
            }

            dbg!(h, i_max);
            a.swap_rows(h, i_max);

            for i in h + 1..a.nrows() {
                let leading_of_row = a[(i, k)];
                let leading_of_pivot = a[(h, k)];

                a[(i, k)] = 0;

                if leading_of_row == 0 {
                    continue;
                }

                let f = leading_of_row / leading_of_pivot;

                for j in k + 1..a.nrows() {
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

struct ParitySolution {
    number_steps: usize,
    selection: usize,
}

impl Machine {
    fn optimise_steps(&self, order: &Vec<Vec<usize>>) -> usize {
        for setting in order[self.buttons.len()].iter() {
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

            if option == self.goal {
                return number_set;
            }
        }
        usize::MAX
    }

    fn all_parity_patterns(&self, order: &Vec<Vec<usize>>, goal: usize) -> Vec<usize> {
        let mut settings = vec![];
        for &setting in order[self.buttons.len()].iter() {
            let mut option = 0;
            for (i, &button) in self.buttons.iter().enumerate() {
                let m = 1 << i;
                if m & setting == 0 {
                    continue;
                }
                option ^= button;
            }

            if option == goal {
                settings.push(setting);
            }
        }
        settings
    }

    fn solve_by_halving(&self, order: &Vec<Vec<usize>>) -> usize {
        let mut cache = HashMap::new();
        let mut joltage_cache = HashMap::new();
        self.optimise_joltage(order, &mut cache, &mut joltage_cache, self.joltage.clone())
    }

    fn optimise_joltage(
        &self,
        order: &Vec<Vec<usize>>,
        selection_cache: &mut HashMap<usize, Vec<usize>>,
        joltage_cache: &mut HashMap<Joltage, usize>,
        goal: Joltage,
    ) -> usize {
        // eprintln!(">>      {:?}", &goal);
        if goal.iter().all(|&x| x == 0) {
            //     eprintln!("0");
            return 0;
        }
        if let Some(&x) = joltage_cache.get(&goal) {
            // eprintln!("found {goal:?} -> {x}");
            return x;
        }

        let bin_goal = to_parity(&goal);
        // let selections_to_make_even = &self.all_parity_patterns(order, bin_goal);
        let selections_to_make_even = selection_cache
            .entry(bin_goal)
            .or_insert_with(|| self.all_parity_patterns(order, bin_goal))
            .clone();

        let mut ans = usize::MAX;
        for presses in selections_to_make_even {
            let Some(after) = self.apply_pattern(&goal, presses) else {
                continue;
            };
            let half_goal = after.iter().map(|x| x / 2).collect();
            let cost = Saturating(presses.count_ones() as usize);

            let half_cost =
                Saturating(self.optimise_joltage(order, selection_cache, joltage_cache, half_goal));
            let choice_cost = cost + Saturating(2) * half_cost;
            ans = ans.min(choice_cost.0);
        }

        joltage_cache.insert(goal, ans);
        // eprintln!("<<      {:?}", &ans);
        ans
    }

    fn apply_pattern(&self, goal: &Joltage, pattern: usize) -> Option<Joltage> {
        let mut copy = goal.clone();

        for selected_btn in BitIter::from(pattern) {
            for row in BitIter::from(self.buttons[selected_btn]) {
                if copy[row] == 0 {
                    return None;
                }
                copy[row] -= 1;
            }
        }
        debug_assert!(copy.iter().all(|x| x.is_even()));
        Some(copy)
    }
}

fn to_parity(joltage: &[usize]) -> usize {
    joltage
        .iter()
        .enumerate()
        .map(|(i, &req)| (req & 1) << i)
        .fold(0, |a, b| a | b)
}

impl Equation {
    fn solve_microlp(&self) -> usize {
        let mut problem = Problem::new(microlp::OptimizationDirection::Minimize);

        let mut vars = vec![];
        for i in 0..self.a.ncols() {
            vars.push(problem.add_integer_var(1.0, (0, i32::MAX)));
        }

        for row_idx in 0..self.a.nrows() {
            let mut joltage_linexp = LinearExpr::empty();
            for i in 0..self.a.ncols() {
                if self.a[(row_idx, i)] == 1 {
                    joltage_linexp.add(vars[i], 1.0);
                } else {
                    joltage_linexp.add(vars[i], 0.0);
                }
            }
            problem.add_constraint(
                joltage_linexp,
                microlp::ComparisonOp::Eq,
                self.y[row_idx] as f64,
            );
        }

        let ans = problem
            .solve()
            .expect("could not solve problem")
            .objective();
        ans.round() as usize
    }

    fn solve_linalg(&self) -> usize {
    }
}

#[cfg(not(feature = "cbc"))]
impl Equation {
    fn solve(&self) -> usize {
        self.solve_microlp()
    }
}

#[cfg(feature = "cbc")]
mod cbc {
    use super::*;
    use coin_cbc::{Model, Solution};

    impl Equation {
        fn solve_cbc(&self) -> usize {
            let mut model = Model::default();
            model.set_obj_sense(coin_cbc::Sense::Minimize);

            let mut cols = vec![];
            for _ in 0..self.a.ncols() {
                let col = model.add_integer();
                model.set_obj_coeff(col, 1.0);
                model.set_col_lower(col, 0.0);
                cols.push(col);
            }

            let mut rows = vec![];
            for j in 0..self.a.nrows() {
                let row = model.add_row();
                model.set_row_equal(row, self.y[j] as f64);
                rows.push(row)
            }

            for (i, &row) in rows.iter().enumerate() {
                for (j, &col) in cols.iter().enumerate() {
                    model.set_weight(row, col, self.a[(i, j)] as f64);
                }
            }

            let mut raw = model.to_raw();
            dbg!(raw.solve());
            dbg!(raw.col_solution());

            let ans = 0.0;
            ans as usize
        }

        pub fn solve(&self) -> usize {
            self.solve_cbc()
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_equation_cbc() {
            let m: Machine = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
                .parse()
                .unwrap();
            let e = Equation::from_machine(&m);
            assert_eq!(e.solve_cbc(), 10);
        }
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
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_machine() {
        let m: Machine = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
            .parse()
            .unwrap();

        let ordered = generate_sequences(10);
        assert_eq!(m.optimise_steps(&ordered), 2);
    }

    #[test]
    fn test_equation_microlp() {
        let m: Machine = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
            .parse()
            .unwrap();
        let e = Equation::from_machine(&m);
        assert_eq!(e.solve_microlp(), 10);
    }

    #[test]
    fn test_gen() {
        let a = generate_sequences(3);
        assert_eq!(a[3], vec![0, 1, 2, 4, 3, 5, 6, 7])
    }

    #[test]
    fn test_parity() {
        assert_eq!(to_parity(&[3, 5, 4, 7]), 0b1011);
        assert_eq!(to_parity(&[1, 1, 0, 8, 3, 5, 4, 7]), 0b10110011);
    }

    #[test]
    fn test_consistency() {
        assert_eq!(parse_indicator("...###"), to_parity(&[0, 0, 0, 1, 1, 1]));
        assert_eq!(parse_button("4,5"), 0b110000);
    }

    #[test]
    fn test_part_two_dp() {
        let m: Machine = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
            .parse()
            .unwrap();

        let order = &generate_sequences(6);

        assert_eq!(m.solve_by_halving(order), 10);
    }
}
