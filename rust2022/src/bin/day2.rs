use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn read_from_file(path: &str) -> Lines<BufReader<File>> {
    let file = File::open(path).expect("Cannot read file :/");

    BufReader::new(file).lines()
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

fn main() {
    let RESULTS: HashMap<&str, char> = HashMap::from([
        ("AX", 'C'),
        ("BX", 'A'),
        ("CX", 'B'),
        ("AY", 'A'),
        ("BY", 'B'),
        ("CY", 'C'),
        ("AZ", 'B'),
        ("BZ", 'C'),
        ("CZ", 'A'),
    ]);

    let MOVES: HashMap<char, i32> = HashMap::from([('A', 1), ('B', 2), ('C', 3)]);

    let POINTS: HashMap<char, i32> = HashMap::from([('X', 0), ('Y', 3), ('Z', 6)]);

    let path = "src/files/day2.txt";

    let mut result = 0;

    let lines = read_from_file(path);

    let result: i32 = lines
        .map(|line| {
            let mut moves: String = line.unwrap();
            remove_whitespace(&mut moves);

            let needed_move: &char = RESULTS.get(moves.as_str()).unwrap();

            let move_score = MOVES.get(needed_move).unwrap();
            let result_score = POINTS.get(&moves.chars().nth(1).unwrap()).unwrap();

            result_score + move_score
        })
        .collect();
    /*
    for line in lines {
        let mut moves: String = line.unwrap();
        remove_whitespace(&mut moves);

        let needed_move: &char = RESULTS.get(moves.as_str()).unwrap();

        let move_score = MOVES.get(needed_move).unwrap();
        let result_score = POINTS.get(&moves.chars().nth(1).unwrap()).unwrap();

        result += result_score + move_score;
    }*/

    println!("Result of day 2: {}", result);
}
