fn input(s: &'static str) -> Vec<i64> {
    s.trim().split(',').map(|n| n.parse().unwrap()).collect()
}

// "the median minimizes the sum of absolute deviations"
// I'll be honest, I asked a mathematically inclined friend
fn part_one(mut input: Vec<i64>) -> i64 {
    input.sort_unstable();
    let mid = input[input.len() / 2];
    input.iter().map(|x| (x - mid).abs()).sum()
}

// but I did remember: n*(n+1) / 2 is the sum of 1+2+3+...n
fn part_two(input: Vec<i64>) -> i64 {
    let avg = input.iter().sum::<i64>() as f64 / input.len() as f64;
    std::cmp::min(
        input
            .iter()
            .map(|x| {
                let n = (x - avg.floor() as i64).abs();
                n * (n + 1) / 2
            })
            .sum(),
        input
            .iter()
            .map(|x| {
                let n = (x - avg.ceil() as i64).abs();
                n * (n + 1) / 2
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    // these are just for convenience to print the output
    #[test]
    fn day7_1() {
        dbg!(part_one(input(include_str!("../data/day7_1.txt"))));
    }
    #[test]
    fn day7_2() {
        dbg!(part_two(input(include_str!("../data/day7_1.txt"))));
    }
    // samples
    #[test]
    fn day7_1_sample() {
        assert_eq!(37, part_one(input("16,1,2,0,4,2,7,1,2,14")));
    }
    #[test]
    fn day7_2_sample() {
        assert_eq!(168, part_two(input("16,1,2,0,4,2,7,1,2,14")));
    }
}
