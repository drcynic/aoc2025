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
            let mut str = "".to_string();
            for n in 0..num {
                let idx = fst_max_idx(&b[start_idx..(b.len() - num + n + 1)]) + start_idx;
                str = format!("{}{}", str, b[idx].to_string());
                start_idx = idx + 1;
            }
            let max_val = str.parse::<usize>().unwrap();
            max_val
        })
        .sum::<usize>()
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
