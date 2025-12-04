use std::{collections::HashSet, fs};

fn main() {
    let rolls = fs::read_to_string("input2.txt")
        .unwrap()
        .lines()
        .enumerate()
        .fold(HashSet::new(), |mut acc, (y, l)| {
            l.trim().as_bytes().iter().enumerate().for_each(|(x, &b)| {
                if b == b'@' {
                    acc.insert((x as i64, y as i64));
                }
            });
            acc
        });
    let part1 = rolls
        .iter()
        .filter(|(x, y)| {
            let adj = [(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)]
                .iter()
                .filter(|dir| {
                    let p = (x + dir.0, y + dir.1);
                    rolls.contains(&p)
                })
                .count();
            adj < 4
        })
        .count();
    println!("part1 {:?}", part1);

    let mut rolls = fs::read_to_string("input2.txt")
        .unwrap()
        .lines()
        .enumerate()
        .fold(HashSet::new(), |mut acc, (y, l)| {
            l.trim().as_bytes().iter().enumerate().for_each(|(x, &b)| {
                if b == b'@' {
                    acc.insert((x as i64, y as i64));
                }
            });
            acc
        });

    let mut count = 0;
    let mut last = 0;
    loop {
        let to_rm = rolls
            .iter()
            .filter(|(x, y)| {
                let adj = [(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)]
                    .iter()
                    .filter(|dir| {
                        let p = (x + dir.0, y + dir.1);
                        rolls.contains(&p)
                    })
                    .count();
                adj < 4
            })
            .cloned()
            .collect::<HashSet<(i64, i64)>>();
        count += to_rm.len();
        if count == last {
            break;
        }
        last = count;
        rolls = rolls.difference(&to_rm).cloned().collect();
    }
    println!("part2 {:?}", count);
}
