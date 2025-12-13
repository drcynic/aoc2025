use std::fs;

use itertools::Itertools;
use memoize::memoize;
use num::Integer;

fn main() {
    let input = fs::read_to_string("input2.txt").unwrap();
    let mut indicators = Vec::new();
    let mut toggles = Vec::new();
    let mut joltages = Vec::new();
    input.lines().for_each(|l| {
        let (i, t) = l.trim().split_once("] ").unwrap();
        let i = i[1..].chars().map(|c| if c == '#' { 1 } else { 0 }).collect::<Vec<_>>();
        indicators.push(i);

        let (t, j) = t.trim().split_once(" {").unwrap();
        let t = t
            .split_whitespace()
            .map(|s| {
                s[1..s.len() - 1]
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        toggles.push(t);

        let j = j[..j.len() - 1]
            .trim()
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        joltages.push(j);
    });

    let p1 = indicators
        .iter()
        .zip(&toggles)
        .map(|(target, toggles)| {
            toggles
                .iter()
                .powerset()
                .filter_map(|toggles| {
                    let mut state = vec![0usize; target.len()];
                    for &toggle in &toggles {
                        for i in 0..toggle.len() {
                            state[toggle[i]] ^= 1;
                        }
                    }
                    if state == *target { Some(toggles.len()) } else { None }
                })
                .min()
                .unwrap()
        })
        .sum::<usize>();
    println!("part1 {:?}", p1);

    // part2 using the DP solution from
    // https://old.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
    // which is pretty nice and not using ILP
    let p2 = joltages
        .iter()
        .zip(&toggles)
        .map(|(joltages, toggles)| {
            // println!("joltages: {:?}", joltages);
            memoized_flush_dfs();
            dfs(joltages.clone(), toggles)
        })
        .sum::<usize>();
    println!("part2 {:?}", p2);
}

const INVALID: usize = 1000000;

#[memoize(Ignore: toggles)]
fn dfs(targets: Vec<usize>, toggles: &Vec<Vec<usize>>) -> usize {
    if targets.iter().all(|&t| t == 0) {
        return 0;
    }

    let target = targets
        .iter()
        .enumerate()
        .filter(|(_, t)| t.is_odd())
        .fold(vec![0; targets.len()], |mut acc, (i, _)| {
            acc[i] = 1usize;
            acc
        });

    toggles
        .iter()
        .powerset()
        .filter_map(|toggles| {
            let mut state = vec![0usize; target.len()];
            for &toggle in &toggles {
                for i in 0..toggle.len() {
                    state[toggle[i]] ^= 1;
                }
            }
            if state == target { Some(toggles) } else { None }
        })
        .map(|min_toggles| {
            let mut new_targets = targets.clone();
            for t in &min_toggles {
                for i in 0..t.len() {
                    if new_targets[t[i]] == 0 {
                        return INVALID;
                    }
                    new_targets[t[i]] -= 1;
                }
            }
            for n in &mut new_targets {
                *n /= 2;
            }
            dfs(new_targets, toggles) * 2 + min_toggles.len()
        })
        .min()
        .unwrap_or(INVALID)
}
