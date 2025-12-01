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
        .fold((50i32, 0), |(pos, mut count), s| {
            let d = if s.starts_with("L") { -1 } else { 1 };
            let off = s[1..].parse::<i32>().unwrap() * d;
            count += ((pos + off) / 100).abs();
            if off < 0 && pos > 0 && pos <= off.abs() {
                count += 1;
            }
            ((pos + off).rem_euclid(100), count)
        })
        .1;
    println!("{:?}", part2);
}
