use std::path::PathBuf;
// use std::collections::hash_map::DefaultHasher;


fn reflects_along_col(map: &[Vec<char>], col: usize, original_smudge_available: bool) -> (bool, bool) {
    let n_cols = map[0].len() as i64;
    let n_rows = map.len() as i64;
    let mut smudge_available = original_smudge_available;
    // println!("check reflect along column: {col}");
    for c_off in 0i64..n_cols {
        let left_idx = (col as i64) - c_off;
        let right_idx = (col as i64) + 1 + c_off;

        // println!("{} vs. {}", left_idx, right_idx);

        if left_idx < 0 {
            continue;
        }
        if right_idx >= n_cols {
            continue;
        }

        let mut disagreement = 0;
        for j in 0..n_rows {
            if map[j as usize][left_idx as usize] != map[j as usize][right_idx as usize] {
                // println!("Fail");
                disagreement += 1;
                // return false;
            }
        }
        if disagreement == 1 {
            // println!("Smudge candidate - enables new col fold");
            if smudge_available {
                // Greedily consume a smudge
                smudge_available = false;
                continue;
            }
        }
        if disagreement > 0 {
            return (false, original_smudge_available);
        }
    }

    if original_smudge_available {
        if smudge_available {
            (false, smudge_available)
        }
        else {
            (true, smudge_available)
        }
    }
    else {
    // println!("Reflects");
    // If we found a reflection we are done
        (true, smudge_available)
    }
}

fn reflects_along_row(map: &[Vec<char>], row: usize, original_smudge_available: bool) -> (bool, bool) {
    let n_cols = map[0].len() as i64;
    let n_rows = map.len() as i64;
    // println!("check reflect along row: {row}");
    let mut smudge_available = original_smudge_available;
    for r_off in 0i64..n_rows {
        let up_idx = (row as i64) - r_off;
        let down_idx = (row as i64) + 1 + r_off;

        // println!("{} vs. {}", up_idx, down_idx);

        if up_idx < 0 {
            continue;
        }
        if down_idx >= n_rows {
            continue;
        }

        let mut disagreement = 0;
        for j in 0..n_cols {
            if map[up_idx as usize][j as usize] != map[down_idx as usize][j as usize] {
                // println!("Fail");
                disagreement += 1;
                // return false;
            }
        }
        if disagreement == 1 {
            // println!("Smudge candidate - enables new row fold");
            if smudge_available {
                smudge_available = false;
                continue;
            }
        }
        if disagreement > 0 {
            return (false, original_smudge_available);
        }
    }

    if original_smudge_available {
        if smudge_available {
            (false, smudge_available)
        }
        else {
            (true, smudge_available)
        }
    }
    else {
        (true, smudge_available)
    }
}


fn find_ver_line(map: &[Vec<char>], orig_smudge: bool) -> i64 {
    let n_cols = map[0].len();
    let mut smudge = orig_smudge;

    for c in 0..n_cols - 1 {
        let (reflects, new_smudge) = reflects_along_col(map, c, smudge);
        smudge = new_smudge;
        if reflects {
            return c as i64 + 1;
        }
    }

    0
}

fn find_hor_line(map: &[Vec<char>], orig_smudge: bool) -> i64 {
    let n_rows = map.len();
    let mut smudge = orig_smudge;

    for r in 0..n_rows - 1 {
        let (reflects, new_smudge) = reflects_along_row(map, r, smudge);
        smudge = new_smudge;
        if reflects {
            return r as i64 + 1;
        }
    }

    0
}

fn day_13_point_of_incidence(input_fpath: &PathBuf) -> (usize, usize) {
    let in_txt = std::fs::read_to_string(input_fpath)
        .unwrap_or_else(|_| panic!("Read input from {:?}", input_fpath));
    let lines: Vec<String> = in_txt.split_terminator('\n').map(|s| s.to_string()).collect();
    let maps: Vec<Vec<Vec<char>>> = aoc23::chunk_lines_by_blank(&lines).iter()
        .map(|m| m.iter().map(|row| row.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>())
        .collect();

    // let mut s = DefaultHasher::new();
    let mut score = 0;
    let mut score_part_two = 0;
    for map in maps {
        println!("\n\nNew map.\n\n");
        // let n_rows = map.len();
        // let n_cols = map[0].len();

        let ver_line = find_ver_line(&map, false);
        let hor_line = find_hor_line(&map, false);

        // println!("{ver_line} {hor_line}");

        score += 100 * hor_line + ver_line;

        let ver_line_smud= find_ver_line(&map, true);
        let hor_line_smud = find_hor_line(&map, true);

        score_part_two += 100 * hor_line_smud + ver_line_smud;

        // let row_hashes = (0..n_rows).map(|row_idx| {
        //     map[row_idx].hash(&mut s);
        // })
    }

    let part_one_answer: usize = score as usize;
    let part_two_answer: usize = score_part_two as usize;

    (part_one_answer, part_two_answer)
}

fn main() {
    println!(
        "{:?}",
        day_13_point_of_incidence(&PathBuf::from("input/13-demo.txt"))
    );
    println!(
        "{:?}",
        day_13_point_of_incidence(&PathBuf::from("input/13.txt"))
    );
}

