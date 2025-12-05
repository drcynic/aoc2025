use std::fs;

fn main() {
    let input = fs::read_to_string("input2.txt").unwrap();
    let (l, r) = input.split_once("\n\n").unwrap();
    let ranges = l
        .lines()
        .map(|l| {
            let (l, r) = l.split_once("-").unwrap();
            (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();
    let fresh = r
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
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

    let input = fs::read_to_string("input2.txt").unwrap();
    let (l, _) = input.split_once("\n\n").unwrap();
    let mut ranges = l
        .lines()
        .map(|l| {
            let (l, r) = l.split_once("-").unwrap();
            (l.parse::<isize>().unwrap(), r.parse::<isize>().unwrap())
        })
        .collect::<Vec<_>>();
    ranges.sort_by(|&l, &r| l.cmp(&r));
    let mut cur = -1;
    let mut sum = 0;
    for r in &ranges {
        if r.0 > cur {
            sum += r.1 - r.0 + 1;
        } else if r.0 == cur {
            sum += r.1 - cur;
        } else {
            sum += std::cmp::max(r.1 - cur, 0);
        }
        cur = std::cmp::max(r.1, cur);
    }
    println!("part2: {:?}", sum);
}
