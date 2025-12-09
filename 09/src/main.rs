use std::{collections::BTreeSet, f64::consts::PI, fs};

use itertools::Itertools;
use memoize::memoize;

fn main() {
    let input = fs::read_to_string("input2.txt").unwrap();
    let corners = input
        .lines()
        .map(|l| {
            let (l, r) = l.trim().split_once(',').unwrap();
            (l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();
    let mut max_area = 0;
    for i in 0..corners.len() {
        let (x1, y1) = corners[i];
        for j in i + 1..corners.len() {
            let (x2, y2) = corners[j];
            let dx = (x2 - x1).abs() + 1;
            let dy = (y2 - y1).abs() + 1;
            let area = dx * dy;
            max_area = max_area.max(area);
        }
    }
    println!("part1 {:?}", max_area);

    let input = fs::read_to_string("input2.txt").unwrap();
    let corners = input
        .lines()
        .map(|l| {
            let (l, r) = l.trim().split_once(',').unwrap();
            (l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();
    let edges = corners
        .iter()
        .circular_tuple_windows()
        .take(corners.len())
        .fold(BTreeSet::new(), |mut acc, (c, n)| {
            let dx = n.0 - c.0;
            if dx == 0 {
                for y in c.1.min(n.1)..=c.1.max(n.1) {
                    acc.insert((c.0, y));
                }
            } else {
                for x in c.0.min(n.0)..=c.0.max(n.0) {
                    acc.insert((x, c.1));
                }
            }
            acc
        });

    // p2 - some kind of brute force solution, extreme slow, but works
    let mut max_area = 0;
    for i in 0..corners.len() {
        println!("i: {}", i);
        let (x1, y1) = corners[i];
        for j in i + 1..corners.len() {
            let (x2, y2) = corners[j];

            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);

            // check corners
            let b1 = edges.contains(&(min_x, min_y)) || inside_polygon(&corners, (min_x, min_y));
            let b2 = edges.contains(&(max_x, min_y)) || inside_polygon(&corners, (max_x, min_y));
            let b3 = edges.contains(&(min_x, max_y)) || inside_polygon(&corners, (min_x, max_y));
            let b4 = edges.contains(&(max_x, max_y)) || inside_polygon(&corners, (max_x, max_y));
            if !(b1 && b2 && b3 && b4) {
                continue;
            }
            // corners are ok, check inlay edges
            // println!("min_x: {}, max_x: {}, min_y: {}, max_y: {}", min_x, max_x, min_y, max_y);
            let b1 = (min_x + 1..max_x).all(|x| inside_polygon(&corners, (x, min_y + 1)));
            let b2 = (min_x + 1..max_x).all(|x| inside_polygon(&corners, (x, max_y - 1)));
            let b3 = (min_y + 1..max_y).all(|y| inside_polygon(&corners, (min_x + 1, y)));
            let b4 = (min_y + 1..max_y).all(|y| inside_polygon(&corners, (max_x - 1, y)));
            if !(b1 && b2 && b3 && b4) {
                continue;
            }
            let dx = (x2 - x1).abs() + 1;
            let dy = (y2 - y1).abs() + 1;
            let area = dx * dy;
            max_area = max_area.max(area);
        }
    }
    println!("part2 {:?}", max_area);
}

#[memoize(Ignore: polygon)]
pub fn inside_polygon(polygon: &[(i64, i64)], p: (i64, i64)) -> bool {
    let n = polygon.len();
    if n < 3 {
        return false;
    }

    let mut angle = 0.0;

    for i in 0..n {
        let p1 = (polygon[i].0 - p.0, polygon[i].1 - p.1);
        let p2 = (polygon[(i + 1) % n].0 - p.0, polygon[(i + 1) % n].1 - p.1);

        angle += angle_2d(p1.0 as f64, p1.1 as f64, p2.0 as f64, p2.1 as f64);
    }

    angle.abs() >= PI
}

fn angle_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let theta1 = y1.atan2(x1);
    let theta2 = y2.atan2(x2);
    let mut dtheta = theta2 - theta1;

    const TWO_PI: f64 = 2.0 * PI;

    while dtheta > PI {
        dtheta -= TWO_PI;
    }
    while dtheta < -PI {
        dtheta += TWO_PI;
    }

    dtheta
}
