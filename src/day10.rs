use std::{char, collections::HashMap};

fn points() -> HashMap<char, u64> {
    let mut map = HashMap::new();
    map.insert(')', 3);
    map.insert(']', 57);
    map.insert('}', 1197);
    map.insert('>', 25137);
    map
}

fn char_map() -> HashMap<char, char> {
    let mut map = HashMap::new();
    map.insert('(', ')');
    map.insert('[', ']');
    map.insert('{', '}');
    map.insert('<', '>');
    map
}

fn part_one(s: &str) -> u64 {
    let char_map = char_map();
    let points = points();
    s.lines()
        .flat_map(|line| {
            let mut brackets = vec![];
            for c in line.chars() {
                match c {
                    '{' | '(' | '<' | '[' => brackets.push(c),
                    c => {
                        let last = brackets.pop()?;
                        let expected = char_map[&last];
                        if c != expected {
                            return Some(points[&c]);
                        }
                    }
                }
            }
            None
        })
        .sum()
}

fn points_two() -> HashMap<char, u64> {
    let mut map = HashMap::new();
    map.insert(')', 1);
    map.insert(']', 2);
    map.insert('}', 3);
    map.insert('>', 4);
    map
}

fn part_two(s: &str) -> u64 {
    let char_map = char_map();
    let points = points_two();
    let mut final_points = s
        .lines()
        .flat_map(|line| {
            let mut brackets = vec![];
            for c in line.chars() {
                match c {
                    '{' | '(' | '<' | '[' => brackets.push(c),
                    c => {
                        let last = brackets.pop()?;
                        let expected = char_map[&last];
                        if c != expected {
                            return None;
                        }
                    }
                }
            }
            Some(
                brackets
                    .iter()
                    .rev()
                    .map(|c| points[&char_map[c]])
                    .fold(0, |prod, a| 5 * prod + a),
            )
        })
        .collect::<Vec<_>>();
    final_points.sort_unstable();
    final_points[final_points.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day10_1_test() {
        assert_eq!(
            26397,
            dbg!(part_one(include_str!("../data/day10_test.txt")))
        );
    }
    #[test]
    fn day10_1() {
        dbg!(part_one(include_str!("../data/day10_1.txt")));
    }
    #[test]
    fn day10_2_test() {
        assert_eq!(
            288957,
            dbg!(part_two(include_str!("../data/day10_test.txt")))
        );
    }
    #[test]
    fn day10_2() {
        dbg!(part_two(include_str!("../data/day10_1.txt")));
    }
}
