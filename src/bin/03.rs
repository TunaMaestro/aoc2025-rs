advent_of_code::solution!(3);

pub fn parse(input: &str) -> Vec<Vec<u64>> {
    input
        .trim()
        .split("\n")
        .map(|line| line.as_bytes().iter().map(|x| (x - b'0').into()).collect())
        .collect()
}

fn swap(
    mut max: u64,
    mut max_idx: usize,
    mut max2: u64,
    mut max2_idx: usize,
) -> (u64, usize, u64, usize) {
    if (max2 > max) {
        (max, max2) = (max2, max);
        (max_idx, max2_idx) = (max2_idx, max_idx);
    }
    (max, max_idx, max2, max2_idx)
}

fn argmax2(xs: &[u64]) -> (usize, usize) {
    let mut max = xs[0];
    let mut max_idx = 0;
    let mut max2 = xs[1];
    let mut max2_idx = 1;

    (max, max_idx, max2, max2_idx) = swap(max, max_idx, max2, max2_idx);

    for (i, x) in xs.iter().enumerate().skip(2) {
        (max2, max2_idx, _, _) = swap(max2, max2_idx, *x, i);
        (max, max_idx, max2, max2_idx) = swap(max, max_idx, max2, max2_idx);
    }
    (max_idx, max2_idx)
}

fn argmax(xs: &[u64]) -> usize {
    let mut max = xs[0];
    let mut max_idx = 0;
    for (i, x) in xs.iter().enumerate().skip(2) {
        if (*x > max) {
            max = *x;
            max_idx = i;
        }
    }
    max_idx
}

fn bank_joltage(batteries: &[u64]) -> u64 {
    // let (max_idx, max2_idx) = argmax2(batteries);

    // let mut max = batteries[max_idx];

    // let mut max2 = batteries[max_idx];

    // if max_idx == batteries.len() - 1 {
    //     (max, max2) = (max2, max);
    // }
    // (max * 10 + max) as u64

    let max_idx = argmax(batteries);
    let max = batteries[max_idx];

    (if (max_idx < batteries.len() - 1) {
        let max2 = *batteries[max_idx + 1..].iter().max().unwrap();
        10 * max + max2
    } else {
        let max2 = *batteries[..max_idx].iter().max().unwrap();
        10 * max2 + max
    }) as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(parse(input).iter().map(|bank| bank_joltage(bank)).sum())
}

const SELECT_DIGITS: usize = 12;

/// let P(i, j) be the largest joltage found by using j of batteries 0..i
fn bank_joltage_large(batteries: &[u64]) -> u64 {
    let mut dp: Vec<[u64; SELECT_DIGITS+1]> =
        Vec::from_iter(std::iter::repeat_n([0; SELECT_DIGITS+1], batteries.len()));
    for i in 0..batteries.len() {
        for j in 1..=SELECT_DIGITS {
            dp[i][j] = p(i, j, batteries, &mut dp);
        }
    }
    dp.iter().map(|x| x[SELECT_DIGITS - 1]).max().unwrap()
}

fn p(i: usize, j: usize, batteries: &[u64], dp: &mut Vec<[u64; SELECT_DIGITS+1]>) -> u64 {
    // the jth battery is at position i.
    let this_bat = batteries[i];
    if j == 1 {
        return this_bat;
    }
    if (j == 0) {
        return 0;
    }
    if (j > i) {
        return 0;
    }

    u64::max(
        p(i - 1, j, batteries, dp),
        this_bat + 10 * p(i - 1, j - 1, batteries, dp),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .iter()
            .map(|bank| bank_joltage_large(bank))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_argmax() {
        assert_eq!(argmax2(&[4, 1, 2, 6, 8, 3, 1, 5]), (4, 3));
    }
    #[test]
    fn test_joltage() {
        assert_eq!(bank_joltage(&[4, 1, 2, 6, 8, 3, 1, 5]), 85);
        assert_eq!(bank_joltage(&[4, 1, 2, 6, 5, 3, 1, 8]), 68);
    }
    #[test]
    fn test_joltage_large() {
        assert_eq!(
            bank_joltage_large(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            111111111111
        );
        assert_eq!(
            bank_joltage_large(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1,]),
            987654321111
        );
        assert_eq!(
            bank_joltage_large(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9,]),
            811111111119
        );
        assert_eq!(
            bank_joltage_large(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8,]),
            434234234278
        );
        assert_eq!(
            bank_joltage_large(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1,]),
            888911112111
        );
    }
}
