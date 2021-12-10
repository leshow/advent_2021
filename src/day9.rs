use std::collections::{HashSet, VecDeque};

const MOVES: [(isize, isize); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

fn part_one(lines: Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;
    for r in 0..lines.len() {
        for c in 0..lines[0].len() {
            let cur = lines[r][c];
            if MOVES
                .into_iter()
                .all(|(dr, dc)| match get(&lines, r, c, dr, dc) {
                    Some((n, _, _)) if cur < n => true,
                    Some(_) => false,
                    None => true,
                })
            {
                sum += cur + 1;
            }
        }
    }
    sum
}

fn low_points(lines: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    for r in 0..lines.len() {
        for c in 0..lines[0].len() {
            let cur = lines[r][c];
            if [(0, -1), (-1, 0), (0, 1), (1, 0)]
                .into_iter()
                .all(|(dr, dc)| match get(&lines, r, c, dr, dc) {
                    Some((n, _, _)) if cur < n => true,
                    Some(_) => false,
                    None => true,
                })
            {
                ret.push((r, c));
            }
        }
    }
    ret
}

fn part_two(lines: Vec<Vec<u32>>) -> u32 {
    let mut visited = HashSet::new();
    let low_points = low_points(&lines);
    let mut sizes = low_points
        .into_iter()
        .map(|(r, c)| bfs(r, c, &mut visited, &lines))
        .collect::<Vec<_>>();
    sizes.sort_unstable();
    sizes.iter().rev().take(3).product()
}

fn bfs(r: usize, c: usize, visited: &mut HashSet<(usize, usize)>, lines: &[Vec<u32>]) -> u32 {
    let mut q = VecDeque::from([(r, c)]);
    let mut ret = 0;
    visited.insert((r, c));

    while let Some((r, c)) = q.pop_front() {
        let cur = lines[r][c];
        ret += 1;
        for (dr, dc) in MOVES {
            if let Some((n, rr, cc)) = get(lines, r, c, dr, dc) {
                if !visited.contains(&(rr, cc)) && cur < lines[rr][cc] && lines[rr][cc] < 9 {
                    visited.insert((rr, cc));
                    q.push_back((rr, cc));
                }
            }
        }
    }
    ret
}

fn get(
    lines: &[Vec<u32>],
    r: usize,
    c: usize,
    dr: isize,
    dc: isize,
) -> Option<(u32, usize, usize)> {
    let r = ((r as isize) + dr) as usize;
    let c = ((c as isize) + dc) as usize;
    Some((*lines.get(r)?.get(c)?, r, c))
}

fn input(s: &str) -> Vec<Vec<u32>> {
    s.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day9_1() {
        dbg!(part_one(input(include_str!("../data/day9_1.txt"))));
    }
    #[test]
    fn day9_1_test() {
        assert_eq!(
            15,
            dbg!(part_one(input(include_str!("../data/day9_test.txt"))))
        );
    }
    #[test]
    fn day9_2_test() {
        assert_eq!(
            1134,
            dbg!(part_two(input(include_str!("../data/day9_test.txt"))))
        );
    }
    #[test]
    fn day9_2() {
        dbg!(part_two(input(include_str!("../data/day9_1.txt"))));
    }
}
