use std::path::PathBuf;

use itertools::Itertools;


struct SpringRecord {
    statuses: Vec<char>,
    cluster_sizes: Vec<usize>
}

impl SpringRecord {
    fn multiply(&self, n: usize) -> Self {
        SpringRecord {
            statuses: repeat_with_sep(&self.statuses, n),
            cluster_sizes: self.cluster_sizes.repeat(n)
        }
    }
}

fn repeat_with_sep(st: &Vec<char>, n: usize) -> Vec<char> {
    let mut out = Vec::new();

    for idx in 0..n-1 {
        out = [out, st.clone()].concat();
        out.push('?');
    }

    out = [out, st.clone()].concat();

    out
}

fn parse_spring_record(line: &str) -> SpringRecord {
    let parts: Vec<&str> = line.split(' ').collect();
    let statuses: Vec<char> = parts[0].chars().collect();
    let cluster_sizes: Vec<usize> = parts[1].split_terminator(',').map(|nr| nr.parse::<usize>().expect("Parse cluster size")).collect();

    SpringRecord { statuses, cluster_sizes }
}

fn is_valid(statuses: &Vec<char>, cluster_sizes: &Vec<usize>) -> bool {
    let damaged_chunk_lens: Vec<usize> = statuses.split(|c| *c == '.').map(|cluster| cluster.len()).filter(|l| *l != 0).collect();
    &damaged_chunk_lens == cluster_sizes
}


fn is_variant_valid(sr: &SpringRecord, variant: &Vec<bool>) -> bool {
    let mut new_status = sr.statuses.clone();
    let mut wild_idx = 0;

    for idx in 0..new_status.len() {
        if new_status[idx] == '?' {
            new_status[idx] = if variant[wild_idx] { '.' } else { '#' };
            wild_idx += 1;
        }
    }
    assert!(wild_idx == variant.len());

    let iv = is_valid(&new_status, &sr.cluster_sizes);
    // println!("{:?}", variant);
    // if iv {
    //     println!("valid   - {:?}", &new_status.iter().join(""));
    // }
    // else {
    //     println!("INVALID - {:?}", &new_status.iter().join(""));
    // }
    iv
}


// This is a trash approach, lmao.
fn gen(v: &Vec<bool>, idx: usize) -> Vec<Vec<bool>> {
    if idx >= v.len() {
        [v.clone()].to_vec()
    }
    else {
        let mut newv_lhs = v.clone();
        newv_lhs[idx] = false;
        let lhs = gen(&newv_lhs, idx + 1);
        let mut newv_rhs = v.clone();
        newv_rhs[idx] = true;
        let rhs = gen(&newv_rhs, idx + 1);

        [lhs, rhs].concat()
    }
}

// fn gen_fast(v: &Vec<bool>, idx: usize) -> Vec<Vec<bool>> {
//     let tf: Vec<bool> = Vec::new();
//     tf.iter().cartesian_product(other)
// }


fn count_perm(sr: &SpringRecord) -> usize {
    let n_wildcard = sr.statuses.iter().filter(|c| **c == '?').count();
    println!("{n_wildcard} wildcards");
    // tf.iter().com(n_wildcard).map(|variant| is_variant_valid(sr, &variant)).filter(|r| *r).count()
    let zeroes = vec![false; n_wildcard];
    println!("Expected: {}", 2u128.pow(n_wildcard as u32));
    // let cand: Vec<Vec<bool>> = gen(&zeroes, 0);
    // println!("{}", cand.len());
    // let mut cand: Vec<Vec<bool>> = Vec::new();

    let max = 2u64.pow(n_wildcard as u32);
    let cand = (0..max).map(|v| (0..n_wildcard).map (move |n| ((v >> n) & 1) == 1));

    // smart candidate generation
    // let indexed_chars: Vec<(usize, char)> = sr.statuses.into_iter().enumerate().collect();
    // let mystery_chunks: Vec<&[(usize, char)]> = indexed_chars.split(|(idx, c)| *c != '?').filter(|cluster| cluster.len() != 0).collect();

    cand.map(|variant| is_variant_valid(sr, &variant.collect::<Vec<bool>>())).filter(|r| *r).count()
}

/// Searching for all possible permutations for the unknown elements.
///
/// I will first try a brute force baseline and see if it works.
fn day_12_tbd(input_fpath: &PathBuf) -> (usize, usize) {
    let in_txt = std::fs::read_to_string(input_fpath)
        .unwrap_or_else(|_| panic!("Read input from {:?}", input_fpath));
    let records: Vec<SpringRecord> = in_txt.split_terminator('\n').map(parse_spring_record).collect();

    // for rec in records {
    //     println!("{}", aoc23::render(&[rec.statuses]));
    // }

    let records_multiplied: Vec<SpringRecord> = records.iter().map(|r| r.multiply(5)).collect();

    let rperm: Vec<usize> = records.iter().map(count_perm).collect();
    println!("{:?}", rperm);

    let part_one_answer: usize = rperm.iter().sum();
    let part_two_answer: usize = records_multiplied.iter().enumerate().map(|(idx, sr)| {
        if idx > 0 && idx % 5 == 0 {
            println!("Part two at record {idx}");
        }
        count_perm(sr)
    }).sum();

    (part_one_answer, part_two_answer)
}

fn main() {
    println!(
        "{:?}",
        day_12_tbd(&PathBuf::from("input/12-demo-01.txt"))
    );
    // println!(
    //     "{:?}",
    //     day_12_tbd(&PathBuf::from("input/12.txt"))
    // );
}

