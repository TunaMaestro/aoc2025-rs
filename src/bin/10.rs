use std::{fmt::Display, str::FromStr};

use itertools::Itertools;

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

    let mut buttons = vec![];
    let mut joltage = vec![];
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

fn parse_joltage(joltage: &str) -> Vec<usize> {
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
    None
}

#[derive(Debug)]
struct Machine {
    goal_size: usize,
    goal: usize,
    buttons: Vec<usize>,
    joltage: Vec<usize>,
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
}
