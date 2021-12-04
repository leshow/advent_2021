use anyhow::{Context, Result};

pub fn part_one() -> Result<()> {
    let (winners, mut boards) = parse()?;

    for pick in winners {
        for board in boards.iter_mut() {
            if let Some(soln) = board.solve(pick) {
                println!("solution found {}", soln);
                return Ok(());
            }
        }
    }

    Ok(())
}

struct Board(Vec<Vec<Option<u32>>>);
impl Board {
    // terribly unoptimized, but it works
    fn solve(&mut self, pick: u32) -> Option<u32> {
        for row in 0..self.0.len() {
            for col in 0..self.0[0].len() {
                if matches!(self.0[row][col], Some(n) if n == pick) {
                    self.0[row][col] = None;
                }
                if self.has_won() {
                    return Some(self.board_sum(pick));
                }
            }
        }
        None
    }

    fn has_won(&self) -> bool {
        // horizontal
        if self.0.iter().any(|line| line.iter().all(Option::is_none)) {
            return true;
        }
        // vertical
        if (0..self.0[0].len()).any(|col| self.0.iter().all(|line| line[col].is_none())) {
            return true;
        }
        false
    }

    fn board_sum(&self, pick: u32) -> u32 {
        let sum: u32 = self
            .0
            .iter()
            .map(|line| line.iter().flatten().sum::<u32>())
            .sum();
        sum * pick
    }
}

pub fn part_two() -> Result<()> {
    let (winners, mut boards) = parse()?;

    let mut last = None;
    for pick in winners {
        for (i, board) in boards.iter_mut().enumerate() {
            if board.has_won() {
                continue;
            }
            if board.solve(pick).is_some() {
                last = Some((i, pick));
            }
        }
    }
    if let Some((i, num)) = last {
        let sum = boards[i].board_sum(num);
        println!("{:?}", sum);
    }

    Ok(())
}

fn parse() -> Result<(Vec<u32>, Vec<Board>)> {
    let (winners, boards) = include_str!("../data/day4_1.txt")
        .split_once("\n\n")
        .context("cant get winners")?;

    let winners = winners.split(',').flat_map(|n| n.parse()).collect();

    let boards = boards
        .split("\n\n")
        .map(|board| {
            Board(
                board
                    .lines()
                    .map(|row| {
                        row.split_whitespace()
                            .flat_map(|n| n.parse())
                            .map(Some)
                            .collect()
                    })
                    .collect(),
            )
        })
        .collect();
    Ok((winners, boards))
}
