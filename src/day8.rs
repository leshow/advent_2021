use std::collections::HashSet;

type Signals = [u8; 10];
type Digits = [u8; 4];

fn input(s: &'static str) -> Vec<(Signals, Digits)> {
    s.trim()
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(" | ").unwrap();
            let mut signals = [0u8; 10];
            for (i, block) in l.split_whitespace().enumerate() {
                signals[i] = count(block);
            }
            signals.sort_by(|a, b| a.count_ones().cmp(&b.count_ones()));
            let mut digits = [0u8; 4];
            for (i, block) in r.split_whitespace().enumerate() {
                digits[i] = count(block);
            }
            (signals, digits)
        })
        .collect()
}

fn count(block: &str) -> u8 {
    block
        .chars()
        .fold(0, |acc, c| acc | (1 << (c as u32 - 'a' as u32)))
}

fn part_one(nums: Vec<(Signals, Digits)>) -> u32 {
    nums.iter()
        .map(|n| {
            n.1.iter()
                .map(|d| match d.count_ones() {
                    // num bars in 1/4/7/8
                    2 | 3 | 4 | 7 => 1,
                    _ => 0,
                })
                .sum::<u32>()
        })
        .sum()
}

// 0 -> 6           x
// 1 -> 2 <- unique x
// 2 -> 5           x
// 3 -> 5           x
// 4 -> 4 <- unique x
// 5 -> 5           x
// 6 -> 6           x
// 7 -> 3 <- unique x
// 8 -> 7 <- unique x
// 9 -> 6           x
// gave up on integer representations, although I'm sure there is a way

fn input_two(input: &str) -> Vec<(Vec<&str>, Vec<HashSet<char>>)> {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (l, r) = line.split_once(" | ").unwrap();
            (
                l.split_whitespace().collect::<Vec<_>>(),
                r.split_whitespace()
                    .map(|s| s.chars().collect::<HashSet<_>>())
                    .collect(),
            )
        })
        .collect()
}

// this one sucked -- took some "inspiration"
fn part_two(lines: Vec<(Vec<&str>, Vec<HashSet<char>>)>) -> u32 {
    let mut sum = 0;

    for (mut words, output) in lines {
        let mut map = [""; 10];
        // get out unique values
        words.retain(|word| match word.len() {
            2 => {
                map[1] = word;
                false
            }
            3 => {
                map[7] = word;
                false
            }
            4 => {
                map[4] = word;
                false
            }
            7 => {
                map[8] = word;
                false
            }
            _ => true,
        });

        // 9 contains 7 & 4
        words.retain(|word| {
            if word.len() == 6
                && map[4].chars().all(|seg| word.contains(seg))
                && map[7].chars().all(|seg| word.contains(seg))
            {
                map[9] = word;
                false
            } else {
                true
            }
        });
        // 3 contains 1
        words.retain(|word| {
            if word.len() == 5 && map[1].chars().all(|seg| word.contains(seg)) {
                map[3] = word;
                false
            } else {
                true
            }
        });

        // 2 not 9
        words.retain(|word| {
            if word.len() == 5 && !word.chars().all(|seg| map[9].contains(seg)) {
                map[2] = word;
                false
            } else {
                true
            }
        });

        // 5 len 5 but not 2 or 3
        words.retain(|word| {
            if word.len() == 5 {
                map[5] = word;
                false
            } else {
                true
            }
        });

        // 6 contains 5 and not 9
        words.retain(|word| {
            if word.len() == 6 && map[5].chars().all(|seg| word.contains(seg)) && word != &map[9] {
                map[6] = word;
                false
            } else {
                true
            }
        });

        map[0] = words[0];

        let n = output
            .iter()
            .map(|set| {
                let (i, _) = map
                    .iter()
                    .enumerate()
                    .find(|&(_, b)| {
                        let map_set = b.chars().collect::<HashSet<_>>();
                        set == &map_set
                    })
                    .unwrap();
                i
            })
            .fold(0, |acc, x| acc * 10 + x as u32);

        sum += n;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day8_1_test() {
        assert_eq!(
            26,
            dbg!(part_one(input(include_str!("../data/day8_test.txt"))))
        );
    }

    #[test]
    fn day8_1() {
        assert_eq!(470, dbg!(part_one(input(include_str!("../data/day8.txt")))));
    }

    #[test]
    fn day8_2() {
        assert_eq!(
            989396,
            dbg!(part_two(input_two(include_str!("../data/day8.txt"))))
        );
    }

    #[test]
    fn day8_2_test() {
        assert_eq!(
            5353,
            dbg!(part_two(input_two(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
        )))
        );
    }
}
