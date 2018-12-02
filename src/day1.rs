use aoc_runner_derive::{aoc_generator, aoc};

use fnv::FnvHashSet;
use std::collections::HashSet;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| {
            let val: i32 = l[1..].parse().unwrap();

            if &l[0..1] == "-" {
                -val
            } else {
                val
            }
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(freqs: &[i32]) -> i32 {
    freqs.iter().sum()
}

#[aoc(day1, part2)]
fn part2(freqs: &[i32]) -> i32 {
    let mut reached = HashSet::new();
    let mut sum = 0;

    reached.insert(sum);

    for f in freqs.iter().cycle() {
        sum += f;

        if !reached.insert(sum) {
            return sum;
        }
    }

    unreachable!()
}

#[aoc(day1, part2, Fnv)]
fn part2_fnv(freqs: &[i32]) -> i32 {
    let mut reached = FnvHashSet::default();
    let mut sum = 0;

    reached.insert(sum);

    for f in freqs.iter().cycle() {
        sum += f;

        if !reached.insert(sum) {
            return sum;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&[1, -2, 3, 1]), 3);
        assert_eq!(part1(&[1, 1, 1]), 3);
        assert_eq!(part1(&[1, 1, -2]), 0);
        assert_eq!(part1(&[-1, -2, -3]), -6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&[1, -2, 3, 1]), 2);
        assert_eq!(part2(&[1, -1]), 0);
        assert_eq!(part2(&[3, 3, 4, -2, -4]), 10);
        assert_eq!(part2(&[-6, 3, 8, 5, -6]), 5);
        assert_eq!(part2(&[7, 7, -2, -7, -4]), 14);
    }

    #[test]
    fn part2_fnv_example() {
        assert_eq!(part2_fnv(&[1, -2, 3, 1]), 2);
        assert_eq!(part2_fnv(&[1, -1]), 0);
        assert_eq!(part2_fnv(&[3, 3, 4, -2, -4]), 10);
        assert_eq!(part2_fnv(&[-6, 3, 8, 5, -6]), 5);
        assert_eq!(part2_fnv(&[7, 7, -2, -7, -4]), 14);
    }
}
