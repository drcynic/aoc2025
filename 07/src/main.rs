use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let mut active_tachyons = HashSet::new();
    let input = fs::read_to_string("input2.txt").unwrap();
    let splitter = input.lines().enumerate().fold(HashSet::new(), |mut acc, (y, l)| {
        l.trim().as_bytes().iter().enumerate().for_each(|(x, &b)| {
            if b == b'^' {
                acc.insert((x as i64, y as i64));
            }
            if b == b'S' {
                active_tachyons.insert((x as i64, y as i64));
            }
        });
        acc
    });
    let height = input.trim().lines().count();

    let mut splits = 0;
    for r in 0..height {
        let mut new_tachyons = HashSet::new();
        for t in active_tachyons {
            let np = (t.0, t.1 + 1);
            if splitter.contains(&np) {
                new_tachyons.insert((t.0.saturating_sub(1), t.1 + 1));
                new_tachyons.insert((t.0.saturating_add(1), t.1 + 1));
                splits += 1;
            } else {
                new_tachyons.insert(np);
            }
        }
        active_tachyons = new_tachyons;
    }
    println!("part1 {:?}", splits);

    let mut active_tachyons = HashMap::new();
    let input = fs::read_to_string("input2.txt").unwrap();
    let splitter = input.lines().enumerate().fold(HashSet::new(), |mut acc, (y, l)| {
        l.trim().as_bytes().iter().enumerate().for_each(|(x, &b)| {
            if b == b'^' {
                acc.insert((x as i64, y as i64));
            }
            if b == b'S' {
                active_tachyons.insert((x as i64, y as i64), 1);
            }
        });
        acc
    });
    let height = input.trim().lines().count();

    for r in 0..height {
        let mut new_tachyons = HashMap::new();
        for (&pos, &count) in &active_tachyons {
            let np = (pos.0, pos.1 + 1);
            if splitter.contains(&np) {
                *new_tachyons.entry((pos.0.saturating_sub(1), pos.1 + 1)).or_insert(0) += count;
                *new_tachyons.entry((pos.0.saturating_add(1), pos.1 + 1)).or_insert(0) += count;
            } else {
                *new_tachyons.entry(np).or_insert(0) += count;
            }
        }
        active_tachyons = new_tachyons;
    }
    let p2 = active_tachyons.values().sum::<i64>();
    println!("part1 {:?}", p2);
}
