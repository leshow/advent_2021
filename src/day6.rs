fn input(s: &'static str) -> [u64; 9] {
    let mut pop = [0; 9];
    // not sure why I can't remove the trailing \n but whatever
    for n in s.trim_end().split(',').map(|n| n.parse::<usize>().unwrap()) {
        pop[n] += 1;
    }
    pop
}

fn part_one(mut fish: [u64; 9], days: usize) -> u64 {
    for _ in 0..days {
        fish.rotate_left(1);
        fish[6] += fish[8];
    }
    fish.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6_1_test() {
        assert_eq!(5_934, dbg!(part_one(input("3,4,3,1,2"), 80)));
    }

    #[test]
    fn day6_1() {
        assert_eq!(
            374_927,
            dbg!(part_one(input(include_str!("../data/day6_1.txt")), 80))
        );
    }

    #[test]
    fn day6_2() {
        assert_eq!(
            1_687_617_803_407,
            dbg!(part_one(input(include_str!("../data/day6_1.txt")), 256))
        );
    }
}
