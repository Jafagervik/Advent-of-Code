use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn part1() {
    let result: u32 = include_str!("../files/day3.txt")
        .lines()
        .map(|line| {
            let half: usize = line.len() / 2;
            let first_half: String = line.chars().take(half).collect();
            let second_half: String = line.chars().skip(half).take(half).collect();

            let mut char_in_both: char = 'a';

            // find same char in each string
            for c in first_half.chars() {
                if second_half.contains(c) {
                    char_in_both = c;
                    break;
                }
            }

            // compute it's value
            let reduce: u32 = if char_in_both.is_uppercase() { 38 } else { 96 };

            char_in_both as u32 - reduce
        })
        .sum();

    println!("Result of day 3 is: {}", result);
}

fn read_from_file(path: &str) -> Lines<BufReader<File>> {
    let file = File::open(path).expect("Cannot read file :/");
    BufReader::new(file).lines()
}

fn main() -> Result<()> {
    let lines = read_from_file("src/files/day3.txt");

    for line in lines {}

    Ok(())
}
