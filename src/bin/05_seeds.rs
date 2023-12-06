use std::path::PathBuf;

#[derive(Debug)]
struct Rule {
    dst_start: usize,
    src_start: usize,
    length: usize,
}

impl Rule {
    fn applies(&self, input: usize) -> bool {
        input >= self.src_start && input < self.src_start + self.length
    }

    fn apply(&self, input: usize) -> usize {
        let offset = input - self.src_start;
        self.dst_start + offset
    }
}

#[derive(Debug)]
struct Mapping {
    rules: Vec<Rule>,
}

impl Mapping {
    fn apply(&self, input: usize) -> usize {
        // We can speed it up by pre-sorting the rules.
        // Assumes only one rule can apply to an input.
        for rule in &self.rules {
            if rule.applies(input) {
                return rule.apply(input);
            }
        }

        input
    }
}

fn parse_into_mapping(lines: &Vec<&str>, start_idx: usize) -> (Mapping, usize) {
    let mut cur_idx = start_idx;
    let mut rules = vec![];
    while cur_idx < lines.len() {
        if lines[cur_idx].trim().len() < 1 {
            break;
        }
        let numbers: Vec<usize> = lines[cur_idx]
            .split(' ')
            .map(|nr| usize::from_str_radix(nr, 10).expect(""))
            .collect();
        rules.push(Rule {
            dst_start: numbers[0],
            src_start: numbers[1],
            length: numbers[2],
        });
        cur_idx += 1;
    }

    (Mapping { rules }, cur_idx)
}

fn day_05_seed(input_fpath: &PathBuf) -> (usize, usize) {
    let in_txt = std::fs::read_to_string(input_fpath)
        .expect(format!("Read input from {:?}", input_fpath).as_str());
    let lines: Vec<&str> = in_txt.split("\n").collect();

    let seed_line = lines[0].strip_prefix("seeds: ").expect("");
    let seed_ids: Vec<usize> = seed_line
        .split(' ')
        .map(|nr| usize::from_str_radix(nr, 10).expect(""))
        .collect();
    println!("{:?}", seed_ids);

    let (seed_to_soil, next_idx) = parse_into_mapping(&lines, 3);
    let (soil_to_fer, next_idx) = parse_into_mapping(&lines, next_idx + 2);
    let (fer_to_wat, next_idx) = parse_into_mapping(&lines, next_idx + 2);
    let (wat_to_lig, next_idx) = parse_into_mapping(&lines, next_idx + 2);
    let (lig_to_temp, next_idx) = parse_into_mapping(&lines, next_idx + 2);
    let (temp_to_hum, next_idx) = parse_into_mapping(&lines, next_idx + 2);
    let (hum_to_loc, _) = parse_into_mapping(&lines, next_idx + 2);

    // Part one goal: find the lowest location number that corresponds to any of the initial seeds.
    let mut min_outcome = usize::MAX;
    for seed in seed_ids.clone() {
        let outcome = hum_to_loc.apply(temp_to_hum.apply(lig_to_temp.apply(
            wat_to_lig.apply(fer_to_wat.apply(soil_to_fer.apply(seed_to_soil.apply(seed)))),
        )));
        if outcome < min_outcome {
            min_outcome = outcome;
        }
    }

    // Part 2, slow AF version - we just generate an effective list of seeds and brute-force it through our mappings.
    // Takes about 1 min on an M1 mac with optimized Rust. Of course, in Python this would likely take >15 min, forcing
    // you to do it right, namely implement the "apply" of a range to function on ranges!
    let mut fancy_seed_ids: Vec<usize> = vec![];
    let mut idx = 0;
    while idx < seed_ids.clone().len() {
        let start = seed_ids[idx];
        for j in 0..seed_ids[idx + 1] {
            fancy_seed_ids.push(start + j);
        }
        idx += 2;
    }

    let mut min_outcome2 = usize::MAX;
    println!("Starting big crunch over {:?} ids", fancy_seed_ids.len());
    for seed in fancy_seed_ids {
        let outcome = hum_to_loc.apply(temp_to_hum.apply(lig_to_temp.apply(
            wat_to_lig.apply(fer_to_wat.apply(soil_to_fer.apply(seed_to_soil.apply(seed)))),
        )));
        if outcome < min_outcome2 {
            min_outcome2 = outcome;
        }
    }

    (min_outcome, min_outcome2)
}

fn main() {
    println!("{:?}", day_05_seed(&PathBuf::from("input/05-demo.txt")));
    println!("{:?}", day_05_seed(&PathBuf::from("input/05.txt")));
}
