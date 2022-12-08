use std::collections::HashSet;

use anyhow::Result;

trait AllUniqueIter: Iterator {
    fn all_unique(self) -> bool;
}

// Taken from a better implementation

impl<'a, I: Iterator<Item = &'a u8>> AllUniqueIter for I {
    fn all_unique(mut self) -> bool {
        let mut set = 0_u32;
        while let Some(byte) = self.next() {
            let offset = (byte - b'a') as usize;
            if set & (1 << offset) != 0 {
                return false;
            } else {
                set |= 1 << offset;
            }
        }

        true
    }
}

fn main() -> Result<()> {
    let input = include_str!("../files/day6.txt");

    let p1 = solve(input, 4);
    let p2 = solve(input, 14);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    Ok(())
}

fn solve(input: &str, slize_size: usize) -> usize {
    /*input
            .windows(slize_size)
            .enumerate()
            .find_map(|(offset, bytes)| bytes.iter().all_unique().then(|| offset + slize_size))
            .unwrap()
    */
    // My own code

    let mut marker: usize = 0;
    let n = input.len();

    let mut sets: HashSet<char>;

    for i in 0..n - slize_size {
        //sets = HashSet::from_iter((i..i + slize_size).map(|s| input.chars().nth(s).unwrap()));
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

    /* Also another implementation

       (0..input.len())
    .find(|&i| {
        input[i..i + message_size]
            .chars()
            .collect::<HashSet<_>>()
            .len()
            == slize_size
    })
    .map(|idx| (idx + slize_size)).unwrap()
    */
}
