fn part_one(lines: Vec<Vec<u32>>) {}

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
}
