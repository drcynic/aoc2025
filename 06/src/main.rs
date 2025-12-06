use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input2.txt").unwrap();
    let lines = input.trim().lines().collect::<Vec<_>>();
    let ops_idx = lines.len() - 1;
    let ops = lines[ops_idx]
        .trim()
        .split(" ")
        .filter_map(|e| if e == " " || e == "" { None } else { Some(e) })
        .collect::<Vec<_>>();
    let num_ops = ops.len();

    let part1 = lines[..ops_idx]
        .iter()
        .fold(vec![0; num_ops], |mut acc, l| {
            let vals = l.trim().split(" ").filter_map(|e| e.parse::<usize>().ok()).collect::<Vec<_>>();
            vals.iter().enumerate().for_each(|(i, &val)| {
                if ops[i] == "*" {
                    if acc[i] == 0 {
                        acc[i] = val;
                    } else {
                        acc[i] *= val;
                    }
                } else {
                    acc[i] += val;
                }
            });
            acc
        })
        .iter()
        .sum::<usize>();
    println!("part1 {:?}", part1);

    let vals_str = lines[..ops_idx].iter().map(|l| l.to_string()).collect::<Vec<_>>();
    let mut split_indices = vec![0];
    for c in 0..vals_str[0].len() {
        if vals_str.iter().all(|val| val.chars().nth(c).unwrap() == ' ') {
            split_indices.push(c);
        }
    }
    split_indices.push(vals_str[0].len());

    let p2 = split_indices
        .iter()
        .tuple_windows()
        .enumerate()
        .map(|(c, (&start, &end))| {
            let op = if ops[c] == "+" { |l: usize, r: usize| l + r } else { |l, r| l * r };
            let start = if start == 0 { 0 } else { start + 1 };
            let vals = vals_str.iter().map(|s| &s[start..end]).collect::<Vec<_>>();
            let mut col_res = if ops[c] == "*" { 1 } else { 0 };
            for sc in 0..vals[0].len() {
                let num = vals
                    .iter()
                    .map(|val| val.chars().nth(sc).unwrap())
                    .join("")
                    .trim()
                    .parse::<usize>()
                    .unwrap();
                col_res = op(col_res, num);
            }
            col_res
        })
        .sum::<usize>();
    println!("part2 {:?}", p2);
}
