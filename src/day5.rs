use std::cell::Cell;
use std::num::NonZeroU8;

const DIFF: u8 = b'a' - b'A';

#[inline]
fn diff(a: u8, b: u8) -> u8 {
    u8::max(a, b) - u8::min(a, b)
}

#[aoc(day5, part1)]
fn part1(input: &str) -> usize {
    reduce(input.trim().as_bytes())
}

#[aoc(day5, part2)]
fn part2(input: &str) -> Option<usize> {
    let input = input.trim().as_bytes();
    (b'A'..=b'Z')
        .map(|c| reduce(input.iter().filter(|&&a| a != c && a != c + DIFF)))
        .min()
}

#[aoc(day5, part1, Stack)]
fn part1_stack(input: &str) -> usize {
    stack(input.trim().as_bytes())
}

#[aoc(day5, part2, Stack)]
fn part2_stack(input: &str) -> Option<usize> {
    let input = input.trim().as_bytes();

    (b'A'..=b'Z')
        .map(|c| stack(input.iter().filter(|&&a| a != c && a != c + DIFF)))
        .min()
}

fn reduce<'a>(polymer: impl IntoIterator<Item = &'a u8>) -> usize {
    let polymer: Vec<_> = polymer
        .into_iter()
        .map(|&a| Cell::new(NonZeroU8::new(a)))
        .collect();

    let mut i = 0;
    loop {
        if i + 1 >= polymer.len() {
            break;
        }

        let a = &polymer[i];
        let b = if let Some(b) = polymer[i + 1..].iter().find(|b| b.get().is_some()) {
            b
        } else {
            break;
        };

        match (a.get(), b.get()) {
            (Some(x), Some(y)) => if diff(x.get(), y.get()) == DIFF {
                a.set(None);
                b.set(None);

                i = polymer[..i]
                    .iter()
                    .enumerate()
                    .rev()
                    .find_map(|(i, a)| if a.get().is_some() { Some(i) } else { None })
                    .unwrap_or_else(|| {
                        polymer
                            .iter()
                            .enumerate()
                            .find_map(|(i, a)| if a.get().is_some() { Some(i) } else { None })
                            .unwrap()
                    })
            } else {
                i = polymer
                    .iter()
                    .enumerate()
                    .skip(i + 1)
                    .find_map(|(i, a)| if a.get().is_some() { Some(i) } else { None })
                    .unwrap();
            },
            _ => panic!("{}\n{:#?}", i, polymer),
        }
    }

    polymer.into_iter().filter(|a| a.get().is_some()).count()
}

fn stack<'a>(polymer: impl IntoIterator<Item = &'a u8>) -> usize {
    polymer.into_iter().fold(Vec::new(), |mut stack, &unit| {
        match stack.last() {
            Some(&other) if diff(other, unit) == DIFF => { stack.pop(); },
            _ => stack.push(unit),
        }

        stack
    }).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(part1("dabAcCaCBAcCcaDA"), 10);
        assert_eq!(part1_stack("dabAcCaCBAcCcaDA"), 10);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2("dabAcCaCBAcCcaDA"), Some(4));
        assert_eq!(part2_stack("dabAcCaCBAcCcaDA"), Some(4));
    }
}
