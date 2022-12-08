use anyhow::Result;

fn main() -> Result<()> {
    let mat = include_str!("../files/day8.txt")
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| (c.to_string()).parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let M: usize = mat.len(); // ROWS
    let N: usize = mat[0].len(); // COLS

    println!("Part 1: {}", part1(&mat, M, N));
    println!("Part 2: {}", part2(&mat, M, N));

    Ok(())
}

fn part1(mat: &Vec<Vec<usize>>, M: usize, N: usize) -> usize {
    let circ = 2 * N + 2 * M - 4;

    let mut counter = 0;

    for r in 1..M - 1 {
        for c in 1..N - 1 {
            let val = &mat[r][c];

            // UP
            let blocked = (r + 1..M).filter(|y| mat[*y][c] >= *val).count() != 0;

            if !blocked {
                counter += 1;
                continue;
            }

            // DOWN
            let blocked = (0..r).rev().filter(|y| mat[*y][c] >= *val).count() != 0;

            if !blocked {
                counter += 1;
                continue;
            }

            // RIGHT
            let blocked = (c + 1..N).filter(|x| mat[r][*x] >= *val).count() != 0;

            if !blocked {
                counter += 1;
                continue;
            }

            // LEFT
            let blocked = (0..c).rev().filter(|x| mat[r][*x] >= *val).count() != 0;

            if !blocked {
                counter += 1;
                continue;
            }
        }
    }

    counter + circ
}

fn part2(mat: &Vec<Vec<usize>>, M: usize, N: usize) -> usize {
    let mut viewing_distances: Vec<usize> = Vec::new();

    for r in 1..M - 1 {
        for c in 1..N - 1 {
            let val = &mat[r][c];
            let mut view_dist: usize = 1;

            // UP
            let dist = (r + 1..M).map(|y| mat[y][c]).position(|v| v >= *val);

            let dist = match dist {
                Some(d) => d + 1,
                None => M - r - 1,
            };

            view_dist *= dist;

            // DOWN
            let dist = (0..r).rev().map(|y| mat[y][c]).position(|v| v >= *val);

            let dist = match dist {
                Some(d) => d + 1,
                None => r,
            };

            view_dist *= dist;

            // RIGHT
            let dist = (c + 1..N).map(|x| mat[r][x]).position(|v| v >= *val);

            let dist = match dist {
                Some(d) => d + 1,
                None => N - c - 1,
            };

            //println!("d {} for VAL {}", dist, val);
            view_dist *= dist;

            // LEFT
            let dist = (0..c).rev().map(|x| mat[r][x]).position(|v| v >= *val);

            let dist = match dist {
                Some(d) => d + 1,
                None => c,
            };

            view_dist *= dist;

            viewing_distances.push(view_dist);
        }
    }

    // Find max of all trees viewing distances
    *viewing_distances.iter().max().unwrap() as usize
}
