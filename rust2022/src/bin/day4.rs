use anyhow::Result;

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
