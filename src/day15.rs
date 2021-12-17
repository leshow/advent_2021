use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

const DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
struct El {
    cost: usize,
    pos: (usize, usize),
}

impl PartialOrd for El {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for El {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

pub fn part_one(grid: Vec<Vec<u8>>) -> usize {
    let mut cost_map = HashMap::new();
    let mut visited = BinaryHeap::new();
    visited.push(Reverse(El::default()));

    while let Some(Reverse(node)) = visited.pop() {
        for (dx, dy) in DIRS {
            let x = (node.pos.0 as isize + dx) as usize;
            let y = (node.pos.1 as isize + dy) as usize;
            if let Some(new_cost) = grid.get(y).and_then(|row| row.get(x)) {
                let cost = node.cost + *new_cost as usize;
                match cost_map.get(&(x, y)) {
                    Some(c) if *c > cost => {
                        cost_map.insert((x, y), cost);
                        visited.push(Reverse(El { cost, pos: (x, y) }));
                    }
                    None => {
                        cost_map.insert((x, y), cost);
                        visited.push(Reverse(El { cost, pos: (x, y) }));
                    }
                    _ => {}
                }
            }
        }
    }
    *cost_map.get(&(grid.len() - 1, grid[0].len() - 1)).unwrap()
}

pub fn part_two(grid: Vec<Vec<u8>>) -> usize {
    let mut cost_map = HashMap::new();
    let mut visited = BinaryHeap::new();
    visited.push(Reverse(El::default()));

    let rows = 5 * grid[0].len();
    let cols = 5 * grid.len();

    while let Some(Reverse(node)) = visited.pop() {
        for (dx, dy) in DIRS {
            let x = node.pos.0 as isize + dx;
            let y = node.pos.1 as isize + dy;
            if x >= 0 && y >= 0 && x < rows as isize && y < cols as isize {
                let x = x as usize;
                let y = y as usize;
                let cost = node.cost + new_cost(&grid, x, y);
                match cost_map.get(&(x, y)) {
                    Some(c) if *c > cost => {
                        cost_map.insert((x, y), cost);
                        visited.push(Reverse(El { cost, pos: (x, y) }));
                    }
                    None => {
                        cost_map.insert((x, y), cost);
                        visited.push(Reverse(El { cost, pos: (x, y) }));
                    }
                    _ => {}
                }
            }
        }
    }
    *cost_map.get(&(cols - 1, rows - 1)).unwrap()
}

fn new_cost(grid: &[Vec<u8>], x: usize, y: usize) -> usize {
    let cols = grid.len();
    let rows = grid[0].len();
    let cost = grid[y % cols][x % rows] as usize;
    let adjustment = y / cols + x / rows;
    // cost wraps at 10
    ((cost + adjustment) % 10) + ((cost + adjustment) / 10)
}

pub fn input(s: &str) -> Vec<Vec<u8>> {
    s.trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day15_1_test() {
        assert_eq!(
            40,
            dbg!(part_one(input(include_str!("../data/day15_test.txt"))))
        );
    }
    #[test]
    fn day15_1() {
        dbg!(part_one(input(include_str!("../data/day15.txt"))));
    }
    #[test]
    fn day15_2() {
        dbg!(part_two(input(include_str!("../data/day15.txt"))));
    }
}
