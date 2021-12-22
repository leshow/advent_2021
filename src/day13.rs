#![allow(clippy::type_complexity)]

use std::collections::HashSet;

fn part_one((coords, folds): (Vec<(usize, usize)>, Vec<(u8, usize)>)) -> usize {
    let (fold_axis, fold_index) = folds.get(0).cloned().unwrap();

    coords
        .into_iter()
        .filter_map(|(x, y)| match fold_axis {
            b'x' if x == fold_index => None,
            b'x' if x > fold_index => Some((fold_index - (x - fold_index), y)),
            b'y' if y == fold_index => None,
            b'y' if y > fold_index => Some((x, fold_index - (y - fold_index))),
            _ => Some((x, y)),
        })
        .collect::<HashSet<_>>()
        .len()
}

fn input(s: &str) -> (Vec<(usize, usize)>, Vec<(u8, usize)>) {
    let (coords, folds) = s.trim().split_once("\n\n").unwrap();
    let folds = folds
        .lines()
        .map(|line| {
            let (fold_axis, fold_index) = line[11..].split_once('=').unwrap();
            (fold_axis.as_bytes()[0], fold_index.parse().unwrap())
        })
        .collect::<Vec<_>>();

    let coords = coords
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect::<Vec<_>>();
    (coords, folds)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day13_1_test() {
        assert_eq!(17, part_one(input(include_str!("../data/day13_test.txt"))));
    }
    #[test]
    fn day13_1() {
        dbg!(part_one(input(include_str!("../data/day13.txt"))));
    }
}
