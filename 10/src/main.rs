use std::{collections::BTreeSet, f64::consts::PI, fs};

use good_lp::*;
use itertools::Itertools;
use memoize::memoize;

fn main() {
    let input = fs::read_to_string("input2.txt").unwrap();
    let mut indicators = Vec::new();
    let mut toggles = Vec::new();
    let mut joltages = Vec::new();
    input.lines().for_each(|l| {
        let (i, t) = l.trim().split_once("] ").unwrap();
        let i = i[1..]
            .chars()
            .enumerate()
            .fold(0u32, |acc, (i, c)| acc | (if c == '#' { 1 } else { 0 }) << i);
        indicators.push(i);

        let (t, j) = t.trim().split_once(" {").unwrap();
        let t = t
            .split_whitespace()
            .map(|s| {
                s[1..s.len() - 1]
                    .split(',')
                    .fold(0u32, |acc, s| acc | 1 << s.parse::<u32>().unwrap())
            })
            .collect::<Vec<_>>();
        toggles.push(t);

        let j = j[..j.len() - 1]
            .trim()
            .split(",")
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        joltages.push(j);
    });

    let p1 = indicators
        .iter()
        .zip(&toggles)
        .map(|(i, toggles)| {
            memoized_flush_dfs();
            dfs(0, 0, *i, &toggles)
        })
        .sum::<usize>();
    println!("part1 {:?}", p1);

    // hmm use ILP solver as everything else seems to be to slow
    let p2 = joltages
        .iter()
        .zip(&toggles)
        .map(|(j, toggles)| {
            let mut vars = variables!();
            let press_vars = (0..toggles.len())
                .map(|_| vars.add(variable().min(0).integer()))
                .collect::<Vec<_>>();
            let mut expressions = vec![0.into_expression(); j.len()];
            for idx in 0..toggles.len() {
                let toggle = toggles[idx];
                for ti in 0..j.len() {
                    if toggle & (1 << ti) != 0 {
                        expressions[ti] += press_vars[idx];
                    }
                }
            }
            let hs = expressions
                .into_iter()
                .zip(j)
                .fold(highs(vars.minimise(press_vars.iter().sum::<Expression>())), |mut acc, (e, &j)| {
                    acc.add_constraint(e.eq(j as f64));
                    acc
                })
                .solve()
                .unwrap();
            press_vars.iter().map(|&v| hs.value(v)).sum::<f64>() as usize
        })
        .sum::<usize>();
    println!("part2 {:?}", p2);
}

#[memoize(Ignore: toggles)]
fn dfs(state: u32, cost: usize, target: u32, toggles: &[u32]) -> usize {
    if cost > 15 {
        return usize::MAX;
    }

    if state == target {
        return cost;
    }

    let mut cur_cost = usize::MAX;
    for toggle in toggles {
        let c = dfs(state ^ toggle, cost + 1, target, toggles);
        cur_cost = cur_cost.min(c);
    }
    cur_cost
}
