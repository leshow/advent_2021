use anyhow::{Context, Result};

use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

pub fn part_one() -> Result<()> {
    let lines = BufReader::new(File::open("./data/day3_1.txt")?)
        .lines()
        .flatten()
        // could use bit twiddles and leave each line as a u16, but I'm lazy
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();

    let len = lines
        .get(0)
        .context("didnt parse any lines-- check input")?
        .len();

    // store gamma, we can bitwise not to get epsilon
    let mut gamma: u32 = 0;

    for col in 0..len {
        let zeroes = lines.iter().filter(|line| line[col] == 0).count();
        let ones = lines.len() - zeroes;
        let mcb = if zeroes >= ones { 0 } else { 1 };
        gamma = (gamma << 1) | mcb
    }
    // shift off the 1's that will be left over after NOT-ing
    // gamma
    let shift_len = u32::BITS - len as u32;
    let epsilon = !gamma << shift_len >> shift_len;
    println!("{:b} {:b}", gamma, epsilon);
    println!(
        "gamma {:?} epsilon {:?} {:?}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    Ok(())
}

pub fn part_two() -> Result<()> {
    let lines = BufReader::new(File::open("./data/day3_1.txt")?)
        .lines()
        .flatten()
        // could use bit twiddles and leave each line as a u16, but I'm lazy
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();

    let len = lines
        .get(0)
        .context("didnt parse any lines-- check input")?
        .len();

    let ox = filter(lines.clone(), len, true).context("failed to get a value for ox")?;
    let co2 = filter(lines, len, false).context("failed to get a value for co2")?;

    println!("{}", ox * co2);
    Ok(())
}

fn filter(mut lines: Vec<Vec<u8>>, len: usize, most_common: bool) -> Option<u32> {
    for col in 0..len {
        let zeroes = lines.iter().filter(|line| line[col] == 0).count();
        let ones = lines.len() - zeroes;
        let mut common = if zeroes > ones { 0 } else { 1 };
        if !most_common {
            // flip bit
            common ^= 1;
        }
        lines.retain(|line| line[col] == common);
        if lines.len() == 1 {
            return Some(
                lines
                    .get(0)?
                    .iter()
                    .fold(0, |acc, b| (acc << 1) | *b as u32),
            );
        }
    }
    None
}
