use std::collections::HashSet;
use std::path::PathBuf;

struct Scratchcard {
    winning: HashSet<u64>,
    owned: HashSet<u64>,
}

impl Scratchcard {
    fn get_winning_numbers(&self) -> HashSet<u64> {
        self.winning.intersection(&self.owned).cloned().collect()
    }
}

/// Parses a scratchcard specification string of the form "Card K: A B C | D E" where A.. are non-negative integers.
fn parse_scratchcard(line: &str) -> Scratchcard {
    let useful_chunk = line.split(":").collect::<Vec<&str>>()[1];
    let number_chunks: Vec<&str> = useful_chunk.split("|").collect();
    let winning: HashSet<u64> = number_chunks[0]
        .split_terminator(" ")
        .map(|nr| nr.trim())
        .filter(|nr| nr.len() > 0)
        .map(|nr| {
            u64::from_str_radix(nr, 10).expect(format!("Parse winning number: {}", nr).as_str())
        })
        .collect();
    let owned: HashSet<u64> = number_chunks[1]
        .split_terminator(" ")
        .map(|nr| nr.trim())
        .filter(|nr| nr.len() > 0)
        .map(|nr| {
            u64::from_str_radix(nr, 10).expect(format!("Parse owned number: {}", nr).as_str())
        })
        .collect();
    Scratchcard { winning, owned }
}

fn day_04_scratchcards(input_fpath: &PathBuf) -> (u64, u64) {
    let lines = std::fs::read_to_string(input_fpath)
        .expect(format!("Read input from {:?}", input_fpath).as_str());
    let scratchcards: Vec<Scratchcard> = lines
        .split_terminator("\n")
        .map(parse_scratchcard)
        .collect();
    let scratch_scores: Vec<usize> = scratchcards
        .iter()
        .map(|scratchcard| scratchcard.get_winning_numbers().len())
        .collect();
    let part_one_score = scratch_scores
        .iter()
        .map(|winners| 2u64.pow(*winners as u32 - 1))
        .sum();

    let mut multipliers = vec![1; scratchcards.len()];
    for (idx, scratch_scores) in scratch_scores.iter().enumerate() {
        let cur_mult = multipliers[idx];
        let end = idx + (*scratch_scores as usize);
        for j in idx + 1..=end {
            multipliers[j] += cur_mult;
        }
    }
    let part_two_score = multipliers.iter().sum();
    (part_one_score, part_two_score)
}

fn main() {
    println!(
        "{:?}",
        day_04_scratchcards(&PathBuf::from("input/04-demo.txt"))
    );
    println!("{:?}", day_04_scratchcards(&PathBuf::from("input/04.txt")));
}
