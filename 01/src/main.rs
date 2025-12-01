use std::fs;

fn main() {
    let part1 = fs::read_to_string("input2.txt")
        .unwrap()
        .trim()
        .split("\n")
        .fold((50i32, 0), |(pos, count), x| {
            let d = if x.starts_with("L") { -1 } else { 1 };
            let off = x[1..].parse::<i32>().unwrap() * d;
            let pos = (pos + off).rem_euclid(100);
            (pos, count + if pos == 0 { 1 } else { 0 })
        })
        .1;
    println!("{:?}", part1);

    let part2 = fs::read_to_string("input2.txt")
        .unwrap()
        .trim()
        .split("\n")
        .flat_map(|x| {
            let d = if x.starts_with("L") { -1 } else { 1 };
            let off = x[1..].parse::<usize>().unwrap();
            std::iter::repeat_n(d, off)
        })
        .fold((50i32, 0), |(pos, count), off| {
            let pos = (pos + off).rem_euclid(100);
            (pos, count + if pos == 0 { 1 } else { 0 })
        })
        .1;
    println!("{:?}", part2);
}
