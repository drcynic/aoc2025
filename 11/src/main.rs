use std::{collections::BTreeMap, fs};

use memoize::memoize;

fn main() {
    let input = fs::read_to_string("input2.txt").unwrap();
    let devices = input.lines().fold(BTreeMap::new(), |mut acc, l| {
        let (d, o) = l.trim().split_once(": ").unwrap();
        let o = o.trim().split_whitespace().collect::<Vec<_>>();
        acc.insert(d.to_string(), o);
        acc
    });

    let path_count = dfs("you".to_string(), "out".to_string(), &devices);
    println!("part1 {:?}", path_count);

    let part1 = dfs("svr".to_string(), "dac".to_string(), &devices);
    let part2 = dfs("dac".to_string(), "fft".to_string(), &devices);
    let part3 = dfs("fft".to_string(), "out".to_string(), &devices);
    let s1 = part1 * part2 * part3;
    let part1 = dfs("svr".to_string(), "fft".to_string(), &devices);
    let part2 = dfs("fft".to_string(), "dac".to_string(), &devices);
    let part3 = dfs("dac".to_string(), "out".to_string(), &devices);
    let s2 = part1 * part2 * part3;
    println!("part2 {:?}", s1 + s2);
}

#[memoize(Ignore: devices)]
fn dfs(pos: String, target: String, devices: &BTreeMap<String, Vec<&str>>) -> usize {
    if pos == target {
        return 1;
    } else if !devices.contains_key(&pos) {
        return 0;
    }
    devices[&pos].iter().map(|d| dfs(d.to_string(), target.clone(), devices)).sum()
}
