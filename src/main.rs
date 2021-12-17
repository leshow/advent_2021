#![allow(dead_code)]
mod day1;
mod day10;
mod day11;
mod day12;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    dbg!(day15::part_two(day15::input(include_str!(
        "../data/day15.txt"
    ))));
}
