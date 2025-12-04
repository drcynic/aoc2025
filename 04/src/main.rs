use std::{collections::HashSet, fs};

fn main() {
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

    let part1 = adj(&rolls).count();
    println!("part1 {:?}", part1);

    let mut count = 0;
    loop {
        let to_rm = adj(&rolls).cloned().collect::<HashSet<(i64, i64)>>();
        if to_rm.is_empty() {
            break;
        }
        count += to_rm.len();
        to_rm.iter().for_each(|p| {
            rolls.remove(p);
        });
    }
    println!("part2 {:?}", count);
}

fn adj(rolls: &HashSet<(i64, i64)>) -> impl Iterator<Item = &(i64, i64)> {
    rolls.iter().filter(|p| {
        let adj = [(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)]
            .iter()
            .filter(|dir| rolls.contains(&(p.0 + dir.0, p.1 + dir.1)))
            .count();
        adj < 4
    })
}
