use anyhow::Context;
use std::ops::Add;

use std::{
    fs::{self, File},
    io::{prelude::*, BufReader},
};

pub fn part_one() -> anyhow::Result<()> {
    let pos: Pos = BufReader::new(File::open("./data/day2_1.txt")?)
        .lines()
        .flatten()
        .flat_map(|line| line.parse::<Pos>())
        .fold(Pos::default(), |acc, a| acc + a);
    println!("{:?} mul: {}", pos, pos.x * pos.y);

    Ok(())
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

impl std::str::FromStr for Pos {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');
        let direction = iter.next().context("cant parse direction")?;
        let units = iter.next().context("cant parse units")?.parse::<i32>()?;
        match direction {
            "forward" => Ok(Pos { x: units, y: 0 }),
            "down" => Ok(Pos { x: 0, y: units }),
            "up" => Ok(Pos { x: 0, y: -units }),
            _ => Err(anyhow::anyhow!("unknown direction")),
        }
    }
}

impl Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// use separate types for p2

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
struct PosTwo {
    x: i32,
    y: i32,
    aim: i32,
}

// not the prettiest, but we borrow the Pos parsing and treat Pos.y as aim,
// we can add them with PosTwo to do the rest
impl Add<Pos> for PosTwo {
    type Output = PosTwo;

    fn add(self, rhs: Pos) -> Self::Output {
        let aim = self.aim + rhs.y;
        PosTwo {
            x: self.x + rhs.x,
            y: self.y + (aim * rhs.x),
            aim,
        }
    }
}

pub fn part_two() -> anyhow::Result<()> {
    let pos: PosTwo = BufReader::new(File::open("./data/day2_1.txt")?)
        .lines()
        .flatten()
        .flat_map(|line| line.parse::<Pos>())
        .fold(PosTwo::default(), |acc, a| acc + a);
    println!("{:?} mul: {}", pos, pos.x * pos.y);

    Ok(())
}
