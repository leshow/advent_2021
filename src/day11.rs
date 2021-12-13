const SIZE: usize = 10;
const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
];
type Map = [[u8; SIZE]; SIZE];

fn part_one(mut map: Map) -> u64 {
    (0..100).map(|_| step(&mut map)).sum()
}

fn step(map: &mut Map) -> u64 {
    for row in map.iter_mut() {
        for oct in row.iter_mut() {
            *oct += 1;
        }
    }
    (0..SIZE)
        .flat_map(|y| (0..SIZE).map(move |x| (x, y)))
        .filter_map(|(x, y)| (map[y][x] > 9).then(|| do_flash(map, x, y)))
        .sum()
}

fn do_flash(map: &mut Map, x: usize, y: usize) -> u64 {
    // reset to 0
    map[y][x] = 0;
    // add 1 for each roll over recursively
    1 + DIRS
        .iter()
        .map(|(xx, yy)| ((x as isize + xx) as usize, (y as isize + yy) as usize))
        .filter_map(|(x, y)| {
            let oct = map.get_mut(y)?.get_mut(x)?;
            (*oct > 0)
                .then(|| *oct += 1)
                .and((*oct > 9).then(|| do_flash(map, x, y)))
        })
        .sum::<u64>()
}

fn input(s: &str) -> Map {
    s.trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn part_two(mut map: Map) -> u64 {
    (1..)
        .find(|_| step(&mut map) == SIZE as u64 * SIZE as u64)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day11_1_test() {
        assert_eq!(
            1656,
            dbg!(part_one(input(include_str!("../data/day11_test.txt"))))
        );
    }
    #[test]
    fn day11_1() {
        dbg!(part_one(input(include_str!("../data/day11.txt"))));
    }
    #[test]
    fn day11_2_test() {
        assert_eq!(
            195,
            dbg!(part_two(input(include_str!("../data/day11_test.txt"))))
        );
    }
    #[test]
    fn day11_2() {
        dbg!(part_two(input(include_str!("../data/day11.txt"))));
    }
}
