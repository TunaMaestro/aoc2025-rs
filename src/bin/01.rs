advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let res = input
        .trim()
        .split("\n")
        .map(|x| {
            let sign = if x.bytes().nth(0).expect("Line has byte") == b'L' {
                -1
            } else {
                1
            };
            return sign * x[1..].parse::<isize>().unwrap();
        })
        .scan(50, |acc, x| {
            let new = (*acc + x).rem_euclid(100);
            *acc = new;
            Some(new)
        })
        .filter(|&x| x == 0)
        .count();
    Some(res as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    const RING: isize = 100;
    let nums = input.trim().split("\n").map(|x| {
        let sign = if x.bytes().nth(0).expect("Line has byte") == b'L' {
            -1
        } else {
            1
        };
        return sign * x[1..].parse::<isize>().unwrap();
    });

    let mut pos = 50;
    let mut zeros = 0;

    for n in nums {
        zeros += n.abs() / RING;
        let partial_turn = n % RING;
        let mut turn_pos = pos;
        let unbounded_change = pos + partial_turn;
        let next_pos = unbounded_change.rem_euclid(RING);

        let turn_pos = pos + partial_turn;
        if ((turn_pos >= RING || turn_pos <= 0) && pos != 0) {
            zeros += 1;
        }
        pos = next_pos;
        // dbg!(pos, zeros);
    }

    Some(zeros as u64)
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_1000() {
        let result = part_two(&"R1000");
        assert_eq!(result, Some(10));
    }
}
