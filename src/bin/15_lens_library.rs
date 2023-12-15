use std::{path::PathBuf};

enum Op {
    Add {label: String, focal_length: i32},
    Remove { label: String },
}

impl Op {
    fn get_hash(&self) -> usize {
        match self {
            Op::Add { label, focal_length } => reindeer_hash(label),
            Op::Remove { label } => reindeer_hash(label),
        }
    }
}

#[derive(Debug)]
struct Box {
    // TODO(andrei): Find and use an order-preserving map!
    lens_order: Vec<(String, i32)>,
}

impl Box {
    fn new() -> Self {
        Box {
            lens_order: Vec::new()
        }
    }

    fn remove_by(&mut self, key: &String) {
        if let Some(index) = self.lens_order.iter().position(|x| &x.0 == key) {
            self.lens_order.remove(index);
        }
    }

    fn update_by(&mut self, key: &String, focal_length: i32) {
        if let Some(index) = self.lens_order.iter().position(|x| &x.0 == key) {
            self.lens_order[index].1 = focal_length;
        }
        else {
            self.lens_order.push((key.clone(), focal_length));
        }
    }

    fn focusing_score(&self) -> usize {
        self.lens_order.iter().enumerate().map(|(idx, (_, focal_length))| (idx + 1) * (*focal_length as usize)).sum()
    }

}

fn parse_op(input: &str) -> Op {
    if input.ends_with('-') {
        let label = input[..input.len() - 1].to_string();
        Op::Remove { label }
    }
    else {
        let parts: Vec<&str> = input.split('=').collect();
        let label = parts[0].to_string();
        let rhs = parts[1];
        let focal_length: i32 = rhs.parse().expect("");
        Op::Add { label, focal_length }
    }
}

fn process_ops(ops: &[Op]) -> Vec<Box> {
    let mut boxes: Vec<Box> = Vec::new();
    for _ in 0..256 {
        boxes.push(Box::new());
    }

    for op in ops {
        let hash = op.get_hash();
        match op {
            Op::Add { label, focal_length } => {
                // println!("Add {focal_length} to {hash} under {label}.");
                boxes[hash].update_by(label, *focal_length);
            },
            Op::Remove { label } => {
                boxes[hash].remove_by(label);
            }
        }
    }

    for b in &boxes {
        println!("{:?}", b);
    }

    boxes
}

/// Implements the reindeer hash from the problem statement
///
/// Determine the ASCII code for the current character of the string.
/// Increase the current value by the ASCII code you just determined.
/// Set the current value to itself multiplied by 17.
/// Set the current value to the remainder of dividing itself by 256.
fn reindeer_hash(input: &str) -> usize {
    input.chars().fold(0,|cur, ch| {
        ((cur + (ch as usize)) * 17) % 256
    })
}


fn day_15_lens_library(input_fpath: &PathBuf) -> (usize, usize) {
    let in_txt = std::fs::read_to_string(input_fpath)
        .unwrap_or_else(|_| panic!("Read input from {:?}", input_fpath));
    let codes: Vec<&str> = in_txt.split_terminator(',').collect();
    let hashes: Vec<usize> = codes.clone().into_iter().map(reindeer_hash).collect();
    let part_one_answer: usize = hashes.iter().sum();

    let ops: Vec<Op> = codes.into_iter().map(parse_op).collect();
    let outcome = process_ops(&ops);
    let box_focusing_scores: Vec<usize> = outcome.iter().map(|b| b.focusing_score())
        .enumerate().map(|(box_number, fs)| (1 + box_number) * fs).collect();
    // println!("{:?}", box_focusing_scores);
    let part_two_answer: usize = box_focusing_scores.iter().sum();

    (part_one_answer, part_two_answer)
}

fn main() {
    println!(
        "{:?}",
        day_15_lens_library(&PathBuf::from("input/15-demo.txt"))
    );
    // 462209 is not good
    println!(
        "{:?}",
        day_15_lens_library(&PathBuf::from("input/15.txt"))
    );
}


