use std::{
    cmp::{self, Ordering},
    collections::HashMap,
};

use anyhow::{Context, Result};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

fn part_one() -> Result<usize> {
    let map = include_str!("../data/day5_1.txt")
        .lines()
        .flat_map(|line| {
            let mut iter = line.split(" -> ");
            let mut fst = iter.next()?.split(',');
            let mut snd = iter.next()?.split(',');
            let x = fst.next()?.parse().unwrap();
            let y = fst.next()?.parse().unwrap();
            let xx = snd.next()?.parse().unwrap();
            let yy = snd.next()?.parse().unwrap();

            Some((Pos { x, y }, Pos { x: xx, y: yy }))
        })
        .filter(|(a, b)| a.x == b.x || a.y == b.y)
        .fold(HashMap::new(), |mut map, (a, b)| {
            for x in cmp::min(a.x, b.x)..=cmp::max(a.x, b.x) {
                for y in cmp::min(a.y, b.y)..=cmp::max(a.y, b.y) {
                    *map.entry(Pos { x, y }).or_insert(0) += 1;
                }
            }
            map
        });

    Ok(map.values().filter(|count| **count > 1).count())
}

fn part_two() -> Result<usize> {
    let map = include_str!("../data/day5_1.txt")
        .lines()
        .flat_map(|line| {
            let mut iter = line.split(" -> ");
            let mut fst = iter.next()?.split(',');
            let mut snd = iter.next()?.split(',');
            let x = fst.next()?.parse().unwrap();
            let y = fst.next()?.parse().unwrap();
            let xx = snd.next()?.parse().unwrap();
            let yy = snd.next()?.parse().unwrap();

            Some((Pos { x, y }, Pos { x: xx, y: yy }))
        })
        .fold(HashMap::new(), |mut map, (a, b)| {
            for (x, y) in build_range(a.x, b.x).zip(build_range(a.y, b.y)) {
                *map.entry(Pos { x, y }).or_insert(0) += 1;
            }
            map
        });

    Ok(map.values().filter(|count| **count > 1).count())
}

fn build_range(start: i64, end: i64) -> Box<dyn Iterator<Item = i64>> {
    match start.cmp(&end) {
        Ordering::Equal => Box::new(std::iter::repeat(start)),
        Ordering::Greater => Box::new((end..=start).rev()),
        Ordering::Less => Box::new(start..=end),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_1() {
        let count = part_one().unwrap();
        println!("{}", count);
        assert_eq!(count, 6666);
    }

    #[test]
    fn day5_2() {
        let count = part_two().unwrap();
        println!("{}", count);
    }
}
