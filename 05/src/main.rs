use std::{collections::BTreeSet, fs};

fn main() {
    let input = fs::read_to_string("input2.txt").unwrap();
    let (l, r) = input.split_once("\n\n").unwrap();
    let ranges = l
        .lines()
        .map(|l| {
            let (l, r) = l.split_once("-").unwrap();
            (l.parse::<isize>().unwrap(), r.parse::<isize>().unwrap())
        })
        .collect::<BTreeSet<_>>();

    let fresh = r
        .lines()
        .map(|l| l.parse::<isize>().unwrap())
        .filter(|&i| {
            for range in &ranges {
                if range.0 <= i && i <= range.1 {
                    return true;
                }
            }
            false
        })
        .count();
    println!("part1 {:?}", fresh);

    let mut cur = -1;
    let mut sum = 0;
    for r in &ranges {
        if r.0 > cur {
            sum += r.1 - r.0 + 1;
        } else {
            sum += (r.1 - cur).max(0);
        }
        cur = r.1.max(cur);
    }
    println!("part2: {:?}", sum);
}
