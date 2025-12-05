use std::{collections::HashSet, process::exit, u64};

advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(",")
        .map(|s| {
            let mut iter = s.split("-");
            let a: u64 = iter.next().unwrap().parse().unwrap();
            let b: u64 = iter.next().unwrap().parse().unwrap();
            (a, b)
        })
        .collect()
}

fn pow10(n: u32) -> u64 {
    10_u64.pow(n)
}

fn sum_range(a: u64, b: u64) -> u64 {
    // eprintln!("{a}\t{b}");
    let a_log = a.ilog10();
    let b_log = b.ilog10();
    if (a_log != b_log) {
        let mut sum = 0;
        // partial from edges
        sum += sum_range(a, pow10(a_log + 1) - 1) + sum_range(pow10(b_log), b);

        // inner perfect ranges i.e. 10-99
        for pow in a_log + 1..=b_log - 1 {
            eprintln!("Had to do inner recursion");
            if pow % 2 == 0 {
                continue;
            }
            let low = pow10(pow);
            let high = low * 10 - 1;
            sum += sum_range(low, high);
        }
        return sum;
    }
    // both must have even digits
    let digits = a_log + 1;
    if digits % 2 != 0 {
        return 0;
    }
    let mask = 10_u64.pow(digits / 2);
    let a_high = a / mask;
    let a_low = a % mask;
    let b_high = b / mask;
    let b_low = b % mask;

    // let big_diff = b_high - a_high;
    // let big_count = big_diff * mask;
    // let chop_top = b_high.saturating_sub(b_low);
    // let chop_bot = a_low.saturating_sub(a_high);

    // return big_count.saturating_sub(chop_top).saturating_sub(chop_bot);

    let double_min = a_low.min(a_high);
    let double_max = b_low.max(b_high);
    let mut sum = 0;
    for i in double_min..=double_max {
        let e = i + (i * mask);
        if (e < a || e > b) {
            // eprintln!("!! {a}-{b} {e}");
            continue;
        }
        sum += e;
    }

    sum
}

fn sum_range_poly(a: u64, b: u64, repeats: u32, tried: &mut HashSet<u64>) -> u64 {
    debug_assert_eq!(a.ilog10(), b.ilog10());
    let digits = a.ilog10() + 1;

    debug_assert_eq!(digits % repeats, 0);

    let low = pow10(digits / repeats - 1);
    // let high = u64::MIN;
    // let mask = 10_u64.pow(digits / repeats);

    // let mut partial_a
    // for (i in 0..repeats) {

    // }

    let mut sum = 0;
    for i in low..=(10 * low) {
        let e = repeat_number(i, repeats);
        if (e < a || e > b) {
            // eprintln!("!! {a}-{b} {e}");
            continue;
        }
        if (tried.contains(&e)) {
            continue;
        }
        tried.insert(e);
        sum += e;
    }

    sum
}

fn repeat_number(mut x: u64, repeats: u32) -> u64 {
    // dbg!(x, repeats);
    let mut sum = x;
    let closest_ten = pow10(x.ilog10() + 1);
    for i in 1..repeats {
        x *= closest_ten;
        sum += x;
    }
    sum
}

fn sum_all(a: u64, b: u64) -> u64 {
    let mut tried: HashSet<u64> = HashSet::new();
    let a_log = a.ilog10();
    let b_log = b.ilog10();
    if (a_log != b_log) {
        return sum_all(a, pow10(a_log + 1) - 1) + sum_all(pow10(b_log), b);
    }
    let digits = a_log + 1;
    let mut sum = 0;
    for repeats in 2..=digits {
        if digits % repeats == 0 {
            sum += sum_range_poly(a, b, repeats, &mut tried);
        }
    }
    return sum;
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(parse(input).iter().map(|&(a, b)| sum_range(a, b)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(parse(input).iter().map(|&(a, b)| sum_all(a, b)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }

    #[test]
    fn test_11_22() {
        assert_eq!(sum_range(11, 22), 33);
    }
    #[test]
    fn test_95_115() {
        assert_eq!(sum_range(95, 115), 99);
    }
    #[test]
    fn test_100s() {
        assert_eq!(
            sum_range(10, 99),
            11 + 22 + 33 + 44 + 55 + 66 + 77 + 88 + 99
        );
    }
    #[test]
    fn test_100s_plus_4_digits() {
        assert_eq!(
            sum_range(10, 1010),
            11 + 22 + 33 + 44 + 55 + 66 + 77 + 88 + 99 + 1010
        );
    }

    #[test]
    fn test_recursion() {
        assert!(sum_range(1, 1000000) > 0);
    }

    #[test]
    fn test_repeat_number() {
        assert_eq!(repeat_number(123, 4), 123123123123);
    }

    #[test]
    fn test_poly() {
        let mut tried = HashSet::new();
        assert_eq!(sum_range_poly(99, 99, 2, &mut tried), 99);
        assert_eq!(sum_range_poly(100, 115, 3, &mut tried), 111);
    }
}
