// A collection of common AoC helpers.

use itertools::Itertools;

pub fn render(chars: &[Vec<char>]) -> String {
    chars
        .iter()
        .map(|row| row.iter().collect::<String>())
        .join("\n")
}

pub fn concat_nums(nums: &[usize]) -> usize {
    let num_str = nums.iter().map(|n| n.to_string()).join("");
    num_str
        .parse::<usize>()
        .unwrap_or_else(|_| panic!("Cannot parse to usize: {}", num_str))
}

pub fn chunk_lines_by_blank(lines: &Vec<String>) -> Vec<Vec<String>> {
    // TODO(andrei): Consider solving with iterators.
    let mut out = vec![];
    let mut cur = vec![];

    for line in lines {
        if line.is_empty() {
            out.push(cur.clone());
            cur = vec![];
        } else {
            cur.push(line.clone());
        }
    }
    out.push(cur.clone());

    out
}

/// Reads a file, assumed to hold files of identical lengths, into a vector of vectors.
fn read_to_char_grid(input_fpath: &PathBuf) -> Vec<Vec<char>> {
    let in_txt = std::fs::read_to_string(input_fpath)
        .unwrap_or_else(|_| panic!("Read input from {:?}", input_fpath));
    let rows = in_txt.split_terminator('\n');
    rows.map(|s| s.chars().collect()).collect()
}
