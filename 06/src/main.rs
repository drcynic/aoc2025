use std::{collections::BTreeSet, fs};

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
        let mut valid = true;
        for r in 0..vals_str.len() {
            let ch = &vals_str[r][c..c + 1];
            if ch != " " {
                valid = false;
                break;
            }
        }
        if valid {
            split_indices.push(c);
        }
    }
    split_indices.push(vals_str[0].len());

    let p2 = split_indices
        .windows(2)
        .enumerate()
        .map(|(c, w)| {
            let (start, end) = (w[0] + 1, w[1]);
            let start = if start == 1 { 0 } else { start };
            let vals = vals_str.iter().map(|s| &s[start..end]).collect::<Vec<_>>();
            let mut col_res = if ops[c] == "*" { 1 } else { 0 };
            for sc in 0..vals[0].len() {
                let mut nums = Vec::new();
                for vi in 0..vals.len() {
                    let ch = vals[vi].as_bytes()[sc];
                    if ch >= b'0' && ch <= b'9' {
                        nums.push(ch - b'0');
                    }
                }
                let num_str = nums.iter().join("");
                let num = num_str.parse::<usize>().unwrap();
                if ops[c] == "+" {
                    col_res += num;
                } else {
                    col_res *= num;
                }
            }
            col_res
        })
        .sum::<usize>();

    println!("part2 {:?}", p2);
}
