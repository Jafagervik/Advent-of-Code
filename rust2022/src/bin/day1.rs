use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
// 1. find elf carrying the most calories
// 2. return how much

// Go through file, sum up consecutive rows into a vec and store next

fn read_from_file(path: &str) -> Lines<BufReader<File>> {
    let file = File::open(path).expect("Cannot read file :/");

    BufReader::new(file).lines()
}

fn main() {
    let path = "src/files/day1.txt";

    let lines = read_from_file(path);

    let mut calories: Vec<i32> = Vec::new();
    let mut curr = 0;

    for line in lines {
        if let Ok(val) = line {
            match val.parse::<i32>() {
                Ok(v) => curr += v,
                Err(_) => {
                    calories.push(curr);
                    curr = 0;
                }
            }
        }
    }

    let _result_p1: i32 = calories.iter().max().unwrap().to_owned();

    calories.sort();
    calories.reverse();

    let result: i32 = calories.iter().take(3).sum();

    println!("Result of day 1: {}", result);
}
