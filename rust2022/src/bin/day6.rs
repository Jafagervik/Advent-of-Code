use std::collections::HashSet;

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../files/day6.txt");

    let p1 = solve(&input, 4);
    let p2 = solve(&input, 14);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    Ok(())
}

fn solve(input: &str, slize_size: usize) -> usize {
    let mut marker: usize = 0;

    let mut sets: HashSet<char>;

    for i in 0..input.len() - slize_size {
        sets = (i..i + slize_size)
            .map(|s| input.chars().nth(s).unwrap())
            .collect();

        if sets.len() == slize_size {
            marker = i + slize_size;
            break;
        }

        sets.clear();
    }

    return marker;
}
