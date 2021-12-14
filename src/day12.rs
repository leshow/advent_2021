use std::collections::HashMap;

fn part_one(s: HashMap<&str, Vec<&str>>) -> u64 {
    todo!()
}

fn input(s: &str) -> HashMap<&str, Vec<&str>> {
    s.lines().map(|line| line.split_once('_').unwrap()).fold(
        HashMap::new(),
        |mut map, (fst, snd)| {
            map.entry(fst).or_insert_with(Vec::new).push(snd);
            map
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day12_1_test() {
        assert_eq!(
            226,
            dbg!(part_one(input(include_str!("../data/day12.txt"))))
        );
    }
}
