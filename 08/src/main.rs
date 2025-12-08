use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Point { x, y, z }
    }

    fn dist_squared(&self, other: &Point) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

fn main() {
    let input = fs::read_to_string("input2.txt").unwrap();
    let jb = input
        .lines()
        .map(|l| {
            let v = l.trim().split(',').map(|c| c.parse::<i64>().unwrap()).collect::<Vec<_>>();
            Point::new(v[0], v[1], v[2])
        })
        .collect::<Vec<_>>();
    let mut circuits = jb.iter().map(|b| HashSet::from([b.clone()])).collect::<Vec<_>>();
    let mut dists_by_boxes = HashMap::new();
    for jbs in 0..jb.len() {
        let cs = jb[jbs];
        for jbe in jbs + 1..jb.len() {
            let ce = jb[jbe];
            let d = cs.dist_squared(&ce);
            dists_by_boxes.insert(d, (cs, ce));
        }
    }
    let circuit_idx = |p: &Point, circuits: &Vec<HashSet<Point>>| {
        for (i, c) in circuits.iter().enumerate() {
            if c.contains(p) {
                return i;
            }
        }
        unreachable!("Point not found in any circuit")
    };

    // p1
    let rounds = 1000;
    let sorted_dists = dists_by_boxes.iter().sorted_by_key(|&(d, _)| d).take(rounds).collect::<Vec<_>>();
    for (_, p) in &sorted_dists {
        let i1 = circuit_idx(&p.0, &circuits);
        let i2 = circuit_idx(&p.1, &circuits);
        if i1 == i2 {
            continue;
        }

        let to_move = circuits[i2].clone();
        circuits[i1].extend(to_move);
        circuits.remove(i2);
    }
    let p1 = circuits.iter().map(|c| c.len()).sorted().rev().take(3).product::<usize>();
    println!("part1 {:?}", p1);

    // p2
    let mut circuits = jb.iter().map(|b| HashSet::from([b.clone()])).collect::<Vec<_>>();
    let sorted_dists = dists_by_boxes.iter().sorted_by_key(|&(d, _)| d).collect::<Vec<_>>();
    let mut last = *sorted_dists[0].1;
    for &(_, p) in &sorted_dists {
        let i1 = circuit_idx(&p.0, &circuits);
        let i2 = circuit_idx(&p.1, &circuits);
        if i1 == i2 {
            continue;
        }

        let to_move = circuits[i2].clone();
        circuits[i1].extend(to_move);
        circuits.remove(i2);
        last = *p;
        if circuits.len() == 1 {
            break;
        }
    }
    println!("part2 {:?}", last.0.x * last.1.x);
}
