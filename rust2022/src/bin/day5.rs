use std::str::FromStr;

use anyhow::Result;

#[derive(Clone, Debug)]
struct Task {
    amount: usize,
    from: usize,
    to: usize,
}

impl Task {
    pub fn new(amount: usize, from: usize, to: usize) -> Self {
        Task { amount, from, to }
    }
}

impl FromStr for Task {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut container: Vec<usize> = s
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<usize>>();

        Ok(Task::new(container[0], container[1], container[2]))
    }
}

fn main() -> Result<()> {
    let input = include_str!("../files/day5.txt");

    let mut container: Vec<String> = vec![
        String::from("WDGBHRV"),
        String::from("JNGCRF"),
        String::from("LSFHDNJ"),
        String::from("JDSV"),
        String::from("SHDRQWNV"),
        String::from("PGHCM"),
        String::from("FJBGLZHC"),
        String::from("SJR"),
        String::from("LGSRBNVM"),
    ];

    let mut container2: Vec<String> = container.clone();

    let part1: String = p1(input, &mut container);
    let part2: String = p2(input, &mut container2);

    println!("Result of part 1: {}", part1);
    println!("Result of part 2: {}", part2);

    Ok(())
}

fn p1(input: &str, container: &mut Vec<String>) -> String {
    for line in input.lines() {
        let task = line.parse::<Task>().unwrap();

        move_slot(&task, container, false);
    }

    get_top(&container)
}

fn p2(input: &str, container: &mut Vec<String>) -> String {
    for line in input.lines() {
        let task = line.parse::<Task>().unwrap();

        move_slot(&task, container, true);
    }

    get_top(&container)
}

fn move_slot(task: &Task, container: &mut Vec<String>, keep_still: bool) {
    let mut items = String::new();

    (0..task.amount).for_each(|_| {
        let elem: char = container[task.from - 1].pop().unwrap();

        items.push(elem);
    });

    if keep_still {
        items = items.chars().rev().collect();
    }

    container[task.to - 1].push_str(items.as_str());
}

fn get_top(container: &Vec<String>) -> String {
    container
        .iter()
        .map(|s| s.chars().last().unwrap())
        .collect::<String>()
}
