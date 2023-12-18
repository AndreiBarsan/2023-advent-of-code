// AoC 2023 Day 16: The Floor Will Be Lava

use std::path::PathBuf;

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

/// Returns zero or more coordinates where the beam currently at state 'cmd' will have arrived after one step.
fn handle_cmd(map: &[Vec<char>], cmd: &(usize, usize, Dir)) -> Vec<(usize, usize, Dir)> {
    let (c_row, c_col, c_dir) = *cmd;
    let n_rows = map.len();
    let n_cols = map[0].len();
    let c_entry = map[c_row][c_col];

    let is_in_bounds = |r: usize, c: usize| r != usize::MAX && r < n_rows && c != usize::MAX && c < n_cols;

    let guard = |candidate_commands: &[(usize, usize, Dir)]| {
        candidate_commands
            .iter()
            .filter(|(r, c, _)| is_in_bounds(*r, *c))
            .map(|(r, c, d)| (*r, *c, *d))
            .collect::<Vec<(usize, usize, Dir)>>()
    };

    let ret = match (c_entry, c_dir) {
        // Empty space
        ('.', Dir::Up) => guard(&vec![(c_row - 1, c_col, Dir::Up)]),
        ('.', Dir::Down) => guard(&vec![(c_row + 1, c_col, Dir::Down)]),
        ('.', Dir::Left) => guard(&vec![(c_row, c_col - 1, Dir::Left)]),
        ('.', Dir::Right) => guard(&vec![(c_row, c_col + 1, Dir::Right)]),
        // Vertical splitter
        ('|', Dir::Up) => guard(&vec![(c_row - 1, c_col, Dir::Up)]),
        ('|', Dir::Down) => guard(&vec![(c_row + 1, c_col, Dir::Down)]),
        ('|', Dir::Left) => guard(&vec![(c_row - 1, c_col, Dir::Up), (c_row + 1, c_col, Dir::Down)]),
        ('|', Dir::Right) => guard(&vec![(c_row - 1, c_col, Dir::Up), (c_row + 1, c_col, Dir::Down)]),
        // Horizontal splitter
        ('-', Dir::Up) => guard(&vec![(c_row, c_col - 1, Dir::Left), (c_row, c_col + 1, Dir::Right)]),
        ('-', Dir::Down) => guard(&vec![(c_row, c_col - 1, Dir::Left), (c_row, c_col + 1, Dir::Right)]),
        ('-', Dir::Left) => guard(&vec![(c_row, c_col - 1, Dir::Left)]),
        ('-', Dir::Right) => guard(&vec![(c_row, c_col + 1, Dir::Right)]),
        // Right-leaning mirror
        ('\\', Dir::Up) => guard(&vec![(c_row, c_col - 1, Dir::Left)]),
        ('\\', Dir::Down) => guard(&vec![(c_row, c_col + 1, Dir::Right)]),
        ('\\', Dir::Left) => guard(&vec![(c_row - 1, c_col, Dir::Up)]),
        ('\\', Dir::Right) => guard(&vec![(c_row + 1, c_col, Dir::Down)]),
        // Left-leaning mirror
        ('/', Dir::Up) => guard(&vec![(c_row, c_col + 1, Dir::Right)]),
        ('/', Dir::Down) => guard(&vec![(c_row, c_col - 1, Dir::Left)]),
        ('/', Dir::Left) => guard(&vec![(c_row + 1, c_col, Dir::Down)]),
        ('/', Dir::Right) => guard(&vec![(c_row - 1, c_col, Dir::Up)]),
        _ => panic!("Invalid map entry or direction {}, {:?}", c_entry, c_dir),
    };

    ret
}

fn propagate_light(in_contraption: &[Vec<char>], start_row: usize, start_col: usize, start_dir: Dir) -> usize {
    let n_rows = in_contraption.len();
    let n_cols = in_contraption[0].len();

    let mut has_down_beam: Vec<Vec<bool>> = Vec::new();
    let mut has_up_beam: Vec<Vec<bool>> = Vec::new();
    let mut has_left_beam: Vec<Vec<bool>> = Vec::new();
    let mut has_right_beam: Vec<Vec<bool>> = Vec::new();
    let mut drawing: Vec<Vec<char>> = Vec::new();
    for _ in 0..in_contraption.len() {
        let false_row = vec![false; n_cols];
        has_down_beam.push(false_row.clone());
        has_up_beam.push(false_row.clone());
        has_left_beam.push(false_row.clone());
        has_right_beam.push(false_row.clone());
        drawing.push(vec!['.'; n_cols]);
    }

    let mut queue: Vec<(usize, usize, Dir)> = Vec::new();
    queue.push((start_row, start_col, start_dir)); // The problem statement tells us this is the start.
    has_right_beam[start_row][start_col] = true;

    while let Some(cmd) = queue.pop() {
        let next = handle_cmd(in_contraption, &cmd);
        for (r, c, dir) in &next {
            match dir {
                Dir::Up => {
                    if !has_up_beam[*r][*c] {
                        has_up_beam[*r][*c] = true;
                        queue.push((*r, *c, *dir));
                    }
                }
                Dir::Down => {
                    if !has_down_beam[*r][*c] {
                        has_down_beam[*r][*c] = true;
                        queue.push((*r, *c, *dir));
                    }
                }
                Dir::Left => {
                    if !has_left_beam[*r][*c] {
                        has_left_beam[*r][*c] = true;
                        queue.push((*r, *c, *dir));
                    }
                }
                Dir::Right => {
                    if !has_right_beam[*r][*c] {
                        has_right_beam[*r][*c] = true;
                        queue.push((*r, *c, *dir));
                    }
                }
            }
        }
    }

    // Final stage for Part One - count all cells which contain at least one beam
    let mut energy = 0;
    for row in 0..n_rows {
        for col in 0..n_cols {
            if has_down_beam[row][col] || has_up_beam[row][col] || has_left_beam[row][col] || has_right_beam[row][col] {
                energy += 1;
                drawing[row][col] = '#';
            }
        }
    }
    // println!("Result: \n{}", aoc23::render(&drawing));
    energy
}

/// Solves Part Two by enumerating over all potential ray starting points and returning the max observed energy.
///
/// This runs in a few ms on an M1 Pro in Release mode, so it seems we don't need to do anything fancier to speed up the
/// computations.
fn get_best_energy(contraption: &[Vec<char>]) -> usize {
    let mut start_candidates: Vec<(usize, usize, Dir)> = Vec::new();
    let n_rows = contraption.len();
    let n_cols = contraption[0].len();

    for idx in 0..n_cols {
        start_candidates.push((0, idx, Dir::Down));
        start_candidates.push((n_rows - 1, idx, Dir::Up));
    }
    for idx in 0..n_rows {
        start_candidates.push((idx, 0, Dir::Right));
        start_candidates.push((n_cols - 1, idx, Dir::Left));
    }

    start_candidates
        .iter()
        .map(|(start_row, start_col, start_dir)| propagate_light(contraption, *start_row, *start_col, *start_dir))
        .max()
        .expect("A nonzero number of start configurations was expected.")
}

fn day_16_lava(input_fpath: &PathBuf) -> (usize, usize) {
    let contraption = aoc23::read_to_char_grid(input_fpath);
    // println!("{}", aoc23::render(&contraption));

    let default_energy = propagate_light(&contraption, 0, 0, Dir::Right);
    let part_one_answer: usize = default_energy;

    let best_energy = get_best_energy(&contraption);
    let part_two_answer: usize = best_energy;

    (part_one_answer, part_two_answer)
}

fn main() {
    println!("{:?}", day_16_lava(&PathBuf::from("input/16-demo.txt")));
    println!("{:?}", day_16_lava(&PathBuf::from("input/16.txt")));
}
