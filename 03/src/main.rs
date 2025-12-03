use std::fs;

fn main() {
    let part1 = solve("input2.txt", 2);
    println!("part1 {:?}", part1);
    let part2 = solve("input2.txt", 12);
    println!("part2 {:?}", part2);
}

fn solve(input: &str, num: usize) -> usize {
    fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|l| {
            let b = l.trim().as_bytes().iter().map(|&b| (b - b'0') as usize).collect::<Vec<_>>();
            let mut start_idx = 0;
            (0..num).fold(0, |max_val, n| {
                let idx = fst_max_idx(&b[start_idx..(b.len() - num + n + 1)]) + start_idx;
                start_idx = idx + 1;
                max_val + b[idx] * 10usize.pow((num - n - 1) as u32)
            })
        })
        .sum()
}

fn fst_max_idx(bat: &[usize]) -> usize {
    let mut max_idx = 0;
    let mut max_val = bat[0];
    for i in 1..bat.len() {
        if bat[i] > max_val {
            max_idx = i;
            max_val = bat[i];
        }
    }
    max_idx
}
