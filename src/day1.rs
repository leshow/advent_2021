use anyhow::Result;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

pub fn part_one() -> Result<()> {
    let buf = BufReader::new(File::open("./data/day1_1.txt")?)
        .lines()
        .flatten()
        .flat_map(|line| line.parse::<u32>())
        .collect::<Vec<_>>();
    let count = buf
        .iter()
        .zip(buf.iter().skip(1))
        .fold(0, |inc, (f, s)| if s > f { inc + 1 } else { inc });
    println!("{}", count);

    Ok(())
}

pub fn part_two() -> Result<()> {
    let buf = BufReader::new(File::open("./data/day1_1.txt")?)
        .lines()
        .flatten()
        .flat_map(|line| line.parse::<u32>())
        .collect::<Vec<_>>();
    let iter = buf.windows(3).map(|slide| slide.iter().sum());
    let count = iter.clone().zip(iter.skip(1)).fold(
        0,
        |inc, (f, s): (u32, u32)| if s > f { inc + 1 } else { inc },
    );
    println!("{}", count);
    Ok(())
}
