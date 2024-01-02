use std::path::PathBuf;

/// Returns whether there is a mirror between 'col' and 'col + 1' and whether a smudge was consumed.
fn reflects_along_col(map: &[Vec<char>], col: usize, original_smudge_available: bool) -> (bool, bool) {
    let n_cols = map[0].len() as i64;
    let n_rows = map.len() as i64;
    let mut smudge_available = original_smudge_available;

    for c_off in 0i64..n_cols {
        let left_idx = (col as i64) - c_off;
        let right_idx = (col as i64) + 1 + c_off;

        if left_idx < 0 {
            continue;
        }
        if right_idx >= n_cols {
            continue;
        }

        let mut disagreement = 0;
        for j in 0..n_rows {
            if map[j as usize][left_idx as usize] != map[j as usize][right_idx as usize] {
                disagreement += 1;
                if disagreement > 2 {
                    // Micro-optimization: Eagerly give up if we find a col that's already very different.
                    return (false, original_smudge_available);
                }
            }
        }
        // If the columns differ by exactly one, and the smudge is available, greedily try to use it.
        if disagreement == 1 && smudge_available {
            smudge_available = false;
            continue;
        }
        if disagreement > 0 {
            return (false, original_smudge_available);
        }
    }

    if original_smudge_available {
        if smudge_available {
            // Mistake: We didn't actually use the smudge, which is a requirement.
            (false, smudge_available)
        } else {
            (true, smudge_available)
        }
    } else {
        // If we found a reflection we are done
        (true, smudge_available)
    }
}

fn reflects_along_row(map: &[Vec<char>], row: usize, original_smudge_available: bool) -> (bool, bool) {
    let n_cols = map[0].len() as i64;
    let n_rows = map.len() as i64;
    let mut smudge_available = original_smudge_available;
    for r_off in 0i64..n_rows {
        let up_idx = (row as i64) - r_off;
        let down_idx = (row as i64) + 1 + r_off;

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
        } else {
            (true, smudge_available)
        }
    } else {
        (true, smudge_available)
    }
}

/// Finds the column index of a vertical line.
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

/// Finds the row index of a vertical line.
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
    let in_txt = std::fs::read_to_string(input_fpath).unwrap_or_else(|_| panic!("Read input from {:?}", input_fpath));
    let lines: Vec<String> = in_txt.split_terminator('\n').map(|s| s.to_string()).collect();
    let maps: Vec<Vec<Vec<char>>> = aoc23::chunk_lines_by_blank(&lines)
        .iter()
        .map(|m| {
            m.iter()
                .map(|row| row.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .collect();

    // Solve both parts at the same time: They are the same, except that part two greedily tries to find one smudge,
    // i.e., one line which differs by exactly one pixels from its reflection.
    let mut score_part_one = 0;
    let mut score_part_two = 0;
    for map in maps {
        let ver_line = find_ver_line(&map, false);
        let hor_line = find_hor_line(&map, false);
        score_part_one += 100 * hor_line + ver_line;

        let ver_line_smud = find_ver_line(&map, true);
        let hor_line_smud = find_hor_line(&map, true);
        score_part_two += 100 * hor_line_smud + ver_line_smud;
    }

    let part_one_answer: usize = score_part_one as usize;
    let part_two_answer: usize = score_part_two as usize;

    (part_one_answer, part_two_answer)
}

fn main() {
    println!("{:?}", day_13_point_of_incidence(&PathBuf::from("input/13-demo.txt")));
    println!("{:?}", day_13_point_of_incidence(&PathBuf::from("input/13.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_problem_13_point_of_incidence() {
        assert_eq!(
            (31265, 39359),
            day_13_point_of_incidence(&PathBuf::from("input/13.txt"))
        );
    }
}
