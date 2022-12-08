/*use anyhow::Result;

fn main() -> Result<()> {
    let result = include_str!("../files/day4.txt")
        .lines()
        .filter_map(|l| {
            let values: Vec<i32> = l
                .split(|c| c == ',' || c == '-')
                .map(|e| e.parse().unwrap())
                .collect();

            if values[0] <= values[2] && values[1] >= values[3]
                || values[2] <= values[0] && values[3] >= values[1]
                || values[3] >= values[0] && values[1] >= values[2]
            {
                Some(true)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .len();

    println!("Result of day 4: {}", result);

    Ok(())
}

fn _alt() -> usize {
    let lines = include_str!("../files/day4.txt").lines();

    let mut result: i32 = 0;

    for line in lines {
        let chars: Vec<_> = line.split(|c| c == ',' || c == '-').collect();

        let values: Vec<i32> = chars.iter().map(|e| e.parse().unwrap()).collect();

        // Part 2 is just adding the last clause
        if values[0] <= values[2] && values[1] >= values[3] {
            result += 1;
        } else if values[2] <= values[0] && values[3] >= values[1] {
            result += 1;
        } else if values[3] >= values[0] && values[1] >= values[2] {
            result += 1;
        }
    }

    return result as usize;
}
*/

use std::{fs, num::ParseIntError, str::FromStr};

pub fn part1(path: String) -> i32 {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    return contents
        .lines()
        .map(|line| line.parse::<Pair>())
        .filter(|pair| pair.as_ref().unwrap().full_contains())
        .count() as i32;
}

pub fn part2(path: String) -> i32 {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    return contents
        .lines()
        .map(|line| line.parse::<Pair>())
        .filter(|pair| pair.as_ref().unwrap().any_overlap())
        .count() as i32;
}

struct Pair {
    a_min: i32,
    a_max: i32,
    b_min: i32,
    b_max: i32,
}

impl Pair {
    fn full_contains(&self) -> bool {
        return (self.a_min >= self.b_min && self.a_max <= self.b_max)
            || (self.b_min >= self.a_min && self.b_max <= self.a_max);
    }

    fn any_overlap(&self) -> bool {
        return (self.a_max >= self.b_min && self.a_max <= self.b_max)
            || (self.b_max >= self.a_min && self.b_max <= self.a_max);
    }
}

impl FromStr for Pair {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pairs = s.split(",").flat_map(|s1| s1.split("-"));
        return Ok(Pair {
            a_min: pairs.next().unwrap().parse()?,
            a_max: pairs.next().unwrap().parse()?,
            b_min: pairs.next().unwrap().parse()?,
            b_max: pairs.next().unwrap().parse()?,
        });
    }
}

fn main() {
    println!("Part 2: {}", part2(String::from("src/files/day4.txt")));
}
