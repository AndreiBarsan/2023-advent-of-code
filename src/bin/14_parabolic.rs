/// AoC 2023 Day 14 - Parabolic Reflector Dish

use std::path::PathBuf;
use std::fmt;

#[derive(Clone)]
struct Platform {
    rocks: Vec<Vec<char>>
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rock_grid = aoc23::render(&self.rocks);
        write!(f, "Platform:\n{}", rock_grid)
    }
}

impl Platform {
    fn n_rows(&self) -> usize {
        self.rocks.len()
    }

    fn n_cols(&self) -> usize {
        self.rocks[0].len()
    }

    fn load_score(&self) -> usize {
        let n_rows = self.n_rows();
        self.rocks.iter().enumerate().map(|(row_idx, row)| {
            let factor = n_rows - row_idx;
            let n_rocks = row.iter().filter(|ch| **ch == 'O').count();
            factor * n_rocks
        }).sum()
    }

    fn roll_up(&mut self) -> () {
        for _ in 0..self.n_rows().max(self.n_cols()) {
            // north roll sim
            for row in 0..self.n_rows() {
                for col in 0..self.n_cols() {
                    let cur = self.rocks[row][col];
                    let free_above = row > 0 && self.rocks[row - 1][col] == '.';
                    if cur == 'O' && free_above {
                        self.rocks[row - 1][col] = 'O';
                        self.rocks[row][col] = '.';
                    }
                }
            }
        }
    }

    fn roll_down(&mut self) -> () {
        for _ in 0..self.n_rows().max(self.n_cols()) {
            // south roll sim
            for row in (0..self.n_rows()).rev() {
                for col in 0..self.n_cols() {
                    let cur = self.rocks[row][col];
                    let free_below = row < self.n_rows() - 1 && self.rocks[row + 1][col] == '.';
                    if cur == 'O' && free_below {
                        self.rocks[row + 1][col] = 'O';
                        self.rocks[row][col] = '.';
                    }
                }
            }
        }
    }

    fn roll_left(&mut self) -> () {
        for _ in 0..self.n_rows().max(self.n_cols()) {
            // west roll sim
            for col in 0..self.n_cols() {
                for row in 0..self.n_rows() {
                    let cur = self.rocks[row][col];
                    let free_left = col > 0 && self.rocks[row][col - 1] == '.';
                    if cur == 'O' && free_left {
                        self.rocks[row][col - 1] = 'O';
                        self.rocks[row][col] = '.';
                    }
                }
            }
        }
    }

    fn roll_right(&mut self) -> () {
        for _ in 0..self.n_rows().max(self.n_cols()) {
            // east roll sim
            for col in (0..self.n_cols()).rev() {
                for row in 0..self.n_rows() {
                    let cur = self.rocks[row][col];
                    let free_right = col < self.n_cols() - 1 && self.rocks[row][col + 1] == '.';
                    if cur == 'O' && free_right {
                        self.rocks[row][col + 1] = 'O';
                        self.rocks[row][col] = '.';
                    }
                }
            }
        }
    }

    fn cycle(&mut self) -> () {
        // North -> West -> South -> East
        // println!("\nBefore: {}", self);
        self.roll_up();
        // println!("\nAfter up: {}", self);
        self.roll_left();
        // println!("\nAfter left: {}", self);
        self.roll_down();
        // println!("\nAfter down: {}", self);
        self.roll_right();
        // println!("\nAfter right: {}", self);
        // println!("***");
    }


}



fn day_14_parabolic_reflector_dish(input_fpath: &PathBuf) -> (usize, usize) {
    let platform = Platform { rocks: aoc23::read_to_char_grid(input_fpath) };

    let mut north_tilt_platform = platform.clone();
    north_tilt_platform.roll_up();

    let part_one_answer: usize = north_tilt_platform.load_score();

    // Iterate a fraction of the total steps to figure out how often patterns repeat themselves, so we don't have to
    // simulate all 1bn steps, which would take about a day on an M1 Pro with somewhat optimized Rust as of 2023.
    let mut it_plat = platform.clone();
    let sim_steps = 800;
    let full_sim_steps = 1_000_000_000;
    let mut readings: Vec<usize> = Vec::new();
    // Mostly heuristic solution based on automatically identifying the periodicity of the system. There's probably an
    // mathematical way to establish this but I couldn't figure it out.
    for _ in 0..sim_steps {
        it_plat.cycle();
        readings.push(it_plat.load_score());
    }

    // Find the loop length
    let start = 300; // Start late enough to "catch" the steady state.
    let mut sm = start;
    let mut bg = start + 1;
    while bg < readings.len() {
        if readings[sm] == readings[bg] && readings[sm - 1] == readings[bg - 1] {
            break;
        }
        sm += 1;
        bg += 2;
    }
    let period = bg - sm;
    if readings[sm] == readings[bg] && readings[sm - 1] == readings[bg - 1] {
        println!("Found loop of size {period}");
    }
    else {
        panic!("Could not identify system steady state. Try simulating more seed steps.")
    }

    let true_offset = (full_sim_steps - start) % period + start - 1;
    let part_two_answer: usize = readings[true_offset];

    (part_one_answer, part_two_answer)
}

fn main() {
    println!(
        "{:?}",
        day_14_parabolic_reflector_dish(&PathBuf::from("input/14-demo.txt"))
    );
    // 100272 is too high
    println!(
        "{:?}",
        day_14_parabolic_reflector_dish(&PathBuf::from("input/14.txt"))
    );
}