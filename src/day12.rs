use std::collections::{HashMap, VecDeque};

const START: &str = "start";
const END: &str = "end";

fn part_one(adj_map: HashMap<&str, Vec<&str>>) -> Option<u64> {
    let mut q = VecDeque::new();
    q.push_back(vec![START]);
    let mut count = 0;

    while let Some(cur_path) = q.pop_front() {
        let last = cur_path.last()?;
        for next in adj_map.get(last)?.iter() {
            if next == &START || was_visited(&cur_path, next)? {
                continue;
            }
            if next == &END {
                count += 1;
                continue;
            }
            let mut new = cur_path.clone();
            new.push(next);
            q.push_back(new);
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
    // these exist just to run the code
    #[test]
    fn day12_1() {
        dbg!(part_one(input(include_str!("../data/day12.txt"))));
    }
}
