use std::path::PathBuf;
use aoc23::render;

fn parse_row(row: &str) -> Vec<char> {
    row.chars().collect()
}

fn find_s(map: &[Vec<char>]) -> (usize, usize) {
    for (ri, row) in map.iter().enumerate() {
        for (ci, ch) in row.iter().enumerate() {
            if ch == &'S' {
                return (ri, ci);
            }
        }
    }
    panic!("Invalid map - could not find starting location!");
}

fn get_neighbors(map: &[Vec<char>], r: usize, c: usize) -> Vec<(usize, usize)> {
    let rows = map.len();
    let cols = map[0].len();
    let mut res = Vec::new();
    if r > 0 {
        res.push((r - 1, c));
    }
    if r < rows - 1 {
        res.push((r + 1, c));
    }
    if c > 0 {
        res.push((r, c - 1));
    }
    if c < cols - 1 {
        res.push((r, c + 1));
    }
    res
}

fn connects_left(val: char) -> bool {
    val == 'S' || val == 'J' || val == '7' || val == '-'
}

fn connects_right(val: char) -> bool {
    val == 'S' || val == 'F' || val == 'L' || val == '-'
}

fn connects_up(val: char) -> bool {
    val == 'S' || val == 'L' || val == 'J' || val == '|'
}

fn connects_down(val: char) -> bool {
    val == 'S' || val == 'F' || val == '7' || val == '|'
}

fn connects(map: &[Vec<char>], p1: (usize, usize), p2: (usize, usize)) -> bool {
    let p1_val = map[p1.0][p1.1];
    let p2_val = map[p2.0][p2.1];

    assert!(p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1) == 1); // format!("Invalid neighbors: {:?} and {:?}", p1, p2));

    if p1_val == '.' || p2_val == '.' {
        return false;
    }

    // P1 is to the left of P2
    if p1.0 == p2.0 && p1.1 == p2.1 - 1 {
        connects_right(p1_val) && connects_left(p2_val)
    }
    // P1 is to the right of P2
    else if p1.0 == p2.0 && p1.1 == p2.1 + 1 {
        connects_left(p1_val) && connects_right(p2_val)
    }
    // P1 is above P2
    else if p1.1 == p2.1 && p1.0 == p2.0 - 1 {
        connects_down(p1_val) && connects_up(p2_val)
    }
    // P1 is below P2
    else if p1.1 == p2.1 && p1.0 == p2.0 + 1 {
        connects_up(p1_val) && connects_down(p2_val)
    }
    else {
        panic!("Invalid coordinate configuration!");
    }
}

fn find_loop_from_candidate(map: &[Vec<char>], start: (usize, usize), n_coord: (usize, usize)) -> Option<Vec<Vec<(i64, i64)>>> {
    let mut queue: Vec<((usize, usize), usize)> = Vec::new();
    let mut prev: Vec<Vec<(i64, i64)>> = Vec::new();
    let mut step: Vec<Vec<i64>> = Vec::new();
    for _ in 0..map.len() {
        let row = vec![(-1, -1); map[0].len()];
        prev.push(row);
        let row = vec![-1; map[0].len()];
        step.push(row);
    }

    prev[n_coord.0][n_coord.1] = (start.0 as i64, start.1 as i64);
    queue.push((n_coord, 0));

    let mut found = false;
    while let Some((cur, step)) = queue.pop() {
        if map[cur.0][cur.1] == 'S' {
            println!("Found S again at {} {}", cur.0, cur.1);
            println!("Prev was: {:?}", prev[cur.0][cur.1]);
            found = true;
            break;
        }
        // println!("{:?} @ {}", cur, step);
        let s_neighbors = get_neighbors(map, cur.0, cur.1);
        for n_coord in s_neighbors {
            if connects(map, cur, n_coord) {
                if prev[n_coord.0][n_coord.1] == (-1, -1) {
                    if map[n_coord.0][n_coord.1] == 'S' && step == 0 {
                        // prevent degenerate loops
                        continue;
                    }
                    // unvisited
                    prev[n_coord.0][n_coord.1] = (cur.0 as i64, cur.1 as i64);
                    queue.push((n_coord, step + 1));
                }
            }
        }
    }
    if found {
        Some(prev)
    }
    else {
        None
    }
}

fn bfs_loop(map: &[Vec<char>]) -> Vec<(usize, usize)> {
    let start = find_s(map);
    // prev[start.0][start.1]

    // Special logic to handle the fact that 'S' is ambiguous.
    // TODO(andrei): With my fancy new connect functions, do we _actually_ need this?
    let s_neighbors = get_neighbors(map, start.0, start.1);
    let mut prev = None;
    for n_coord in s_neighbors {
        if connects(map, start, n_coord) {
            if let Some(res) = find_loop_from_candidate(map, start, n_coord) {
                prev = Some(res);
                break;
            }
            // TODO do the whole algo for each initial special neighbor!
            break;
        }
    }

    if let Some(prev) = prev {
        let mut traj = Vec::new();
        let mut cur = prev[start.0][start.1];
        loop {
            traj.push(cur);
            cur = prev[cur.0 as usize][cur.1 as usize];
            if map[cur.0 as usize][cur.1 as usize] == 'S' {
                break;
            }
        }


        traj.iter().rev().map(|p| (p.0 as usize, p.1 as usize)).collect()
    }
    else {
        panic!("Could not find any loop");
    }
}

fn day_10_tbd(input_fpath: &PathBuf) -> (i64, i64) {
    let in_txt = std::fs::read_to_string(input_fpath)
        .unwrap_or_else(|_| panic!("Read input from {:?}", input_fpath));
    let map: Vec<Vec<char>> = in_txt.split_terminator('\n').map(parse_row).collect();
    // println!("{}", render(&map));

    let traj = bfs_loop(&map);
    // let tl = traj.len();
    // let td: Vec<usize> = (0..tl).map(|v| v.min(tl + 1 - v) as usize).collect();
    // let traj_mid = traj[traj.len() / 2];
    let max_dist = (traj.len() as f64) / 2.0;
    // println!("max dist float: {}", max_dist);
    // println!("Found traj: {:?}", traj);
    // println!("{:?}", td);
    // println!("{:?}", traj_mid);

    let part_one_answer: i64 = max_dist.round() as i64;
    let part_two_answer: i64 = 0;

    (part_one_answer, part_two_answer)
}

fn main() {
    println!(
        "{:?}",
        day_10_tbd(&PathBuf::from("input/10-demo-00.txt"))
    );
    println!(
        "{:?}",
        day_10_tbd(&PathBuf::from("input/10-demo-01.txt"))
    );
    // I got Part 1 right on my first try, 2023-10-10 at 1:04am. I am actually very proud of this, haha!
    println!(
        "{:?}",
        day_10_tbd(&PathBuf::from("input/10.txt"))
    );
}
