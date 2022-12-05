use std::str::FromStr;

use anyhow::Result;

#[derive(Debug)]
struct Task {
    field: u64,
}

impl Task {
    pub fn new(field: u64) -> Self {
        Task { field }
    }
}

impl FromStr for Task {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Task::new(s.parse::<u64>().unwrap_or(0)))
    }
}

fn main() -> Result<()> {
    let input = include_str!("../files/day6.txt");

    let p1 = solve(&input);
    let p2 = solve(&input);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    Ok(())
}

fn solve(input: &str) -> u64 {
    input
        .lines()
        .map(|line| line.parse::<Task>().unwrap())
        .map(|t| t.field)
        .sum()
}
