use itertools::Itertools;
use num_traits::sign;
use std::fs;

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let mut pos = 50;
    let part1 = input
        .trim()
        .split("\n")
        .map(|x| {
            let d = if x.starts_with("L") { -1 } else { 1 };
            let off = x[1..].parse::<i32>().unwrap();
            pos += off * d;
            pos = pos.rem_euclid(100);
            pos
        })
        .filter(|&x| x == 0)
        .count();
    println!("{:?}", part1);

    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let mut pos: i32 = 50;
    let part2 = input
        .trim()
        .split("\n")
        .flat_map(|x| {
            let d = if x.starts_with("L") { -1 } else { 1 };
            let off = x[1..].parse::<usize>().unwrap();
            std::iter::repeat_n(d, off)
        })
        .map(|off| {
            pos += off;
            pos = pos.rem_euclid(100);
            pos
        })
        .filter(|&x| x == 0)
        .count();
    println!("{:?}", part2);
}
