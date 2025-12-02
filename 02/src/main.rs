use std::fs;

fn main() {
    let part1 = fs::read_to_string("input2.txt")
        .unwrap()
        .trim()
        .split(",")
        .flat_map(|s| {
            let (l, r) = s.trim().split_once("-").unwrap();
            let l = l.parse::<usize>().unwrap();
            let r = r.parse::<usize>().unwrap();
            l..=r
        })
        .filter(|id| {
            let id_str = id.to_string();
            let lh = id_str.len() / 2;
            id_str[..lh] == id_str[lh..]
        })
        .sum::<usize>();
    println!("part1 {:?}", part1);

    let part2 = fs::read_to_string("input2.txt")
        .unwrap()
        .trim()
        .split(",")
        .flat_map(|s| {
            let (l, r) = s.trim().split_once("-").unwrap();
            let l = l.parse::<usize>().unwrap();
            let r = r.parse::<usize>().unwrap();
            l..=r
        })
        .filter(|id| {
            let id_str = id.to_string();
            let slen = id_str.len();
            for l in 1..=slen / 2 {
                if slen % l != 0 {
                    continue;
                }
                let fst = &id_str[..l].as_bytes();
                if id_str.as_bytes().chunks(l).skip(1).all(|c| c == *fst) {
                    return true;
                }
            }
            false
        })
        .sum::<usize>();
    println!("part2 {:?}", part2);
}
