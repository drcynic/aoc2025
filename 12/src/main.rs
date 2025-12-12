use std::fs;

fn main() {
    let input = fs::read_to_string("input2.txt").unwrap();
    let b = input.trim().split("\n\n").collect::<Vec<_>>();
    let shapes = b[..b.len() - 1]
        .iter()
        .map(|b| {
            let mut shape = [[false; 3]; 3];
            b.lines().skip(1).enumerate().for_each(|(i, l)| {
                l.chars().enumerate().for_each(|(j, c)| {
                    shape[i][j] = c == '#';
                });
            });
            shape
        })
        .map(|shape| {
            let mut shapes = Vec::new();
            let mut shape = shape;
            let rotate_right = |shape: [[bool; 3]; 3]| {
                let mut new_shape = [[false; 3]; 3];
                for i in 0..3 {
                    for j in 0..3 {
                        new_shape[j][2 - i] = shape[i][j];
                    }
                }
                new_shape
            };
            let flip_horizontal = |shape: [[bool; 3]; 3]| {
                let mut new_shape = [[false; 3]; 3];
                for i in 0..3 {
                    for j in 0..3 {
                        new_shape[i][2 - j] = shape[i][j];
                    }
                }
                new_shape
            };
            let flip_vertical = |shape: [[bool; 3]; 3]| {
                let mut new_shape = [[false; 3]; 3];
                for i in 0..3 {
                    for j in 0..3 {
                        new_shape[2 - i][j] = shape[i][j];
                    }
                }
                new_shape
            };
            for _ in 0..4 {
                shapes.push(shape);
                shapes.push(flip_horizontal(shape));
                shapes.push(flip_vertical(shape));
                shape = rotate_right(shape);
            }
            shapes.sort_unstable();
            shapes.dedup();
            shapes
        })
        .collect::<Vec<_>>();
    // print_shapes(&shapes[0]);
    let regions = b
        .last()
        .unwrap()
        .lines()
        .map(|l| {
            let (d, s) = l.trim().split_once(": ").unwrap();
            let shapes = s.trim().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
            let (w, h) = d.split_once('x').unwrap();
            let (w, h) = (w.parse::<usize>().unwrap(), h.parse::<usize>().unwrap());
            (w, h, shapes)
        })
        .collect::<Vec<_>>();

    let p1 = regions
        .iter()
        .filter(|(w, h, shape_cnt)| {
            let area = w * h;
            let needed_area = shape_cnt.iter().sum::<usize>() * 9;
            // in general the check below is sufficient, but I had the DFS stuff already implemented
            if area < needed_area {
                return false;
            }
            dfs(&mut vec![vec![false; *w]; *h], &shapes, &mut shape_cnt.clone())
        })
        .count();
    println!("p1: {}", p1);
}

fn dfs(grid: &mut Vec<Vec<bool>>, shapes: &Vec<Vec<[[bool; 3]; 3]>>, shape_cnt: &mut [usize]) -> bool {
    if shape_cnt.iter().all(|&cnt| cnt == 0) {
        return true;
    }
    let (w, h) = (grid[0].len(), grid.len());
    for y in 0..h {
        for x in 0..w {
            let start_pos = (x, y);
            if grid[y][x] {
                continue;
            }
            for (i, shape) in shapes.iter().enumerate() {
                if shape_cnt[i] == 0 {
                    continue;
                }
                for variant in shape {
                    if !valid_shape(&grid, variant, start_pos) {
                        continue;
                    }
                    // print_grid(&grid);
                    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    put_shape(grid, variant, start_pos);
                    shape_cnt[i] -= 1;
                    if dfs(grid, shapes, shape_cnt) {
                        return true;
                    }
                    shape_cnt[i] += 1;
                    remove_shape(grid, variant, start_pos);
                }
            }
        }
    }
    false
}

fn valid_shape(grid: &Vec<Vec<bool>>, shape: &[[bool; 3]; 3], pos: (usize, usize)) -> bool {
    let (w, h) = (grid[0].len(), grid.len());
    if pos.0 + 3 > w || pos.1 + 3 > h {
        return false;
    }
    for y in 0..3 {
        for x in 0..3 {
            let p = (pos.0 + x, pos.1 + y);
            if grid[p.1][p.0] && shape[y][x] {
                return false;
            }
        }
    }
    true
}

fn put_shape(grid: &mut Vec<Vec<bool>>, shape: &[[bool; 3]; 3], pos: (usize, usize)) {
    for y in 0..3 {
        for x in 0..3 {
            if shape[y][x] {
                grid[pos.1 + y][pos.0 + x] = true;
            }
        }
    }
}

fn remove_shape(grid: &mut Vec<Vec<bool>>, shape: &[[bool; 3]; 3], pos: (usize, usize)) {
    for y in 0..3 {
        for x in 0..3 {
            if shape[y][x] {
                grid[pos.1 + y][pos.0 + x] = false;
            }
        }
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<bool>>) {
    for row in grid {
        for &cell in row {
            print!("{}", if cell { '#' } else { '.' });
        }
        println!();
    }
    println!();
}

#[allow(dead_code)]
fn print_shapes(shapes: &[[[bool; 3]; 3]]) {
    for shape in shapes {
        for row in shape {
            for &cell in row {
                print!("{}", if cell { '#' } else { '.' });
            }
            println!();
        }
        println!();
    }
}
