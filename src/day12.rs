use std::{
    collections::{HashMap, VecDeque},
    usize,
};

const START: &str = "start";
const END: &str = "end";

fn part_one(adj_map: HashMap<&str, Vec<&str>>) -> Option<u64> {
    let mut q = VecDeque::new();
    q.push_back(vec![START]);
    let mut count = 0;

    while let Some(cur_path) = q.pop_front() {
        let last = cur_path.last()?;
        for next in adj_map.get(last)?.iter() {
            match next {
                &START => {}
                next if was_visited(&cur_path, next)? => {}
                &END => {
                    count += 1;
                }
                _ => {
                    let mut new = cur_path.clone();
                    new.push(next);
                    q.push_back(new);
                }
            }
        }
    }
    Some(count)
}

fn was_visited(path: &[&str], neighbour: &str) -> Option<bool> {
    Some(neighbour.chars().next()?.is_lowercase() && path.contains(&neighbour))
}

fn input(s: &str) -> HashMap<&str, Vec<&str>> {
    s.trim()
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .fold(HashMap::new(), |mut map, (fst, snd)| {
            map.entry(fst).or_insert_with(Vec::new).push(snd);
            map.entry(snd).or_insert_with(Vec::new).push(fst);
            map
        })
}

fn part_two<'a>(mut adj_map: HashMap<&'a str, Vec<&'a str>>) -> Option<u64> {
    let mut q = VecDeque::new();
    q.push_back(vec![START]);
    let mut count = 0;

    while let Some(cur_path) = q.pop_front() {
        let last = cur_path.last()?;
        for next in adj_map.get_mut(last)?.iter_mut() {
            match *next {
                START => {}
                END => {
                    count += 1;
                }
                next if !next.chars().next().unwrap().is_lowercase() => {
                    let mut new = cur_path.clone();
                    new.push(next);
                    q.push_back(new);
                }
                _ => match cur_path.iter().filter(|&x| x == next).count() {
                    0 => {
                        let mut new = cur_path.clone();
                        new.push(next);
                        q.push_back(new);
                    }
                    1 if !has_revisited_small(&cur_path) => {
                        let mut new = cur_path.clone();
                        new.push(next);
                        q.push_back(new);
                    }
                    _ => {}
                },
            }
        }
    }
    Some(count)
}

fn has_revisited_small(cur_path: &[&str]) -> bool {
    cur_path
        .iter()
        .filter(|p| p.chars().next().unwrap().is_lowercase())
        .fold(HashMap::new(), |mut map, small| {
            *map.entry(small).or_insert(0) += 1;
            map
        })
        .into_iter()
        .filter(|&(_, v)| v > 1)
        .count()
        > 0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day12_1_test() {
        assert_eq!(
            Some(10),
            dbg!(part_one(input(include_str!("../data/day12_test.txt"))))
        );
    }

    #[test]
    fn day12_2_test() {
        assert_eq!(
            Some(36),
            dbg!(part_two(input(include_str!("../data/day12_test.txt"))))
        );
    }

    // these exist just to run the code
    #[test]
    fn day12_1() {
        dbg!(part_one(input(include_str!("../data/day12.txt"))));
    }
    #[test]
    fn day12_2() {
        dbg!(part_two(input(include_str!("../data/day12.txt"))));
    }
}
