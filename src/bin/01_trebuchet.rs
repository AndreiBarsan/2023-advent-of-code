use std::path::PathBuf;

fn extract_code_digit_only(line: &str) -> i64 {
    let mut first_digit: i32 = 0;
    let mut last_digit: i32 = 0;
    for ch in line.chars() {
        if ch.is_numeric() {
            let val = (ch as i32) - ('0' as i32);
            if first_digit == 0 {
                first_digit = val;
            }
            last_digit = val;
        }
    }
    let code = first_digit * 10 + last_digit;
    code as i64
}

/// Returns the index of the first digit after start_idx, or the length of the string.
fn parse_until_digit_or_end(line: &str, start_idx: usize) -> usize {
    let maybe_next_digit = line[start_idx..].find(|ch: char| ch.is_digit(10));
    maybe_next_digit
        .map(|end_idx| start_idx + end_idx)
        .unwrap_or(line.len())
}

fn map_to_digit(line_chunk: &str) -> Option<(i32, usize)> {
    if line_chunk.starts_with("one") {
        Some((1, 3))
    } else if line_chunk.starts_with("two") {
        Some((2, 3))
    } else if line_chunk.starts_with("three") {
        Some((3, 5))
    } else if line_chunk.starts_with("four") {
        Some((4, 4))
    } else if line_chunk.starts_with("five") {
        Some((5, 4))
    } else if line_chunk.starts_with("six") {
        Some((6, 3))
    } else if line_chunk.starts_with("seven") {
        Some((7, 5))
    } else if line_chunk.starts_with("eight") {
        Some((8, 5))
    } else if line_chunk.starts_with("nine") {
        Some((9, 4))
    } else {
        None
    }
}

fn extract_code_smart(line: &str) -> i64 {
    let mut first_digit: i32 = 0;
    let mut last_digit: i32 = 0;
    let chrs: Vec<char> = line.chars().collect();
    let mut idx = 0;
    while idx < chrs.len() {
        let ch = chrs[idx];
        let val = if ch.is_numeric() && '0' != ch {
            Some((ch as i32) - ('0' as i32))
        } else {
            let old_idx = idx;
            let end_idx = parse_until_digit_or_end(line, old_idx);

            if end_idx > old_idx {
                let ret = map_to_digit(&line[old_idx..end_idx]);
                if let Some(ret) = ret {
                    // Note we cannot advance the iterator greedily since it doesn't catch the case of "merged" words,
                    // like "twone". In this case if we try to be clever we will miss the "one", but we should pick it
                    // as the last digit between two and one.
                    Some(ret.0)
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some(digit) = val {
            // println!("{}", digit);
            if first_digit == 0 {
                first_digit = digit;
            }
            last_digit = digit;
        }

        idx += 1;
    }

    assert!(
        first_digit != 0 && last_digit != 0,
        "Invalid line found: {}",
        line
    );
    let code = first_digit * 10 + last_digit;
    code as i64
}

fn day_01_trebuchet(fpath: &PathBuf) -> (i64, i64) {
    let lines =
        std::fs::read_to_string(fpath).expect(format!("Read input from {:?}", fpath).as_str());

    let code_it = lines.split_terminator("\n").map(extract_code_digit_only);
    let part_one_code: i64 = code_it.sum();

    let part_two_code_it = lines.split_terminator("\n").map(extract_code_smart);
    let part_two_code: i64 = part_two_code_it.sum();

    (part_one_code, part_two_code)
}

fn main() {
    println!(
        "{:?}",
        day_01_trebuchet(&PathBuf::from("input/01-demo-01.txt"))
    );
    println!(
        "{:?}",
        day_01_trebuchet(&PathBuf::from("input/01-demo-02.txt"))
    );
    println!(
        "{:?}",
        day_01_trebuchet(&PathBuf::from("input/01-demo-03.txt"))
    );
    println!("{:?}", day_01_trebuchet(&PathBuf::from("input/01.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_full_problem() {
        let (part_one, part_two) = day_01_trebuchet(&PathBuf::from("input/01.txt"));
        assert_eq!(part_one, 54968);
        assert_eq!(part_two, 54094);
    }
}
